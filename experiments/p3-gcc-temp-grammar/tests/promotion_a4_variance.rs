use std::collections::BTreeSet;

use execsurface_model::canonical::{
    CanonicalEffect, CanonicalExecutable, CanonicalPath, CanonicalSurface, NormalizationMetadata,
    OpenIntent, PathClass, PathResolution,
};
use execsurface_model::FileOperation;
use execsurface_p3_gcc_temp_grammar::{
    apply_candidate, eligible_gcc_temp_paths, GCC_TEMP_CANONICAL,
};

const GCC: &str = "/usr/bin/gcc";

fn path(value: &str, class: PathClass) -> CanonicalPath {
    CanonicalPath {
        value: value.to_owned(),
        class,
        resolution: PathResolution::Lexical,
    }
}

fn exe(value: &str) -> CanonicalExecutable {
    CanonicalExecutable {
        path: path(value, PathClass::System),
        family: value.rsplit('/').next().unwrap_or(value).to_owned(),
    }
}

fn create_intent() -> OpenIntent {
    OpenIntent {
        read: true,
        write: true,
        create: true,
        truncate: false,
        append: false,
        path_only: false,
        resolve_flags: 0,
        other_flags: 128,
    }
}

fn read_intent() -> OpenIntent {
    OpenIntent {
        read: true,
        write: false,
        create: false,
        truncate: false,
        append: false,
        path_only: false,
        resolve_flags: 0,
        other_flags: 0,
    }
}

fn file(
    actor_path: &str,
    op: FileOperation,
    target: CanonicalPath,
    intent: Option<OpenIntent>,
) -> CanonicalEffect {
    let actor = exe(actor_path);
    CanonicalEffect::FilePathAccess {
        actor: Some(actor.clone()),
        execution_chain: vec![exe("/bin/bash"), actor],
        operation: op,
        target,
        open_intent: intent,
    }
}

fn gcc_pair(name: &str) -> Vec<CanonicalEffect> {
    let target = path(name, PathClass::Temp);
    vec![
        file(
            GCC,
            FileOperation::Open,
            target.clone(),
            Some(create_intent()),
        ),
        file(GCC, FileOperation::Delete, target, None),
    ]
}

fn surface(effects: Vec<CanonicalEffect>) -> CanonicalSurface {
    CanonicalSurface {
        schema_version: 2,
        normalization: NormalizationMetadata {
            profile_version: 3,
            semantic_roots: vec!["tmp".to_owned()],
        },
        effects,
    }
}

fn eligible(surface: &CanonicalSurface) -> BTreeSet<String> {
    eligible_gcc_temp_paths(surface)
}

#[test]
fn a4_01_bounded_benign_pair_reduces_false_review() {
    let left = surface(gcc_pair("$TMP/ccABC123.s"));
    let right = surface(gcc_pair("$TMP/ccXYZ789.s"));
    assert_ne!(
        left, right,
        "raw identity churn must remain visible before projection"
    );
    let left_projected = apply_candidate(&left);
    let right_projected = apply_candidate(&right);
    assert_eq!(
        left_projected, right_projected,
        "bounded ephemeral churn should collapse in the derived view"
    );
    assert!(left_projected.effects.iter().all(|effect| match effect {
        CanonicalEffect::FilePathAccess { target, .. } => target.value == GCC_TEMP_CANONICAL,
        _ => false,
    }));
}

#[test]
fn a4_02_meaningful_non_ephemeral_drift_survives_projection() {
    let baseline = surface(gcc_pair("$TMP/ccABC123.s"));
    let mut changed_effects = gcc_pair("$TMP/ccXYZ789.s");
    changed_effects.push(file(
        GCC,
        FileOperation::Open,
        path("/etc/passwd", PathClass::System),
        Some(read_intent()),
    ));
    let changed = surface(changed_effects);
    assert_ne!(apply_candidate(&baseline), apply_candidate(&changed));
}

#[test]
fn a4_03_wrong_actor_never_projects() {
    let input = surface(vec![
        file(
            "/usr/bin/clang",
            FileOperation::Open,
            path("$TMP/ccABC123.s", PathClass::Temp),
            Some(create_intent()),
        ),
        file(
            "/usr/bin/clang",
            FileOperation::Delete,
            path("$TMP/ccABC123.s", PathClass::Temp),
            None,
        ),
    ]);
    assert!(eligible(&input).is_empty());
    assert_ne!(
        apply_candidate(&input).effects[0],
        apply_candidate(&surface(gcc_pair("$TMP/ccABC123.s"))).effects[0]
    );
}

