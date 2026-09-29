use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::AuthorityState;

pub const A2_PROPOSITION_IDS: [&str; 10] = [
    "P4.PROC.CREATE_RELATION",
    "P4.EXEC.SUCCESS",
    "P4.PATH.ACCESS_ATTEMPT",
    "P4.FILE.OPEN_OBJECT",
    "P4.FD.IO_ATTRIBUTION",
    "P4.FILE.RENAME_DELETE",
    "P4.NET.CONNECT_DESTINATION",
    "P4.CAUSAL.EXEC_LINEAGE",
    "P4.OBSERVER.HEALTH_LOSS",
    "P4.FDTABLE.RELATION",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapClass {
    NoGap,
    RepresentationGap,
    ObservationGap,
    SuccessSemanticsGap,
    IdentityGap,
    CompletenessGap,
    NoProductRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityGapRow {
    pub proposition_id: &'static str,
    pub raw_v2_sources: &'static [&'static str],
    pub mapped_shape: &'static str,
    pub best_authority: AuthorityState,
    pub conditional_authority: Option<&'static str>,
    pub satisfied_completeness: &'static [&'static str],
    pub unproven_completeness: &'static [&'static str],
    pub identity_basis: &'static str,
    pub temporal_binding: &'static str,
    pub causal_binding: &'static str,
    pub reason_codes: &'static [&'static str],
    pub product_usable: bool,
    pub primary_gap: GapClass,
    pub missing_evidence: &'static str,
    pub plausible_strengthening_source: &'static str,
    pub prior_evidence: &'static str,
}

