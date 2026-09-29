use std::collections::BTreeSet;

use execsurface_p4_backend_authority::AuthorityState;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum GapClass {
    #[serde(rename = "no_gap")]
    None,
    RepresentationGap,
    ObservationGap,
    SuccessSemanticsGap,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct GapRow {
    proposition_id: &'static str,
    raw_v2_evidence: &'static [&'static str],
    mapped_shape: &'static str,
    best_authority: AuthorityState,
    primary_gap: GapClass,
    completeness_satisfied: &'static [&'static str],
    completeness_not_proven: &'static [&'static str],
    identity_basis: &'static str,
    temporal_binding: &'static str,
    causal_binding: &'static str,
    reason_codes: &'static [&'static str],
    usable_for_frozen_product_question: bool,
    second_backend_could_plausibly_strengthen: bool,
    exact_missing_evidence: &'static str,
}

fn matrix() -> Vec<GapRow> {
    vec![
        GapRow {
            proposition_id: "P4.PROC.CREATE_RELATION",
            raw_v2_evidence: &["process_spawn"],
            mapped_shape: "ProcessCreated",
            best_authority: AuthorityState::Direct,
            primary_gap: GapClass::None,
            completeness_satisfied: &["session_scope", "capability", "causal_chain"],
            completeness_not_proven: &[],
            identity_basis: "runtime_process_relation",
            temporal_binding: "lifecycle_event",
            causal_binding: "direct_event",
            reason_codes: &[],
            usable_for_frozen_product_question: true,
            second_backend_could_plausibly_strengthen: false,
            exact_missing_evidence: "none within the bounded declared scope",
        },
        GapRow {
            proposition_id: "P4.EXEC.SUCCESS",
            raw_v2_evidence: &["process_exec_success"],
            mapped_shape: "ProcessExecSucceeded",
            best_authority: AuthorityState::Direct,
            primary_gap: GapClass::None,
            completeness_satisfied: &["session_scope", "capability", "causal_chain"],
            completeness_not_proven: &[],
            identity_basis: "runtime_executable_identity",
            temporal_binding: "successful_operation_result",
            causal_binding: "direct_event",
            reason_codes: &[],
            usable_for_frozen_product_question: true,
            second_backend_could_plausibly_strengthen: false,
            exact_missing_evidence: "none within the bounded declared scope",
        },
        GapRow {
            proposition_id: "P4.PATH.ACCESS_ATTEMPT",
            raw_v2_evidence: &["file_open_path_argument"],
            mapped_shape: "FilePathnameAttemptObserved",
            best_authority: AuthorityState::AttemptOnly,
            primary_gap: GapClass::None,
            completeness_satisfied: &["session_scope", "capability"],
            completeness_not_proven: &["object_identity"],
            identity_basis: "lexical_argument",
            temporal_binding: "pre_operation_intent",
            causal_binding: "direct_event",
            reason_codes: &[],
            usable_for_frozen_product_question: true,
            second_backend_could_plausibly_strengthen: false,
            exact_missing_evidence: "none for the explicit attempt-scoped proposition",
        },
        GapRow {
            proposition_id: "P4.FILE.OPEN_OBJECT",
            raw_v2_evidence: &["file_open_path_argument", "positive_fd_io_when_present"],
            mapped_shape: "FileOpenObjectObserved",
            best_authority: AuthorityState::Unsupported,
            primary_gap: GapClass::ObservationGap,
            completeness_satisfied: &["session_scope"],
            completeness_not_proven: &["capability", "object_identity"],
            identity_basis: "insufficient_for_successful_open_object",
            temporal_binding: "attempt_only_without_open_result",
            causal_binding: "direct_attempt_only",
            reason_codes: &["successful_open_object_not_observed_in_raw_v2"],
            usable_for_frozen_product_question: false,
            second_backend_could_plausibly_strengthen: true,
            exact_missing_evidence: "post-open success/result evidence with kernel-grounded opened-object identity even when no later IO occurs",
        },
        GapRow {
            proposition_id: "P4.FD.IO_ATTRIBUTION",
            raw_v2_evidence: &["positive_fd_read_write", "fd_state_machine", "process_spawn"],
            mapped_shape: "FileFdIoObserved",
            best_authority: AuthorityState::Direct,
            primary_gap: GapClass::RepresentationGap,
            completeness_satisfied: &["session_scope", "capability", "object_identity"],
            completeness_not_proven: &["fd_table_relation_under_raw_v2_clone"],
            identity_basis: "runtime_fd_path_correlated",
            temporal_binding: "successful_operation_result",
            causal_binding: "state_machine_correlated",
            reason_codes: &["raw_v2_clone_discards_clone_files_relation"],
            usable_for_frozen_product_question: true,
            second_backend_could_plausibly_strengthen: false,
            exact_missing_evidence: "retain the already-observed clone/clone3 CLONE_FILES relation in proof-bearing evidence so dependent FD attribution can remain complete",
        },
        GapRow {
            proposition_id: "P4.FILE.RENAME_DELETE",
            raw_v2_evidence: &["rename_delete_path_arguments"],
            mapped_shape: "FileRenameAttemptObserved",
            best_authority: AuthorityState::AttemptOnly,
            primary_gap: GapClass::SuccessSemanticsGap,
            completeness_satisfied: &["session_scope", "capability"],
            completeness_not_proven: &["successful_effect"],
            identity_basis: "lexical_argument",
            temporal_binding: "pre_operation_intent",
            causal_binding: "direct_event",
            reason_codes: &["attempt_does_not_establish_successful_rename_delete"],
            usable_for_frozen_product_question: false,
            second_backend_could_plausibly_strengthen: true,
            exact_missing_evidence: "post-operation success/result evidence for the rename/delete effect with preserved subject and target identity",
        },
        GapRow {
            proposition_id: "P4.NET.CONNECT_DESTINATION",
            raw_v2_evidence: &["connect_sockaddr_argument"],
            mapped_shape: "NetworkConnectDestinationAttemptObserved",
            best_authority: AuthorityState::AttemptOnly,
            primary_gap: GapClass::SuccessSemanticsGap,
            completeness_satisfied: &["session_scope", "capability"],
            completeness_not_proven: &["successful_effect"],
            identity_basis: "attempted_endpoint_argument",
            temporal_binding: "pre_operation_intent",
            causal_binding: "direct_event",
            reason_codes: &["attempt_does_not_establish_successful_connect"],
            usable_for_frozen_product_question: false,
            second_backend_could_plausibly_strengthen: true,
            exact_missing_evidence: "post-connect success/result evidence bound to the attempted endpoint and originating execution chain",
        },
        GapRow {
            proposition_id: "P4.CAUSAL.EXEC_LINEAGE",
            raw_v2_evidence: &["process_spawn", "process_exec_success", "execution_chain_state"],
            mapped_shape: "CausalExecLineageObserved",
            best_authority: AuthorityState::DerivedBounded,
            primary_gap: GapClass::None,
            completeness_satisfied: &["session_scope", "capability", "causal_chain"],
            completeness_not_proven: &[],
            identity_basis: "runtime_process_relation",
            temporal_binding: "session_lifecycle",
            causal_binding: "named_lineage_state_machine_derivation",
            reason_codes: &[],
            usable_for_frozen_product_question: true,
            second_backend_could_plausibly_strengthen: false,
            exact_missing_evidence: "none within the named bounded derivation contract",
        },
        GapRow {
            proposition_id: "P4.OBSERVER.HEALTH_LOSS",
            raw_v2_evidence: &["observation_complete", "warning_codes", "resource_truncation"],
            mapped_shape: "ObserverHealthLoss",
            best_authority: AuthorityState::Lost,
            primary_gap: GapClass::None,
            completeness_satisfied: &["health_state_is_explicit"],
            completeness_not_proven: &[],
            identity_basis: "observer_session",
            temporal_binding: "session_lifecycle",
            causal_binding: "direct_observer_state",
            reason_codes: &["loss_is_first_class_and_blocks_dependent_authority"],
            usable_for_frozen_product_question: true,
            second_backend_could_plausibly_strengthen: false,
            exact_missing_evidence: "none for detecting and preserving observer loss as a first-class state",
        },
        GapRow {
            proposition_id: "P4.FDTABLE.RELATION",
            raw_v2_evidence: &["process_spawn_mechanism", "fork_vfork_semantics"],
            mapped_shape: "FdTableRelationObserved",
            best_authority: AuthorityState::DerivedBounded,
            primary_gap: GapClass::RepresentationGap,
            completeness_satisfied: &["fork_vfork_relation"],
            completeness_not_proven: &["clone_clone3_fd_table_relation"],
            identity_basis: "process_creation_relation",
            temporal_binding: "creation_transition",
            causal_binding: "bounded_relation_derivation",
            reason_codes: &["raw_v2_clone_discards_clone_files_relation"],
            usable_for_frozen_product_question: false,
            second_backend_could_plausibly_strengthen: false,
            exact_missing_evidence: "retain the already-observed clone/clone3 flags and causal pairing as a proof-bearing fd-table relationship certificate",
        },
    ]
}

