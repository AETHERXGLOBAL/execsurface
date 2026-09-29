#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use execsurface_baseline::{
        build_lock, BaselineLock, BaselinePayload, CommandIdentity, ObserverIdentity,
        PlatformIdentity, ToolIdentity,
    };
    use execsurface_model::canonical::{
        CanonicalEffect, CanonicalExecutable, CanonicalPath, CanonicalSurface,
        NormalizationMetadata, OpenIntent, PathClass, PathResolution,
    };
    use execsurface_model::FileOperation;
    use execsurface_p3_accepted_variance::{
        build_contract, AcceptanceSelection, SELECTION_SCHEMA_VERSION,
    };
    use execsurface_p3_gcc_temp_grammar::{
        apply_candidate, eligible_gcc_temp_paths, GCC_TEMP_CANONICAL,
    };
    use execsurface_p3_variance_analyzer::{
        analyze, TrustedLearningManifest, TrustedLearningRun, INPUT_SCHEMA_VERSION,
    };

    fn digest(value: u8) -> String {
        format!("sha256:{value:064x}")
    }

    fn path(value: &str, class: PathClass) -> CanonicalPath {
        CanonicalPath {
            value: value.to_owned(),
            class,
            resolution: PathResolution::Lexical,
        }
    }

    fn executable(value: &str, family: &str) -> CanonicalExecutable {
        CanonicalExecutable {
            path: path(value, PathClass::System),
            family: family.to_owned(),
        }
    }

    fn read_effect_with_chain(
        actor_path: &str,
        family: &str,
        chain: Vec<CanonicalExecutable>,
        target: &str,
    ) -> CanonicalEffect {
        CanonicalEffect::FilePathAccess {
            actor: Some(executable(actor_path, family)),
            execution_chain: chain,
            operation: FileOperation::Read,
            target: path(target, PathClass::System),
            open_intent: None,
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

    fn file_effect(
        actor_path: &str,
        family: &str,
        chain: Vec<CanonicalExecutable>,
        operation: FileOperation,
        target: CanonicalPath,
        open_intent: Option<OpenIntent>,
    ) -> CanonicalEffect {
        CanonicalEffect::FilePathAccess {
            actor: Some(executable(actor_path, family)),
            execution_chain: chain,
            operation,
            target,
            open_intent,
        }
    }

    fn baseline(effects: Vec<CanonicalEffect>) -> BaselineLock {
        build_lock(BaselinePayload::new(
            ToolIdentity {
                name: "execsurface".to_owned(),
                version: "0.1.0-alpha.4".to_owned(),
            },
            CommandIdentity {
                executable: executable("/bin/bash", "bash"),
                argument_count: 0,
                label: None,
            },
            PlatformIdentity {
                os: "linux".to_owned(),
                architecture: "x86_64".to_owned(),
            },
            ObserverIdentity {
                name: "linux-ptrace-metadata-v2".to_owned(),
                capabilities: vec!["descendant_tracking".to_owned()],
                limitations: vec!["test".to_owned()],
            },
            CanonicalSurface {
                schema_version: 2,
                normalization: NormalizationMetadata {
                    profile_version: 3,
                    semantic_roots: vec![
                        "home".to_owned(),
                        "tmp".to_owned(),
                        "workspace".to_owned(),
                    ],
                },
                effects,
            },
        ))
        .expect("build baseline")
    }

    fn run(value: u8, effects: Vec<CanonicalEffect>) -> TrustedLearningRun {
        TrustedLearningRun {
            evidence_digest: digest(value),
            observation_complete: true,
            baseline: baseline(effects),
        }
    }

    fn exact_acceptance_contains(
        contract: &execsurface_p3_accepted_variance::AcceptedVarianceContract,
        effect: &CanonicalEffect,
    ) -> bool {
        contract
            .accepted_variable_effects
            .iter()
            .any(|record| &record.effect == effect)
    }

    #[test]
    fn r5_causal_chain_substitution_cannot_inherit_exact_acceptance() {
        let stable = read_effect_with_chain(
            "/usr/bin/cat",
            "cat",
            vec![
                executable("/bin/bash", "bash"),
                executable("/usr/bin/cat", "cat"),
            ],
            "/stable",
        );
        let accepted = read_effect_with_chain(
            "/usr/bin/gcc",
            "gcc",
            vec![
                executable("/bin/bash", "bash"),
                executable("/usr/bin/gcc", "gcc"),
            ],
            "/shared-target",
        );
        let report = analyze(&TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![
                run(1, vec![stable.clone(), accepted.clone()]),
                run(2, vec![stable.clone()]),
                run(3, vec![stable]),
            ],
        })
        .expect("analyze");
        let contract = build_contract(
            &report,
            &AcceptanceSelection {
                schema_version: SELECTION_SCHEMA_VERSION,
                learning_set_digest: report.learning_set_digest.clone(),
                accepted_effects: vec![accepted.clone()],
            },
        )
        .expect("contract");
        assert!(exact_acceptance_contains(&contract, &accepted));

        let substituted = read_effect_with_chain(
            "/usr/bin/gcc",
            "gcc",
            vec![
                executable("/bin/bash", "bash"),
                executable("/usr/bin/make", "make"),
                executable("/usr/bin/gcc", "gcc"),
            ],
            "/shared-target",
        );
        assert!(!exact_acceptance_contains(&contract, &substituted));
    }

    #[test]
    fn r5_bounded_gcc_projection_preserves_non_target_effects_exactly() {
        let gcc_chain = vec![
            executable("/bin/bash", "bash"),
            executable("/usr/bin/go", "go"),
            executable("/usr/bin/gcc", "gcc"),
        ];
        let random = path("$TMP/ccABC123.s", PathClass::Temp);
        let gcc_open = file_effect(
            "/usr/bin/gcc",
            "gcc",
            gcc_chain.clone(),
            FileOperation::Open,
            random.clone(),
            Some(create_intent()),
        );
        let gcc_delete = file_effect(
            "/usr/bin/gcc",
            "gcc",
            gcc_chain,
            FileOperation::Delete,
            random,
            None,
        );
        let sentinel = read_effect_with_chain(
            "/usr/bin/cat",
            "cat",
            vec![
                executable("/bin/bash", "bash"),
                executable("/usr/bin/cat", "cat"),
            ],
            "$TMP/execsurface-p3-v5-sentinel-meaningful.txt",
        );
        let other = read_effect_with_chain(
            "/usr/bin/go",
            "go",
            vec![
                executable("/bin/bash", "bash"),
                executable("/usr/bin/go", "go"),
            ],
            "$WORKSPACE/go.mod",
        );
        let raw = CanonicalSurface {
            schema_version: 2,
            normalization: NormalizationMetadata {
                profile_version: 3,
                semantic_roots: vec![
                    "home".to_owned(),
                    "tmp".to_owned(),
                    "workspace".to_owned(),
                ],
            },
            effects: vec![gcc_open, gcc_delete, sentinel.clone(), other.clone()],
        };
        assert_eq!(eligible_gcc_temp_paths(&raw).len(), 1);

        let projected = apply_candidate(&raw);
        let projected_set = projected.effects.iter().cloned().collect::<BTreeSet<_>>();
        assert!(projected_set.contains(&sentinel));
        assert!(projected_set.contains(&other));
        assert!(projected.effects.iter().any(|effect| match effect {
            CanonicalEffect::FilePathAccess { target, .. } => target.value == GCC_TEMP_CANONICAL,
            _ => false,
        }));
    }

    #[test]
    fn r5_wrong_actor_gcc_mimic_is_neither_eligible_nor_projected() {
        let target = path("$TMP/ccABC123.s", PathClass::Temp);
        let clang = executable("/usr/bin/clang", "clang");
        let surface = CanonicalSurface {
            schema_version: 2,
            normalization: NormalizationMetadata {
                profile_version: 3,
                semantic_roots: vec!["tmp".to_owned()],
            },
            effects: vec![
                file_effect(
                    "/usr/bin/clang",
                    "clang",
                    vec![executable("/bin/bash", "bash"), clang.clone()],
                    FileOperation::Open,
                    target.clone(),
                    Some(create_intent()),
                ),
                file_effect(
                    "/usr/bin/clang",
                    "clang",
                    vec![executable("/bin/bash", "bash"), clang],
                    FileOperation::Delete,
                    target.clone(),
                    None,
                ),
            ],
        };
        assert!(eligible_gcc_temp_paths(&surface).is_empty());
        let projected = apply_candidate(&surface);
        assert!(projected.effects.iter().any(|effect| match effect {
            CanonicalEffect::FilePathAccess { target: seen, .. } => seen == &target,
            _ => false,
        }));
        assert!(!projected.effects.iter().any(|effect| match effect {
            CanonicalEffect::FilePathAccess { target, .. } => target.value == GCC_TEMP_CANONICAL,
            _ => false,
        }));
    }

    #[test]
    fn r5_empty_selection_never_authorizes_recurrent_variable() {
        let stable = read_effect_with_chain(
            "/usr/bin/cat",
            "cat",
            vec![
                executable("/bin/bash", "bash"),
                executable("/usr/bin/cat", "cat"),
            ],
            "/stable",
        );
        let variable = read_effect_with_chain(
            "/usr/bin/go",
            "go",
            vec![
                executable("/bin/bash", "bash"),
                executable("/usr/bin/go", "go"),
            ],
            "/variable",
        );
        let report = analyze(&TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs: vec![
                run(1, vec![stable.clone(), variable.clone()]),
                run(2, vec![stable.clone(), variable]),
                run(3, vec![stable]),
            ],
        })
        .expect("analyze");
        let contract = build_contract(
            &report,
            &AcceptanceSelection {
                schema_version: SELECTION_SCHEMA_VERSION,
                learning_set_digest: report.learning_set_digest.clone(),
                accepted_effects: vec![],
            },
        )
        .expect("contract");
        assert!(contract.accepted_variable_effects.is_empty());
    }
}