pub fn ptrace_raw_v2_matrix() -> Vec<AuthorityGapRow> {
    vec![
        AuthorityGapRow {
            proposition_id: "P4.PROC.CREATE_RELATION",
            raw_v2_sources: &["process_spawn(child_tid,mechanism)", "raw_event.tid"],
            mapped_shape: "ProcessChildCreated(actor,mechanism)",
            best_authority: AuthorityState::Direct,
            conditional_authority: None,
            satisfied_completeness: &["session_scope", "lifecycle", "capability"],
            unproven_completeness: &["child_process_identity_in_canonical_authority_record"],
            identity_basis: "canonical parent executable; child_tid dropped before authority record",
            temporal_binding: "lifecycle_transition",
            causal_binding: "direct_event",
            reason_codes: &["raw_v2_child_tid_not_retained_in_canonical_proposition"],
            product_usable: false,
            primary_gap: GapClass::RepresentationGap,
            missing_evidence: "retain the observed parent/child process relation identity in the proposition-scoped record",
            plausible_strengthening_source: "ptrace raw-v3/process-identity retention; no new kernel backend implied",
            prior_evidence: "A0 process-create contract + raw-v2 ProcessSpawn + canonical ProcessSpawn shape",
        },
        AuthorityGapRow {
            proposition_id: "P4.EXEC.SUCCESS",
            raw_v2_sources: &["process_exec(path)", "raw_event.tid"],
            mapped_shape: "ProcessExecSucceeded(from,executable)",
            best_authority: AuthorityState::Direct,
            conditional_authority: None,
            satisfied_completeness: &["session_scope", "lifecycle", "capability"],
            unproven_completeness: &["process_identity_in_canonical_authority_record"],
            identity_basis: "trace-time resolved executable; raw tid not retained in mapped proposition",
            temporal_binding: "lifecycle_transition + pre_operation_intent",
            causal_binding: "state_machine_correlated",
            reason_codes: &["raw_v2_process_identity_not_retained_in_canonical_exec_proposition"],
            product_usable: false,
            primary_gap: GapClass::RepresentationGap,
            missing_evidence: "retain the successful exec transition's process identity and explicit lineage position in the authority record",
            plausible_strengthening_source: "ptrace raw-v3/process-identity retention; no new kernel backend implied",
            prior_evidence: "A0 exec-success contract + A1.2 confirmed-exec mapping PASS",
        },
        AuthorityGapRow {
            proposition_id: "P4.PATH.ACCESS_ATTEMPT",
            raw_v2_sources: &["file_path_access", "file_open_at2"],
            mapped_shape: "FilePathnameAttemptObserved",
            best_authority: AuthorityState::AttemptOnly,
            conditional_authority: None,
            satisfied_completeness: &["session_scope", "capability"],
            unproven_completeness: &[],
            identity_basis: "trace_time_dirfd_resolved_argument",
            temporal_binding: "pre_operation_intent",
            causal_binding: "direct_event",
            reason_codes: &[],
            product_usable: true,
            primary_gap: GapClass::NoGap,
            missing_evidence: "none for the frozen attempt proposition",
            plausible_strengthening_source: "none required for attempt authority",
            prior_evidence: "A1.3U 5/5 PASS + A1.2 pathname-attempt mapping",
        },
        AuthorityGapRow {
            proposition_id: "P4.FILE.OPEN_OBJECT",
            raw_v2_sources: &["file_path_access/open", "file_descriptor_access only after covered IO"],
            mapped_shape: "no FileOpenObjectObserved record emitted",
            best_authority: AuthorityState::Unsupported,
            conditional_authority: None,
            satisfied_completeness: &[],
            unproven_completeness: &["successful_result", "object_identity", "fd_table_relation"],
            identity_basis: "pathname attempt only at open; later fd identity is not an open-object record",
            temporal_binding: "successful_open_binding absent",
            causal_binding: "open-attempt event only",
            reason_codes: &["P4.FILE.OPEN_OBJECT", "successful_open_object_not_mapped_from_raw_v2"],
            product_usable: false,
            primary_gap: GapClass::ObservationGap,
            missing_evidence: "successful open result, returned FD identity, and post-open object/path binding at the open transition",
            plausible_strengthening_source: "targeted ptrace syscall-exit/fd-state evidence or kernel object/security-hook evidence",
            prior_evidence: "A1.2 open-path attempt never becomes successful-open object + A1.3 missing-open remains capability gap",
        },
        AuthorityGapRow {
            proposition_id: "P4.FD.IO_ATTRIBUTION",
            raw_v2_sources: &["file_descriptor_access(read/write,fd,path)", "process_spawn mechanism"],
            mapped_shape: "FileFdEffectObserved",
            best_authority: AuthorityState::Direct,
            conditional_authority: Some("direct+complete only when raw clone is absent; ambiguous/incomplete when clone is present"),
            satisfied_completeness: &["session_scope", "object_identity when no clone", "fd_table_relation when no clone"],
            unproven_completeness: &["fd_table_relation under raw-v2 clone"],
            identity_basis: "runtime_fd_path_correlated",
            temporal_binding: "successful_operation_result + post_operation_derived_state",
            causal_binding: "state_machine_correlated",
            reason_codes: &["raw_v2_clone_flags_not_retained"],
            product_usable: false,
            primary_gap: GapClass::RepresentationGap,
            missing_evidence: "retain causally paired clone/clone3 CLONE_FILES evidence so shared versus independent fd-table state is explicit",
            plausible_strengthening_source: "accepted internal ptrace clone/fd certificate projected into a future versioned evidence schema",
            prior_evidence: "C1/C6R shared-FD certificate evidence + A1.2 clone ambiguity test",
        },
        AuthorityGapRow {
            proposition_id: "P4.FILE.RENAME_DELETE",
            raw_v2_sources: &["file_rename(from,to)", "file_path_access(delete,path)"],
            mapped_shape: "FileRenameAttemptObserved / FilePathnameAttemptObserved",
            best_authority: AuthorityState::AttemptOnly,
            conditional_authority: None,
            satisfied_completeness: &["session_scope", "capability for attempt observation"],
            unproven_completeness: &["successful_effect_result", "successful target identity"],
            identity_basis: "trace_time_dirfd_resolved_argument",
            temporal_binding: "pre_operation_intent",
            causal_binding: "direct_event",
            reason_codes: &["successful_rename_delete_result_not_retained"],
            product_usable: false,
            primary_gap: GapClass::SuccessSemanticsGap,
            missing_evidence: "successful syscall/effect result bound to source/target identities",
            plausible_strengthening_source: "targeted ptrace syscall-exit evidence or kernel security-hook evidence",
            prior_evidence: "A0 success contract + A1.2/A1.3U attempt containment",
        },
        AuthorityGapRow {
            proposition_id: "P4.NET.CONNECT_DESTINATION",
            raw_v2_sources: &["network_connect_attempt(endpoint)"],
            mapped_shape: "NetworkConnectDestinationAttemptObserved",
            best_authority: AuthorityState::AttemptOnly,
            conditional_authority: None,
            satisfied_completeness: &["session_scope", "capability for attempt observation"],
            unproven_completeness: &["connect success/result semantics", "post-call socket state"],
            identity_basis: "socket_address_argument",
            temporal_binding: "pre_operation_intent",
            causal_binding: "direct_event",
            reason_codes: &["successful_connect_result_not_retained"],
            product_usable: false,
            primary_gap: GapClass::SuccessSemanticsGap,
            missing_evidence: "connect result semantics, including bounded handling of asynchronous connect, bound to socket/destination identity",
            plausible_strengthening_source: "targeted ptrace syscall-exit/socket-state evidence or kernel connect/security-hook evidence",
            prior_evidence: "A0 successful-connect contract + A1.2/A1.3U attempt containment",
        },
        AuthorityGapRow {
            proposition_id: "P4.CAUSAL.EXEC_LINEAGE",
            raw_v2_sources: &["ordered spawn/exec raw events", "canonical execution_chain"],
            mapped_shape: "CausalExecLineageObserved",
            best_authority: AuthorityState::DerivedBounded,
            conditional_authority: Some("bounded to the accepted canonical-exec-lineage-v2 derivation and complete observation"),
            satisfied_completeness: &["session_scope", "causal_lineage"],
            unproven_completeness: &[],
            identity_basis: "canonical executable lineage",
            temporal_binding: "derived ordered lifecycle",
            causal_binding: "lineage_derived",
            reason_codes: &[],
            product_usable: true,
            primary_gap: GapClass::NoGap,
            missing_evidence: "none within the accepted bounded lineage derivation",
            plausible_strengthening_source: "none required for the frozen bounded proposition",
            prior_evidence: "A1.2 named derivation + A1.3 causal-chain and actor-substitution falsification",
        },
        AuthorityGapRow {
            proposition_id: "P4.OBSERVER.HEALTH_LOSS",
            raw_v2_sources: &["observation.complete", "observer warnings"],
            mapped_shape: "ObserverHealthObserved",
            best_authority: AuthorityState::Direct,
            conditional_authority: Some("healthy observation maps direct+complete; detected loss maps lost+incomplete"),
            satisfied_completeness: &["session_scope for declared/detected health state"],
            unproven_completeness: &[],
            identity_basis: "none",
            temporal_binding: "session health state",
            causal_binding: "direct evidence state",
            reason_codes: &["warning codes preserved when present", "observation_incomplete when complete=false"],
            product_usable: true,
            primary_gap: GapClass::NoGap,
            missing_evidence: "none for declared/detected health signals; no universal silent-loss claim",
            plausible_strengthening_source: "none required by the frozen bounded health proposition",
            prior_evidence: "A1.2 incomplete/warning fail-closed tests + A1.3 observer-loss attack",
        },
        AuthorityGapRow {
            proposition_id: "P4.FDTABLE.RELATION",
            raw_v2_sources: &["process_spawn mechanism"],
            mapped_shape: "FdTableRelationObserved",
            best_authority: AuthorityState::DerivedBounded,
            conditional_authority: Some("fork/vfork -> independent_copy derived_bounded; clone -> unknown ambiguous/incomplete"),
            satisfied_completeness: &["fd_table_relation for fork/vfork bounded derivation"],
            unproven_completeness: &["fd_table_relation for clone"],
            identity_basis: "spawn mechanism; clone flags missing from raw-v2",
            temporal_binding: "lifecycle_transition",
            causal_binding: "direct event plus bounded fork/vfork derivation",
            reason_codes: &["raw_v2_clone_flags_not_retained", "P4.FDTABLE.RELATION.EXACT_CLONE"],
            product_usable: false,
            primary_gap: GapClass::RepresentationGap,
            missing_evidence: "causally paired clone/clone3 CLONE_FILES flags retained in versioned evidence",
            plausible_strengthening_source: "accepted internal ptrace clone/fd certificate projected into a future versioned evidence schema",
            prior_evidence: "C1 census/certificate + A1.2 unknown-clone relation + A1.3 dependent-FD falsification",
        },
    ]
}

