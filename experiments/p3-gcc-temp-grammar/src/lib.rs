use execsurface_model::canonical::{CanonicalEffect, CanonicalPath, CanonicalSurface, PathClass};

pub const CANDIDATE_NORMALIZATION_PROFILE_VERSION: u32 = 4;
pub const GCC_TEMP_CANONICAL: &str = "$TMP/cc<ephemeral>.s";

pub fn normalize_gcc_temp_candidate(path: &CanonicalPath) -> CanonicalPath {
    if path.class != PathClass::Temp || !matches_gcc_temp_identity(&path.value) {
        return path.clone();
    }

    let mut normalized = path.clone();
    normalized.value = GCC_TEMP_CANONICAL.to_owned();
    normalized
}

pub fn apply_candidate(surface: &CanonicalSurface) -> CanonicalSurface {
    let mut candidate = surface.clone();
    candidate.normalization.profile_version = CANDIDATE_NORMALIZATION_PROFILE_VERSION;
    candidate.effects = candidate
        .effects
        .into_iter()
        .map(normalize_effect)
        .collect();
    candidate.effects.sort();
    candidate.effects.dedup();
    candidate
}

fn normalize_effect(effect: CanonicalEffect) -> CanonicalEffect {
    match effect {
        CanonicalEffect::FilePathAccess {
            actor,
            execution_chain,
            operation,
            target,
            open_intent,
        } => CanonicalEffect::FilePathAccess {
            actor,
            execution_chain,
            operation,
            target: normalize_gcc_temp_candidate(&target),
            open_intent,
        },
        CanonicalEffect::FileRename {
            actor,
            execution_chain,
            from,
            to,
        } => CanonicalEffect::FileRename {
            actor,
            execution_chain,
            from: normalize_gcc_temp_candidate(&from),
            to: normalize_gcc_temp_candidate(&to),
        },
        other => other,
    }
}

fn matches_gcc_temp_identity(value: &str) -> bool {
    let Some(basename) = value.strip_prefix("$TMP/") else {
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
        CanonicalExecutable, NormalizationMetadata, OpenIntent, PathResolution,
    };
    use execsurface_model::FileOperation;
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

    fn file_effect(
        actor: &str,
        chain: &[&str],
        operation: FileOperation,
        target: &str,
        open_intent: Option<OpenIntent>,
    ) -> CanonicalEffect {
        CanonicalEffect::FilePathAccess {
            actor: Some(executable(actor)),
            execution_chain: chain.iter().map(|path| executable(path)).collect(),
            operation,
            target: canonical(target),
            open_intent,
        }
    }

    fn open_read_intent() -> OpenIntent {
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

    #[test]
    fn preserved_fzf_gcc_paths_collapse_to_one_identity() {
        let normalized = PRESERVED_GCC_PATHS
            .iter()
            .map(|path| normalize_gcc_temp_candidate(&canonical(path)))
            .collect::<Vec<_>>();

        assert!(normalized
            .iter()
            .all(|path| path.value == GCC_TEMP_CANONICAL));
        assert!(normalized.iter().all(|path| path.class == PathClass::Temp));
        assert!(normalized.windows(2).all(|pair| pair[0] == pair[1]));
    }

    #[test]
    fn additional_six_ascii_alphanumeric_names_are_in_scope() {
        for path in [
            "/tmp/ccABC123.s",
            "/tmp/cc000000.s",
            "/tmp/cczzZZ99.s",
            "/tmp/cca1B2c3.s",
        ] {
            assert_eq!(
                normalize_gcc_temp_candidate(&canonical(path)).value,
                GCC_TEMP_CANONICAL
            );
        }
    }

    #[test]
    fn negative_collision_cases_remain_specific() {
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

        for input in cases {
            let before = canonical(input);
            let after = normalize_gcc_temp_candidate(&before);
            assert_eq!(after, before, "unexpected normalization for {input}");
        }
    }

    #[test]
    fn parent_traversal_remains_unknown_and_is_not_normalized() {
        let before = canonical("/tmp/../tmp/ccABC123.s");
        assert_eq!(before.class, PathClass::Unknown);
        let after = normalize_gcc_temp_candidate(&before);
        assert_eq!(after, before);
        assert_ne!(after.value, GCC_TEMP_CANONICAL);
    }

    #[test]
    fn candidate_profile_bump_preserves_actor_operation_chain_and_intent() {
        let gcc_open = file_effect(
            "/usr/bin/gcc",
            &["/usr/bin/bash", "/usr/local/go/bin/go", "/usr/bin/gcc"],
            FileOperation::Open,
            "/tmp/ccABC123.s",
            Some(open_read_intent()),
        );
        let gcc_delete = file_effect(
            "/usr/bin/gcc",
            &["/usr/bin/bash", "/usr/local/go/bin/go", "/usr/bin/gcc"],
            FileOperation::Delete,
            "/tmp/ccXYZ789.s",
            None,
        );
        let other_actor = file_effect(
            "/usr/bin/clang",
            &["/usr/bin/bash", "/usr/bin/clang"],
            FileOperation::Open,
            "/tmp/ccQWE456.s",
            Some(open_read_intent()),
        );
        let different_intent = file_effect(
            "/usr/bin/gcc",
            &["/usr/bin/bash", "/usr/local/go/bin/go", "/usr/bin/gcc"],
            FileOperation::Open,
            "/tmp/ccRTY321.s",
            Some(OpenIntent {
                read: false,
                write: true,
                create: true,
                truncate: false,
                append: false,
                path_only: false,
                resolve_flags: 0,
                other_flags: 0,
            }),
        );

        let source = CanonicalSurface {
            schema_version: 2,
            normalization: NormalizationMetadata {
                profile_version: 3,
                semantic_roots: vec!["tmp".to_owned()],
            },
            effects: vec![
                gcc_open.clone(),
                gcc_delete.clone(),
                other_actor.clone(),
                different_intent.clone(),
            ],
        };

        let candidate = apply_candidate(&source);
        assert_eq!(candidate.normalization.profile_version, 4);
        assert_eq!(candidate.effects.len(), 4);

        for effect in &candidate.effects {
            match effect {
                CanonicalEffect::FilePathAccess { target, .. } => {
                    assert_eq!(target.value, GCC_TEMP_CANONICAL);
                }
                _ => panic!("unexpected effect"),
            }
        }

        let transformed_open = normalize_effect(gcc_open);
        let transformed_delete = normalize_effect(gcc_delete);
        let transformed_other_actor = normalize_effect(other_actor);
        let transformed_different_intent = normalize_effect(different_intent);

        assert_ne!(transformed_open, transformed_delete);
        assert_ne!(transformed_open, transformed_other_actor);
        assert_ne!(transformed_open, transformed_different_intent);
    }

    #[test]
    fn existing_go_build_normalization_remains_distinct_and_untouched() {
        let go = canonical("/tmp/go-build3008370933/b001/vet.cfg");
        assert_eq!(go.value, "$TMP/go-build<ephemeral>/b001/vet.cfg");
        assert_eq!(normalize_gcc_temp_candidate(&go), go);
    }
}
