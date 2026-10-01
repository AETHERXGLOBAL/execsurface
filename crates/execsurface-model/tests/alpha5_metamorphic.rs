use std::collections::BTreeSet;

use execsurface_model::canonical::{CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation,
    ProofRequirement, Proposition, TemporalBinding,
};
use execsurface_model::FileOperation;

fn path(value: &str) -> CanonicalPath {
    CanonicalPath {
        value: value.to_owned(),
        class: PathClass::Workspace,
        resolution: PathResolution::Lexical,
    }
}

fn proposition(value: &str) -> Proposition {
    Proposition::FilePathnameAttemptObserved {
        actor: None,
        execution_chain: vec![],
        operation: FileOperation::Open,
        target: path(value),
        open_intent: None,
    }
}

fn guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
        identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
        temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}

fn requirement(value: &str) -> ProofRequirement {
    ProofRequirement::for_proposition(
        proposition(value),
        guarantees(),
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Lifecycle,
        ]),
    )
}

fn record(value: &str) -> ProofCarryingObservation {
    let mut record = ProofCarryingObservation::new(
        proposition(value),
        guarantees(),
        BackendSemanticProfile {
            name: "bounded-ptrace-profile".to_owned(),
            semantic_profile_version: 1,
        },
    );
    record.completeness.insert(
        CompletenessDimension::SessionScope,
        CompletenessState::Complete,
    );
    record.completeness.insert(
        CompletenessDimension::Lifecycle,
        CompletenessState::Complete,
    );
    record
}

#[test]
fn m01_subject_mutation_always_breaks_bound_admission() {
    let req = requirement("$WORKSPACE/input.txt");
    let control = record("$WORKSPACE/input.txt");
    assert!(control.satisfies(&req));

    for mutated in [
        "$WORKSPACE/input-2.txt",
        "$WORKSPACE/sub/input.txt",
        "$WORKSPACE/INPUT.txt",
        "$WORKSPACE/input.txt/child",
    ] {
        assert!(
            !record(mutated).satisfies(&req),
            "subject mutation unexpectedly preserved proof admission: {mutated}"
        );
    }
}

#[test]
fn m02_adding_ambiguity_is_monotone_fail_closed() {
    let req = requirement("$WORKSPACE/input.txt");
    let mut candidate = record("$WORKSPACE/input.txt");
    assert!(candidate.satisfies(&req));

    candidate
        .ambiguity_codes
        .insert("future_unclassified_ambiguity".to_owned());
    assert!(
        !candidate.satisfies(&req),
        "adding ambiguity must never improve or preserve PASS-eligible admission"
    );
}

#[test]
fn m03_record_and_requirement_roundtrip_preserve_binding() {
    let req = requirement("$WORKSPACE/input.txt");
    let candidate = record("$WORKSPACE/input.txt");
    assert!(candidate.satisfies(&req));

    let record_json = serde_json::to_vec(&candidate).expect("serialize record");
    let req_json = serde_json::to_vec(&req).expect("serialize requirement");
    let record_rt: ProofCarryingObservation =
        serde_json::from_slice(&record_json).expect("deserialize record");
    let req_rt: ProofRequirement = serde_json::from_slice(&req_json).expect("deserialize requirement");

    assert_eq!(record_rt, candidate);
    assert_eq!(req_rt, req);
    assert!(record_rt.satisfies(&req_rt));
    assert!(!record("$WORKSPACE/other.txt").satisfies(&req_rt));
}

#[test]
fn m04_deserialized_missing_proposition_binding_fails_closed() {
    let req = requirement("$WORKSPACE/input.txt");
    let candidate = record("$WORKSPACE/input.txt");
    let mut json = serde_json::to_value(&req).expect("serialize requirement");
    json.as_object_mut()
        .expect("requirement must be object")
        .remove("expected_proposition");
    let stripped: ProofRequirement = serde_json::from_value(json).expect("deserialize stripped requirement");

    assert!(
        !candidate.satisfies(&stripped),
        "serialization omission must not turn an unbound requirement into proof"
    );
}

#[test]
fn m05_schema_version_downgrade_cannot_reuse_v3_proof() {
    let req = requirement("$WORKSPACE/input.txt");
    let mut candidate = record("$WORKSPACE/input.txt");
    assert!(candidate.satisfies(&req));

    candidate.schema_version = 2;
    assert!(
        !candidate.satisfies(&req),
        "v2-labelled record must not be admitted as v3 proof"
    );
}

#[test]
fn m06_backend_name_and_version_cannot_launder_missing_guarantees() {
    let candidate = record("$WORKSPACE/input.txt");
    let stronger = ProofRequirement::for_proposition(
        proposition("$WORKSPACE/input.txt"),
        EvidenceGuarantees {
            identity_bases: BTreeSet::from([IdentityBasis::KernelObjectGrounded]),
            ..EvidenceGuarantees::default()
        },
        BTreeSet::from([CompletenessDimension::SessionScope]),
    );
    assert!(!candidate.satisfies(&stronger));

    for (name, version) in [
        ("trusted", 1),
        ("kernel-authoritative", u32::MAX),
        ("attested-production", 999_999),
    ] {
        let mut renamed = candidate.clone();
        renamed.backend_profile.name = name.to_owned();
        renamed.backend_profile.semantic_profile_version = version;
        assert!(
            !renamed.satisfies(&stronger),
            "backend metadata laundered missing semantic guarantee: {name}/{version}"
        );
    }
}

#[test]
fn m07_requirement_stripping_cannot_create_vacuous_admission() {
    let candidate = record("$WORKSPACE/input.txt");
    let stripped = ProofRequirement::for_proposition(
        proposition("$WORKSPACE/input.txt"),
        EvidenceGuarantees::default(),
        BTreeSet::new(),
    );
    assert!(
        !candidate.satisfies(&stripped),
        "removing all obligations must invalidate rather than broaden a proof contract"
    );
}

#[test]
fn m08_completeness_downgrade_cannot_be_rescued_by_backend_metadata() {
    let req = requirement("$WORKSPACE/input.txt");
    let mut candidate = record("$WORKSPACE/input.txt");
    candidate.completeness.insert(
        CompletenessDimension::Lifecycle,
        CompletenessState::Incomplete {
            reason_code: "forced-alpha5-metamorphic-loss".to_owned(),
        },
    );
    candidate.backend_profile.name = "max-authority-name".to_owned();
    candidate.backend_profile.semantic_profile_version = u32::MAX;

    assert!(
        !candidate.satisfies(&req),
        "backend metadata must not rescue explicitly incomplete evidence"
    );
}
