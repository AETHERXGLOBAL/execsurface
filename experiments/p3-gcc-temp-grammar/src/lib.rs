use std::collections::{BTreeMap, BTreeSet};

use execsurface_model::canonical::{
    CanonicalEffect, CanonicalExecutable, CanonicalPath, CanonicalSurface, PathClass,
};
use execsurface_model::FileOperation;

pub const CANDIDATE_NORMALIZATION_PROFILE_VERSION: u32 = 4;
pub const GCC_TEMP_CANONICAL: &str = "$TMP/cc<gcc-ephemeral>.s";
const BOUNDED_GCC_PATH: &str = "/usr/bin/gcc";

#[derive(Debug, Clone, Default)]
struct CandidateState {
    create_open: bool,
    delete: bool,
    conflict: bool,
}

/// Return the exact canonical temp identities that satisfy the bounded V2
/// producer/role contract. This is descriptive research classification only;
/// it does not authorize behavior or alter the public normalization profile.
pub fn eligible_gcc_temp_paths(surface: &CanonicalSurface) -> BTreeSet<String> {
    let mut states: BTreeMap<String, CandidateState> = BTreeMap::new();

    for effect in &surface.effects {
        match effect {
            CanonicalEffect::FilePathAccess {
                actor,
                execution_chain,
                operation,
                target,
                open_intent,
            } if matches_gcc_temp_identity(target) => {
                let state = states.entry(target.value.clone()).or_default();
                if !is_bounded_gcc_actor(actor.as_ref(), execution_chain) {
                    state.conflict = true;
                    continue;
                }

                match operation {
                    FileOperation::Open
                        if open_intent
                            .as_ref()
                            .is_some_and(|intent| intent.create && intent.write) =>
                    {
                        state.create_open = true;
                    }
                    FileOperation::Delete => state.delete = true,
                    _ => state.conflict = true,
                }
            }
            CanonicalEffect::FileRename { from, to, .. } => {
                for path in [from, to] {
                    if matches_gcc_temp_identity(path) {
                        states.entry(path.value.clone()).or_default().conflict = true;
                    }
                }
            }
            _ => {}
        }
    }

    states
        .into_iter()
        .filter_map(|(path, state)| {
            (state.create_open && state.delete && !state.conflict).then_some(path)
        })
        .collect()
}

pub fn apply_candidate(surface: &CanonicalSurface) -> CanonicalSurface {
    let eligible = eligible_gcc_temp_paths(surface);
    let mut candidate = surface.clone();
    candidate.normalization.profile_version = CANDIDATE_NORMALIZATION_PROFILE_VERSION;
    candidate.effects = candidate
        .effects
        .into_iter()
        .map(|effect| normalize_effect(effect, &eligible))
        .collect();
    candidate.effects.sort();
    candidate.effects.dedup();
    candidate
}

fn normalize_effect(effect: CanonicalEffect, eligible: &BTreeSet<String>) -> CanonicalEffect {
    match effect {
        CanonicalEffect::FilePathAccess {
            actor,
            execution_chain,
            operation,
            mut target,
            open_intent,
        } => {
            if eligible.contains(&target.value) {
                target.value = GCC_TEMP_CANONICAL.to_owned();
            }
            CanonicalEffect::FilePathAccess {
                actor,
                execution_chain,
                operation,
                target,
                open_intent,
            }
        }
        other => other,
    }
}

fn is_bounded_gcc_actor(
    actor: Option<&CanonicalExecutable>,
    execution_chain: &[CanonicalExecutable],
) -> bool {
    let Some(actor) = actor else {
        return false;
    };
    if actor.path.value != BOUNDED_GCC_PATH || actor.family != "gcc" {
        return false;
    }

    execution_chain.last().is_some_and(|last| last == actor)
}