fn by_id<'a>(rows: &'a [GapRow], id: &str) -> &'a GapRow {
    rows.iter()
        .find(|row| row.proposition_id == id)
        .expect("matrix row")
}

#[test]
fn a2_inventory_is_exactly_the_ten_frozen_propositions_once_each() {
    let rows = matrix();
    assert_eq!(rows.len(), 10);
    let ids: BTreeSet<_> = rows.iter().map(|row| row.proposition_id).collect();
    assert_eq!(ids.len(), 10);
    assert_eq!(
        ids,
        BTreeSet::from([
            "P4.CAUSAL.EXEC_LINEAGE",
            "P4.EXEC.SUCCESS",
            "P4.FD.IO_ATTRIBUTION",
            "P4.FDTABLE.RELATION",
            "P4.FILE.OPEN_OBJECT",
            "P4.FILE.RENAME_DELETE",
            "P4.NET.CONNECT_DESTINATION",
            "P4.OBSERVER.HEALTH_LOSS",
            "P4.PATH.ACCESS_ATTEMPT",
            "P4.PROC.CREATE_RELATION",
        ])
    );
}

#[test]
fn a2_direct_and_bounded_no_gap_rows_are_explicit_not_global_backend_claims() {
    let rows = matrix();
    for id in ["P4.PROC.CREATE_RELATION", "P4.EXEC.SUCCESS"] {
        let row = by_id(&rows, id);
        assert_eq!(row.best_authority, AuthorityState::Direct);
        assert_eq!(row.primary_gap, GapClass::None);
        assert!(row.usable_for_frozen_product_question);
    }
    let lineage = by_id(&rows, "P4.CAUSAL.EXEC_LINEAGE");
    assert_eq!(lineage.best_authority, AuthorityState::DerivedBounded);
    assert_eq!(lineage.primary_gap, GapClass::None);
}

