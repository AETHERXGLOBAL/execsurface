#[cfg(test)]
mod tests {
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
        build_contract, serialize_contract, AcceptanceSelection, ContractError,
        SELECTION_SCHEMA_VERSION,
    };
    use execsurface_p3_gcc_temp_grammar::eligible_gcc_temp_paths;
    use execsurface_p3_variance_analyzer::{
        analyze, serialize_report, AnalyzeError, RecurrenceClass, TrustedLearningManifest,
        TrustedLearningRun, INPUT_SCHEMA_VERSION,
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

    fn read_effect(target: &str) -> CanonicalEffect {
        CanonicalEffect::FilePathAccess {
            actor: None,
            execution_chain: vec![],
            operation: FileOperation::Read,
            target: path(target, PathClass::System),
            open_intent: None,
        }
    }

    fn actor_read_effect(actor_path: &str, family: &str, target: &str) -> CanonicalEffect {
        let actor = executable(actor_path, family);
        CanonicalEffect::FilePathAccess {
            actor: Some(actor.clone()),
            execution_chain: vec![executable("/bin/bash", "bash"), actor],
            operation: FileOperation::Read,
            target: path(target, PathClass::System),
            open_intent: None,
        }
    }

    fn baseline_with_profile(
        effects: Vec<CanonicalEffect>,
        architecture: &str,
        observer_name: &str,
        normalization_profile_version: u32,
    ) -> BaselineLock {
        build_lock(BaselinePayload::new(
            ToolIdentity {
                name: "execsurface".to_owned(),
                version: "0.1.0-alpha.4".to_owned(),
            },
            CommandIdentity {
                executable: executable("/usr/bin/example", "example"),
                argument_count: 0,
                label: None,
            },
            PlatformIdentity {
                os: "linux".to_owned(),
                architecture: architecture.to_owned(),
            },
            ObserverIdentity {
                name: observer_name.to_owned(),
                capabilities: vec!["descendant_tracking".to_owned()],
                limitations: vec!["test".to_owned()],
            },
            CanonicalSurface {
                schema_version: 2,
                normalization: NormalizationMetadata {
                    profile_version: normalization_profile_version,
                    semantic_roots: vec!["workspace".to_owned(), "tmp".to_owned()],
                },
                effects,
            },
        ))
        .expect("build baseline")
    }

    fn baseline(effects: Vec<CanonicalEffect>) -> BaselineLock {
        baseline_with_profile(effects, "x86_64", "linux-ptrace-metadata-v2", 3)
    }

    fn run(value: u8, effects: Vec<CanonicalEffect>) -> TrustedLearningRun {
        TrustedLearningRun {
            evidence_digest: digest(value),
            observation_complete: true,
            baseline: baseline(effects),
        }
    }

    fn manifest(runs: Vec<TrustedLearningRun>) -> TrustedLearningManifest {
        TrustedLearningManifest {
            schema_version: INPUT_SCHEMA_VERSION,
            runs,
        }
    }

    fn empty_selection(learning_set_digest: &str) -> AcceptanceSelection {
        AcceptanceSelection {
            schema_version: SELECTION_SCHEMA_VERSION,
            learning_set_digest: learning_set_digest.to_owned(),
            accepted_effects: vec![],
        }
    }

    fn explicit_selection(
        learning_set_digest: &str,
        accepted_effects: Vec<CanonicalEffect>,
    ) -> AcceptanceSelection {
        AcceptanceSelection {
            schema_version: SELECTION_SCHEMA_VERSION,
            learning_set_digest: learning_set_digest.to_owned(),
            accepted_effects,
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

    fn gcc_path_effect(
        actor_path: &str,
        actor_family: &str,
        operation: FileOperation,
        target: CanonicalPath,
        open_intent: Option<OpenIntent>,
    ) -> CanonicalEffect {
        let actor = executable(actor_path, actor_family);
        CanonicalEffect::FilePathAccess {
            actor: Some(actor.clone()),
            execution_chain: vec![
                executable("/bin/bash", "bash"),
                executable("/usr/bin/go", "go"),
                actor,
            ],
            operation,
            target,
            open_intent,
        }
    }

    fn gcc_pair(
        actor_path: &str,
        actor_family: &str,
        target: CanonicalPath,
    ) -> CanonicalSurface {
        CanonicalSurface {
            schema_version: 2,
            normalization: NormalizationMetadata {
                profile_version: 3,
                semantic_roots: vec!["tmp".to_owned()],
            },
            effects: vec![
                gcc_path_effect(
                    actor_path,
                    actor_family,
                    FileOperation::Open,
                    target.clone(),
                    Some(create_intent()),
                ),
                gcc_path_effect(
                    actor_path,
                    actor_family,
                    FileOperation::Delete,
                    target,
                    None,
                ),
            ],
        }
    }

    fn is_exactly_accepted(
        contract: &execsurface_p3_accepted_variance::AcceptedVarianceContract,
        effect: &CanonicalEffect,
    ) -> bool {
        contract
            .accepted_variable_effects
            .iter()
            .any(|record| &record.effect == effect)
    }

    #[test]
    fn a1_single_run_injection_is_observed_not_authorized() {
        let invariant = read_effect("/stable");
        let malicious = read_effect("/malicious");
        let report = analyze(&manifest(vec![
            run(1, vec![invariant.clone(), malicious.clone()]),
            run(2, vec![invariant.clone()]),
            run(3, vec![invariant]),
        ]))
        .expect("analyze");

        let record = report
            .recurrence
            .iter()
            .find(|record| record.effect == malicious)
            .expect("malicious recurrence");
        assert_eq!(record.support_count, 1);
        assert_eq!(record.class, RecurrenceClass::VariableCandidate);

        let contract = build_contract(&report, &empty_selection(&report.learning_set_digest))
            .expect("contract");
        assert!(contract.accepted_variable_effects.is_empty());
    }

    #[test]
    fn a2_repeated_injection_still_does_not_auto_authorize() {
        let invariant = read_effect("/stable");
        let malicious = read_effect("/malicious");
        let report = analyze(&manifest(vec![
            run(1, vec![invariant.clone(), malicious.clone()]),
            run(2, vec![invariant.clone(), malicious.clone()]),
            run(3, vec![invariant]),
        ]))
        .expect("analyze");

        let record = report
            .recurrence
            .iter()
            .find(|record| record.effect == malicious)
            .expect("malicious recurrence");
        assert_eq!(record.support_count, 2);
        assert_eq!(record.class, RecurrenceClass::VariableCandidate);

        let contract = build_contract(&report, &empty_selection(&report.learning_set_digest))
            .expect("contract");
        assert!(contract.accepted_variable_effects.is_empty());
    }

    #[test]
    fn a3_duplicate_vote_inflation_is_rejected() {
        let duplicate = run(1, vec![read_effect("/stable")]);
        let result = analyze(&manifest(vec![duplicate.clone(), duplicate]));
        assert!(matches!(result, Err(AnalyzeError::DuplicateEvidenceDigest(_))));
    }

    #[test]
    fn a4_incomplete_run_poisoning_is_rejected() {
        let stable = read_effect("/stable");
        let mut poisoned = run(2, vec![stable.clone(), read_effect("/malicious")]);
        poisoned.observation_complete = false;
        let result = analyze(&manifest(vec![run(1, vec![stable]), poisoned]));
        assert!(matches!(result, Err(AnalyzeError::IncompleteRun(_))));
    }

    #[test]
    fn a5_run_order_permutation_is_byte_stable() {
        let stable = read_effect("/stable");
        let variable = read_effect("/variable");
        let first = analyze(&manifest(vec![
            run(1, vec![stable.clone(), variable.clone()]),
            run(2, vec![stable.clone()]),
            run(3, vec![stable.clone(), variable.clone()]),
        ]))
        .expect("first");
        let second = analyze(&manifest(vec![
            run(3, vec![stable.clone(), variable.clone()]),
            run(1, vec![stable.clone(), variable.clone()]),
            run(2, vec![stable]),
        ]))
        .expect("second");

        assert_eq!(
            serialize_report(&first).expect("first report bytes"),
            serialize_report(&second).expect("second report bytes")
        );

        let first_contract = build_contract(
            &first,
            &explicit_selection(&first.learning_set_digest, vec![variable.clone()]),
        )
        .expect("first contract");
        let second_contract = build_contract(
            &second,
            &explicit_selection(&second.learning_set_digest, vec![variable]),
        )
        .expect("second contract");
        assert_eq!(
            serialize_contract(&first_contract).expect("first contract bytes"),
            serialize_contract(&second_contract).expect("second contract bytes")
        );
    }

    #[test]
    fn a6_profile_observer_and_normalization_mismatch_are_rejected() {
        let stable = read_effect("/stable");

        let arch = TrustedLearningRun {
            evidence_digest: digest(2),
            observation_complete: true,
            baseline: baseline_with_profile(
                vec![stable.clone()],
                "aarch64",
                "linux-ptrace-metadata-v2",
                3,
            ),
        };
        assert!(matches!(
            analyze(&manifest(vec![run(1, vec![stable.clone()]), arch])),
            Err(AnalyzeError::IncomparableProfile { .. })
        ));

        let observer = TrustedLearningRun {
            evidence_digest: digest(3),
            observation_complete: true,
            baseline: baseline_with_profile(
                vec![stable.clone()],
                "x86_64",
                "different-observer",
                3,
            ),
        };
        assert!(matches!(
            analyze(&manifest(vec![run(1, vec![stable.clone()]), observer])),
            Err(AnalyzeError::IncomparableProfile { .. })
        ));

        let normalization = TrustedLearningRun {
            evidence_digest: digest(4),
            observation_complete: true,
            baseline: baseline_with_profile(
                vec![stable.clone()],
                "x86_64",
                "linux-ptrace-metadata-v2",
                4,
            ),
        };
        assert!(matches!(
            analyze(&manifest(vec![run(1, vec![stable]), normalization])),
            Err(AnalyzeError::IncomparableProfile { .. })
        ));
    }

    #[test]
    fn a7_gcc_grammar_mimicry_outside_bounded_role_is_rejected() {
        let matching = path("$TMP/ccABC123.s", PathClass::Temp);
        let wrong_root = path("$WORKSPACE/ccABC123.s", PathClass::Workspace);
        let nested = path("$TMP/sub/ccABC123.s", PathClass::Temp);

        assert!(eligible_gcc_temp_paths(&gcc_pair(
            "/usr/bin/clang",
            "clang",
            matching.clone()
        ))
        .is_empty());
        assert!(eligible_gcc_temp_paths(&gcc_pair(
            "/usr/bin/gcc",
            "gcc",
            wrong_root
        ))
        .is_empty());
        assert!(eligible_gcc_temp_paths(&gcc_pair(
            "/usr/bin/gcc",
            "gcc",
            nested
        ))
        .is_empty());

        let create_only = CanonicalSurface {
            schema_version: 2,
            normalization: NormalizationMetadata {
                profile_version: 3,
                semantic_roots: vec!["tmp".to_owned()],
            },
            effects: vec![gcc_path_effect(
                "/usr/bin/gcc",
                "gcc",
                FileOperation::Open,
                matching,
                Some(create_intent()),
            )],
        };
        assert!(eligible_gcc_temp_paths(&create_only).is_empty());
    }

    #[test]
    fn a8_actor_substitution_does_not_match_exact_acceptance() {
        let stable = read_effect("/stable");
        let accepted = actor_read_effect("/usr/bin/gcc", "gcc", "/shared-target");
        let report = analyze(&manifest(vec![
            run(1, vec![stable.clone(), accepted.clone()]),
            run(2, vec![stable.clone()]),
            run(3, vec![stable]),
        ]))
        .expect("analyze");
        let contract = build_contract(
            &report,
            &explicit_selection(&report.learning_set_digest, vec![accepted.clone()]),
        )
        .expect("contract");
        assert!(is_exactly_accepted(&contract, &accepted));

        let substituted = actor_read_effect("/usr/bin/clang", "clang", "/shared-target");
        assert!(!is_exactly_accepted(&contract, &substituted));
    }

    #[test]
    fn a9_unseen_similar_target_does_not_match_exact_acceptance() {
        let stable = read_effect("/stable");
        let accepted = read_effect("/cache/item-123");
        let report = analyze(&manifest(vec![
            run(1, vec![stable.clone(), accepted.clone()]),
            run(2, vec![stable.clone()]),
            run(3, vec![stable]),
        ]))
        .expect("analyze");
        let contract = build_contract(
            &report,
            &explicit_selection(&report.learning_set_digest, vec![accepted.clone()]),
        )
        .expect("contract");
        assert!(is_exactly_accepted(&contract, &accepted));
        assert!(!is_exactly_accepted(
            &contract,
            &read_effect("/cache/item-124")
        ));
    }

    #[test]
    fn a10_invariant_laundering_is_rejected() {
        let invariant = read_effect("/stable");
        let variable = read_effect("/variable");
        let report = analyze(&manifest(vec![
            run(1, vec![invariant.clone(), variable]),
            run(2, vec![invariant.clone()]),
            run(3, vec![invariant.clone()]),
        ]))
        .expect("analyze");
        let result = build_contract(
            &report,
            &explicit_selection(&report.learning_set_digest, vec![invariant]),
        );
        assert_eq!(result, Err(ContractError::InvariantSelected));
    }
}