fn matches_gcc_temp_identity(path: &CanonicalPath) -> bool {
    if path.class != PathClass::Temp {
        return false;
    }
    let Some(basename) = path.value.strip_prefix("$TMP/") else {
        return false;
    };
    if basename.contains('/') || basename.len() != 10 {
        return false;
    }

    let bytes = basename.as_bytes();
    bytes[0] == b'c'
        && bytes[1] == b'c'
        && bytes[2..8].iter().all(u8::is_ascii_alphanumeric)
        && bytes[8] == b'.'
        && bytes[9] == b's'
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use execsurface_model::canonical::{
        NormalizationMetadata, OpenIntent, PathResolution,
    };
    use execsurface_normalize::{canonicalize_path, NormalizationConfig};

    const PRESERVED_GCC_PATHS: &[&str] = &[
        "/tmp/ccVwi22P.s",
        "/tmp/ccvi9G26.s",
        "/tmp/ccbVlG4H.s",
        "/tmp/cceq5pJm.s",
        "/tmp/ccsKO93q.s",
        "/tmp/ccpk08mk.s",
        "/tmp/cchd1i4f.s",
    ];

    fn config() -> NormalizationConfig {
        NormalizationConfig {
            workspace: Some("/workspace".to_owned()),
            home: Some("/home/test".to_owned()),
            tmp_roots: vec!["/tmp".to_owned()],
            run_tmp: Some("/tmp/run-current".to_owned()),
            caches: BTreeMap::new(),
        }
    }

    fn canonical(path: &str) -> CanonicalPath {
        canonicalize_path(path, &config()).expect("canonicalize path")
    }

    fn executable(path: &str) -> CanonicalExecutable {
        CanonicalExecutable {
            path: CanonicalPath {
                value: path.to_owned(),
                class: PathClass::System,
                resolution: PathResolution::Lexical,
            },
            family: path.rsplit('/').next().unwrap_or(path).to_owned(),
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

    fn file_effect(
        actor: &str,
        operation: FileOperation,
        target: &str,
        open_intent: Option<OpenIntent>,
    ) -> CanonicalEffect {
        let actor = executable(actor);
        CanonicalEffect::FilePathAccess {
            actor: Some(actor.clone()),
            execution_chain: vec![
                executable("/bin/bash"),
                executable("/usr/bin/go"),
                actor,
            ],
            operation,
            target: canonical(target),
            open_intent,
        }
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

    fn gcc_temp_pair(path: &str) -> Vec<CanonicalEffect> {
        vec![
            file_effect(
                BOUNDED_GCC_PATH,
                FileOperation::Open,
                path,
                Some(create_intent()),
            ),
            file_effect(BOUNDED_GCC_PATH, FileOperation::Delete, path, None),
        ]
    }

    #[test]
    fn preserved_issue_62_paths_are_eligible_under_the_frozen_role() {
        for path in PRESERVED_GCC_PATHS {
            let input = surface(gcc_temp_pair(path));
            let eligible = eligible_gcc_temp_paths(&input);
            assert_eq!(eligible.len(), 1);
            assert!(eligible.contains(&canonical(path).value));

            let normalized = apply_candidate(&input);
            assert_eq!(normalized.normalization.profile_version, 4);
            assert!(normalized.effects.iter().all(|effect| match effect {
                CanonicalEffect::FilePathAccess { target, .. } => {
                    target.value == GCC_TEMP_CANONICAL
                }
                _ => false,
            }));
        }
    }

    #[test]
    fn matching_name_without_create_delete_role_is_not_eligible() {
        let only_create = surface(vec![file_effect(
            BOUNDED_GCC_PATH,
            FileOperation::Open,
            "/tmp/ccABC123.s",
            Some(create_intent()),
        )]);
        assert!(eligible_gcc_temp_paths(&only_create).is_empty());

        let only_delete = surface(vec![file_effect(
            BOUNDED_GCC_PATH,
            FileOperation::Delete,
            "/tmp/ccABC123.s",
            None,
        )]);
        assert!(eligible_gcc_temp_paths(&only_delete).is_empty());

        let read_open = surface(vec![
            file_effect(
                BOUNDED_GCC_PATH,
                FileOperation::Open,
                "/tmp/ccABC123.s",
                Some(read_intent()),
            ),
            file_effect(
                BOUNDED_GCC_PATH,
                FileOperation::Delete,
                "/tmp/ccABC123.s",
                None,
            ),
        ]);
        assert!(eligible_gcc_temp_paths(&read_open).is_empty());
    }

    #[test]
    fn second_actor_on_same_identity_forces_collision_rejection() {
        let mut effects = gcc_temp_pair("/tmp/ccABC123.s");
        effects.push(file_effect(
            "/usr/bin/clang",
            FileOperation::Open,
            "/tmp/ccABC123.s",
            Some(read_intent()),
        ));
        let input = surface(effects);
        assert!(eligible_gcc_temp_paths(&input).is_empty());
        assert!(apply_candidate(&input).effects.iter().any(|effect| match effect {
            CanonicalEffect::FilePathAccess { target, .. } => target.value == "$TMP/ccABC123.s",
            _ => false,
        }));
    }

    #[test]
    fn wrong_actor_path_or_family_is_not_laundered() {
        let clang = surface(vec![
            file_effect(
                "/usr/bin/clang",
                FileOperation::Open,
                "/tmp/ccABC123.s",
                Some(create_intent()),
            ),
            file_effect(
                "/usr/bin/clang",
                FileOperation::Delete,
                "/tmp/ccABC123.s",
                None,
            ),
        ]);
        assert!(eligible_gcc_temp_paths(&clang).is_empty());

        let workspace_gcc = surface(vec![
            file_effect(
                "/workspace/gcc",
                FileOperation::Open,
                "/tmp/ccABC123.s",
                Some(create_intent()),
            ),
            file_effect(
                "/workspace/gcc",
                FileOperation::Delete,
                "/tmp/ccABC123.s",
                None,
            ),
        ]);
        assert!(eligible_gcc_temp_paths(&workspace_gcc).is_empty());
    }

    #[test]
    fn negative_grammar_and_root_collision_cases_remain_distinct() {
        let cases = [
            "/workspace/ccABC123.s",
            "/home/test/ccABC123.s",
            "/tmp/sub/ccABC123.s",
            "/tmp/ccABC12.s",
            "/tmp/ccABC1234.s",
            "/tmp/ccABC-23.s",
            "/tmp/CCABC123.s",
            "/tmp/ccABC123.S",
            "/tmp/ccABC123.o",
            "/tmp/ccABC123.s.extra",
            "/tmp/notccABC123.s",
        ];

        for path in cases {
            let input = surface(gcc_temp_pair(path));
            assert!(eligible_gcc_temp_paths(&input).is_empty(), "unexpected eligibility for {path}");
        }
    }

    #[test]
    fn parent_traversal_is_not_eligible() {
        let input = surface(gcc_temp_pair("/tmp/../tmp/ccABC123.s"));
        assert!(eligible_gcc_temp_paths(&input).is_empty());
    }

    #[test]
    fn existing_go_build_normalization_remains_distinct_and_untouched() {
        let go = canonical("/tmp/go-build3008370933/b001/vet.cfg");
        assert_eq!(go.value, "$TMP/go-build<ephemeral>/b001/vet.cfg");

        let input = surface(vec![file_effect(
            BOUNDED_GCC_PATH,
            FileOperation::Open,
            "/tmp/go-build3008370933/b001/vet.cfg",
            Some(create_intent()),
        )]);
        assert!(eligible_gcc_temp_paths(&input).is_empty());
        let candidate = apply_candidate(&input);
        assert!(candidate.effects.iter().any(|effect| match effect {
            CanonicalEffect::FilePathAccess { target, .. } => {
                target.value == "$TMP/go-build<ephemeral>/b001/vet.cfg"
            }
            _ => false,
        }));
    }
}
