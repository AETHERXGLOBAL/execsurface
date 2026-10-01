#![cfg(feature = "semantics-v3")]

use std::collections::BTreeSet;

use execsurface_model::canonical::{CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation,
    ProofRequirement, Proposition, TemporalBinding,
};
use execsurface_model::FileOperation;

fn target_path(value: &str) -> CanonicalPath {
    CanonicalPath {
        value: value.to_owned(),
        class: PathClass::Workspace,
        resolution: PathResolution::Lexical,
    }
}

fn pathname_proposition(value: &str) -> Proposition {
    Proposition::FilePathnameAttemptObserved {
        actor: None,
        execution_chain: vec![],
        operation: FileOperation::Open,
        target: target_path(value),
        open_intent: None,
    }
}

fn weak_path_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
        identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
        temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}

fn record_for(value: &str) -> ProofCarryingObservation {
    let mut record = ProofCarryingObservation::new(
        pathname_proposition(value),
        weak_path_guarantees(),
        BackendSemanticProfile {
            name: "linux-ptrace-semantics-v3-candidate".to_owned(),
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

fn pathname_requirement(value: &str) -> ProofRequirement {
    ProofRequirement::for_proposition(
        pathname_proposition(value),
        weak_path_guarantees(),
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Lifecycle,
        ]),
    )
}

#[test]
fn d01_wrong_proposition_must_not_satisfy_bound_contract() {
    let expected = record_for("$WORKSPACE/input.txt");
    let substituted = record_for("$WORKSPACE/other.txt");
    let requirement = pathname_requirement("$WORKSPACE/input.txt");
    assert!(expected.satisfies(&requirement));
    assert_ne!(expected.proposition, substituted.proposition);
    assert!(!substituted.satisfies(&requirement));
}

#[test]
fn d02_invalidating_ambiguity_must_block_claimed_complete_requirement() {
    let mut record = record_for("$WORKSPACE/input.txt");
    record.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Complete,
    );
    record
        .ambiguity_codes
        .insert("object_identity_conflict".to_owned());
    let requirement = ProofRequirement::for_proposition(
        pathname_proposition("$WORKSPACE/input.txt"),
        weak_path_guarantees(),
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Lifecycle,
            CompletenessDimension::ObjectIdentity,
        ]),
    );
    assert!(!record.satisfies(&requirement));
}

#[test]
fn d03_empty_requirement_must_not_be_sufficient_semantic_proof() {
    let record = record_for("$WORKSPACE/input.txt");
    assert!(!record.satisfies(&ProofRequirement::default()));
}

#[test]
fn d04_bound_but_obligation_free_requirement_must_fail_closed() {
    let record = record_for("$WORKSPACE/input.txt");
    let requirement = ProofRequirement::for_proposition(
        pathname_proposition("$WORKSPACE/input.txt"),
        EvidenceGuarantees::default(),
        BTreeSet::new(),
    );
    assert!(!record.satisfies(&requirement));
}

#[test]
fn d05_unknown_ambiguity_code_must_fail_closed() {
    let mut record = record_for("$WORKSPACE/input.txt");
    record
        .ambiguity_codes
        .insert("future_unclassified_ambiguity".to_owned());
    let requirement = pathname_requirement("$WORKSPACE/input.txt");
    assert!(!record.satisfies(&requirement));
}

#[test]
fn d06_valid_bound_control_remains_admissible() {
    let record = record_for("$WORKSPACE/input.txt");
    let requirement = pathname_requirement("$WORKSPACE/input.txt");
    assert!(record.satisfies(&requirement));
}