#[test]
fn a4_04_missing_create_delete_role_never_projects() {
    let only_create = surface(vec![file(
        GCC,
        FileOperation::Open,
        path("$TMP/ccABC123.s", PathClass::Temp),
        Some(create_intent()),
    )]);
    let only_delete = surface(vec![file(
        GCC,
        FileOperation::Delete,
        path("$TMP/ccABC123.s", PathClass::Temp),
        None,
    )]);
    assert!(eligible(&only_create).is_empty());
    assert!(eligible(&only_delete).is_empty());
}

#[test]
fn a4_05_conflicting_actor_collision_fails_closed() {
    let mut effects = gcc_pair("$TMP/ccABC123.s");
    effects.push(file(
        "/usr/bin/clang",
        FileOperation::Open,
        path("$TMP/ccABC123.s", PathClass::Temp),
        Some(read_intent()),
    ));
    assert!(eligible(&surface(effects)).is_empty());
}

#[test]
fn a4_06_rename_collision_fails_closed() {
    let mut effects = gcc_pair("$TMP/ccABC123.s");
    let actor = exe(GCC);
    effects.push(CanonicalEffect::FileRename {
        actor: Some(actor.clone()),
        execution_chain: vec![exe("/bin/bash"), actor],
        from: path("$TMP/ccABC123.s", PathClass::Temp),
        to: path("$TMP/ccZZZ999.s", PathClass::Temp),
    });
    assert!(eligible(&surface(effects)).is_empty());
}

#[test]
fn a4_07_wrong_root_and_grammar_near_misses_remain_distinct() {
    let cases = [
        path("$WORKSPACE/ccABC123.s", PathClass::Workspace),
        path("$TMP/ccABC12.s", PathClass::Temp),
        path("$TMP/ccABC1234.s", PathClass::Temp),
        path("$TMP/ccABC-23.s", PathClass::Temp),
        path("$TMP/CCABC123.s", PathClass::Temp),
        path("$TMP/ccABC123.o", PathClass::Temp),
    ];
    for target in cases {
        let input = surface(vec![
            file(
                GCC,
                FileOperation::Open,
                target.clone(),
                Some(create_intent()),
            ),
            file(GCC, FileOperation::Delete, target, None),
        ]);
        assert!(eligible(&input).is_empty());
    }
}

#[test]
fn a4_08_repetition_frequency_never_authorizes_ineligible_identity() {
    let mut effects = Vec::new();
    for _ in 0..256 {
        effects.push(file(
            GCC,
            FileOperation::Open,
            path("$TMP/ccABC123.s", PathClass::Temp),
            Some(create_intent()),
        ));
    }
    let input = surface(effects);
    assert!(
        eligible(&input).is_empty(),
        "frequency cannot manufacture the missing delete role"
    );
}

#[test]
fn a4_09_repeated_wrong_actor_never_becomes_authorized() {
    let mut effects = Vec::new();
    for _ in 0..256 {
        effects.push(file(
            "/usr/bin/clang",
            FileOperation::Open,
            path("$TMP/ccABC123.s", PathClass::Temp),
            Some(create_intent()),
        ));
        effects.push(file(
            "/usr/bin/clang",
            FileOperation::Delete,
            path("$TMP/ccABC123.s", PathClass::Temp),
            None,
        ));
    }
    assert!(eligible(&surface(effects)).is_empty());
}

#[test]
fn a4_10_raw_evidence_is_retained_unchanged() {
    let raw = surface(gcc_pair("$TMP/ccABC123.s"));
    let snapshot = raw.clone();
    let projected = apply_candidate(&raw);
    assert_eq!(raw, snapshot, "projection must not mutate raw evidence");
    assert_ne!(
        raw, projected,
        "derived projection must remain a distinct artifact/view"
    );
}

#[test]
fn a4_11_projection_is_deterministic_under_effect_order_variation() {
    let forward = gcc_pair("$TMP/ccABC123.s");
    let mut reverse = forward.clone();
    reverse.reverse();
    assert_eq!(
        apply_candidate(&surface(forward)),
        apply_candidate(&surface(reverse))
    );
}

#[test]
fn a4_12_unrelated_existing_normalization_remains_distinct() {
    let input = surface(vec![file(
        GCC,
        FileOperation::Open,
        path(
            "$TMP/go-build<ephemeral>/b001/vet.cfg",
            PathClass::Temp,
        ),
        Some(create_intent()),
    )]);
    assert!(eligible(&input).is_empty());
    let projected = apply_candidate(&input);
    assert!(projected.effects.iter().any(|effect| match effect {
        CanonicalEffect::FilePathAccess { target, .. } => {
            target.value == "$TMP/go-build<ephemeral>/b001/vet.cfg"
        }
        _ => false,
    }));
}
