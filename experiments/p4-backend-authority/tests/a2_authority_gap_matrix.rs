pub use execsurface_p4_backend_authority::AuthorityState;

#[path = "../src/a2_matrix.rs"]
mod a2_matrix;

use a2_matrix::{deterministic_json, ptrace_raw_v2_matrix, validate_matrix, GapClass, A2_PROPOSITION_IDS};

fn row(id: &str) -> a2_matrix::AuthorityGapRow {
    ptrace_raw_v2_matrix()
        .into_iter()
        .find(|row| row.proposition_id == id)
        .unwrap_or_else(|| panic!("missing matrix row {id}"))
}

#[test]
fn a2_matrix_exactly_matches_frozen_ten_propositions_in_order() {
    let rows = ptrace_raw_v2_matrix();
    validate_matrix(&rows).expect("valid frozen matrix");
    let ids = rows.iter().map(|row| row.proposition_id).collect::<Vec<_>>();
    assert_eq!(ids, A2_PROPOSITION_IDS);
}

#[test]
fn a2_process_create_retains_child_identity_as_representation_gap() {
    let row = row("P4.PROC.CREATE_RELATION");
    assert_eq!(row.best_authority, AuthorityState::Direct);
    assert_eq!(row.primary_gap, GapClass::RepresentationGap);
    assert!(!row.product_usable);
    assert!(row
        .reason_codes
        .contains(&"raw_v2_child_tid_not_retained_in_canonical_proposition"));
}

#[test]
fn a2_exec_success_does_not_hide_missing_process_identity() {
    let row = row("P4.EXEC.SUCCESS");
    assert_eq!(row.best_authority, AuthorityState::Direct);
    assert_eq!(row.primary_gap, GapClass::RepresentationGap);
    assert!(!row.product_usable);
}

#[test]
fn a2_explicit_path_attempt_is_the_only_attempt_product_no_gap() {
    let row = row("P4.PATH.ACCESS_ATTEMPT");
    assert_eq!(row.best_authority, AuthorityState::AttemptOnly);
    assert_eq!(row.primary_gap, GapClass::NoGap);
    assert!(row.product_usable);
}

#[test]
fn a2_successful_open_object_remains_an_explicit_gap() {
    let row = row("P4.FILE.OPEN_OBJECT");
    assert_eq!(row.best_authority, AuthorityState::Unsupported);
    assert_eq!(row.primary_gap, GapClass::ObservationGap);
    assert!(!row.product_usable);
    assert!(row.unproven_completeness.contains(&"object_identity"));
}

#[test]
fn a2_fd_io_is_not_globally_direct_when_clone_relation_is_unknown() {
    let row = row("P4.FD.IO_ATTRIBUTION");
    assert_eq!(row.best_authority, AuthorityState::Direct);
    assert_eq!(row.primary_gap, GapClass::RepresentationGap);
    assert!(!row.product_usable);
    assert!(row.reason_codes.contains(&"raw_v2_clone_flags_not_retained"));
    assert!(row.conditional_authority.is_some());
}

#[test]
fn a2_rename_delete_attempt_is_not_success_authority() {
    let row = row("P4.FILE.RENAME_DELETE");
    assert_eq!(row.best_authority, AuthorityState::AttemptOnly);
    assert_eq!(row.primary_gap, GapClass::SuccessSemanticsGap);
    assert!(!row.product_usable);
}

#[test]
fn a2_network_connect_attempt_is_not_success_authority() {
    let row = row("P4.NET.CONNECT_DESTINATION");
    assert_eq!(row.best_authority, AuthorityState::AttemptOnly);
    assert_eq!(row.primary_gap, GapClass::SuccessSemanticsGap);
    assert!(!row.product_usable);
}

#[test]
fn a2_causal_lineage_remains_bounded_derivation_not_direct() {
    let row = row("P4.CAUSAL.EXEC_LINEAGE");
    assert_eq!(row.best_authority, AuthorityState::DerivedBounded);
    assert_eq!(row.primary_gap, GapClass::NoGap);
    assert!(row.product_usable);
}

#[test]
fn a2_observer_health_is_bounded_no_gap_not_universal_completeness() {
    let row = row("P4.OBSERVER.HEALTH_LOSS");
    assert_eq!(row.best_authority, AuthorityState::Direct);
    assert_eq!(row.primary_gap, GapClass::NoGap);
    assert!(row.product_usable);
    assert!(row.missing_evidence.contains("no universal silent-loss claim"));
}

#[test]
fn a2_clone_fd_table_relation_remains_representation_gap() {
    let row = row("P4.FDTABLE.RELATION");
    assert_eq!(row.best_authority, AuthorityState::DerivedBounded);
    assert_eq!(row.primary_gap, GapClass::RepresentationGap);
    assert!(!row.product_usable);
    assert!(row.reason_codes.contains(&"raw_v2_clone_flags_not_retained"));
}

#[test]
fn a2_gap_rows_name_missing_evidence_and_strengthening_source() {
    for row in ptrace_raw_v2_matrix() {
        if row.primary_gap != GapClass::NoGap {
            assert!(!row.missing_evidence.trim().is_empty(), "{}", row.proposition_id);
            assert!(
                !row.plausible_strengthening_source.trim().is_empty(),
                "{}",
                row.proposition_id
            );
            assert!(!row.prior_evidence.trim().is_empty(), "{}", row.proposition_id);
        }
    }
}

#[test]
fn a2_matrix_serialization_is_deterministic_and_gap_preserving() {
    let first = ptrace_raw_v2_matrix();
    let second = ptrace_raw_v2_matrix();
    assert_eq!(
        deterministic_json(&first).expect("serialize first"),
        deterministic_json(&second).expect("serialize second")
    );
    assert_eq!(
        first
            .iter()
            .filter(|row| row.primary_gap == GapClass::NoGap)
            .count(),
        3
    );
    assert_eq!(first.iter().filter(|row| !row.product_usable).count(), 7);
}

#[test]
fn a2_emit_canonical_matrix_for_ci_evidence() {
    let rows = ptrace_raw_v2_matrix();
    validate_matrix(&rows).expect("valid matrix");
    let json = String::from_utf8(deterministic_json(&rows).expect("serialize matrix"))
        .expect("utf8 matrix");
    println!("A2_MATRIX_JSON_BEGIN\n{json}\nA2_MATRIX_JSON_END");
}