pub fn validate_matrix(rows: &[AuthorityGapRow]) -> Result<(), String> {
    if rows.len() != A2_PROPOSITION_IDS.len() {
        return Err(format!(
            "expected {} rows, got {}",
            A2_PROPOSITION_IDS.len(),
            rows.len()
        ));
    }
    let expected = A2_PROPOSITION_IDS.into_iter().collect::<BTreeSet<_>>();
    let actual = rows
        .iter()
        .map(|row| row.proposition_id)
        .collect::<BTreeSet<_>>();
    if actual != expected {
        return Err("matrix proposition set does not equal frozen A0 inventory".to_owned());
    }
    if actual.len() != rows.len() {
        return Err("matrix contains duplicate proposition rows".to_owned());
    }
    for row in rows {
        if row.missing_evidence.trim().is_empty()
            || row.plausible_strengthening_source.trim().is_empty()
            || row.prior_evidence.trim().is_empty()
        {
            return Err(format!("incomplete A2 metadata for {}", row.proposition_id));
        }
        if row.primary_gap == GapClass::NoGap && !row.product_usable {
            return Err(format!("NO_GAP row is not product-usable: {}", row.proposition_id));
        }
    }
    Ok(())
}

pub fn deterministic_json(rows: &[AuthorityGapRow]) -> Result<Vec<u8>, serde_json::Error> {
    serde_json::to_vec_pretty(rows)
}