#[test]
fn a2_attempt_scope_is_useful_but_never_laundered_into_success_rows() {
    let rows = matrix();
    let path = by_id(&rows, "P4.PATH.ACCESS_ATTEMPT");
    assert_eq!(path.best_authority, AuthorityState::AttemptOnly);
    assert_eq!(path.primary_gap, GapClass::None);
    assert!(path.usable_for_frozen_product_question);

    for id in ["P4.FILE.RENAME_DELETE", "P4.NET.CONNECT_DESTINATION"] {
        let row = by_id(&rows, id);
        assert_eq!(row.best_authority, AuthorityState::AttemptOnly);
        assert_eq!(row.primary_gap, GapClass::SuccessSemanticsGap);
        assert!(!row.usable_for_frozen_product_question);
        assert!(row.second_backend_could_plausibly_strengthen);
    }
}

#[test]
fn a2_successful_open_object_remains_an_observation_gap_not_path_authority() {
    let rows = matrix();
    let row = by_id(&rows, "P4.FILE.OPEN_OBJECT");
    assert_eq!(row.best_authority, AuthorityState::Unsupported);
    assert_eq!(row.primary_gap, GapClass::ObservationGap);
    assert!(!row.usable_for_frozen_product_question);
    assert!(row.second_backend_could_plausibly_strengthen);
    assert!(row.completeness_not_proven.contains(&"object_identity"));
}

#[test]
fn a2_clone_fd_dependencies_remain_representation_gaps_not_false_completeness() {
    let rows = matrix();
    let fd_io = by_id(&rows, "P4.FD.IO_ATTRIBUTION");
    assert_eq!(fd_io.primary_gap, GapClass::RepresentationGap);
    assert!(fd_io
        .completeness_not_proven
        .contains(&"fd_table_relation_under_raw_v2_clone"));

    let relation = by_id(&rows, "P4.FDTABLE.RELATION");
    assert_eq!(relation.primary_gap, GapClass::RepresentationGap);
    assert!(!relation.usable_for_frozen_product_question);
    assert!(!relation.second_backend_could_plausibly_strengthen);
}

#[test]
fn a2_representation_gap_prefers_retained_existing_evidence_over_new_backend_work() {
    let rows = matrix();
    for id in ["P4.FD.IO_ATTRIBUTION", "P4.FDTABLE.RELATION"] {
        let row = by_id(&rows, id);
        assert_eq!(row.primary_gap, GapClass::RepresentationGap);
        assert!(!row.second_backend_could_plausibly_strengthen);
        assert!(row.exact_missing_evidence.contains("retain"));
    }
}

#[test]
fn a2_observer_loss_is_preserved_as_first_class_fail_closed_state() {
    let rows = matrix();
    let row = by_id(&rows, "P4.OBSERVER.HEALTH_LOSS");
    assert_eq!(row.best_authority, AuthorityState::Lost);
    assert_eq!(row.primary_gap, GapClass::None);
    assert!(row.usable_for_frozen_product_question);
    assert!(row
        .reason_codes
        .iter()
        .any(|reason| reason.contains("blocks")));
}

#[test]
fn a2_every_gap_row_names_exact_missing_evidence_and_reason() {
    for row in matrix()
        .iter()
        .filter(|row| row.primary_gap != GapClass::None)
    {
        assert!(
            !row.reason_codes.is_empty(),
            "{} missing reason",
            row.proposition_id
        );
        assert!(!row.exact_missing_evidence.trim().is_empty());
        assert_ne!(row.exact_missing_evidence, "none");
    }
}

#[test]
fn a2_matrix_serialization_and_order_are_deterministic_and_have_no_score() {
    let first = serde_json::to_vec(&matrix()).expect("serialize matrix");
    let second = serde_json::to_vec(&matrix()).expect("serialize matrix twice");
    assert_eq!(first, second);
    let text = String::from_utf8(first).expect("utf8 json");
    assert!(!text.contains("backend_score"));
    assert!(!text.contains("global_score"));
    assert!(!text.contains("severity_score"));
}

#[test]
fn a2_new_backend_candidates_exist_only_for_observation_or_success_semantics_gaps() {
    for row in matrix() {
        if row.second_backend_could_plausibly_strengthen {
            assert!(matches!(
                row.primary_gap,
                GapClass::ObservationGap | GapClass::SuccessSemanticsGap
            ));
        }
    }
}
