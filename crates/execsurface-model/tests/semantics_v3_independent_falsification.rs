use std::collections::{BTreeMap, BTreeSet};

use execsurface_model::canonical::{
    CanonicalExecutable, CanonicalPath, OpenIntent, PathClass, PathResolution,
};
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

fn exe(value: &str) -> CanonicalExecutable {
    CanonicalExecutable {
        path: CanonicalPath {
            value: value.to_owned(),
            class: PathClass::System,
            resolution: PathResolution::Lexical,
        },
        family: value.rsplit('/').next().unwrap_or(value).to_owned(),
    }
}

fn proposition(actor: &str, operation: FileOperation, chain_tail: &str) -> Proposition {
    Proposition::FilePathnameAttemptObserved {
        actor: Some(exe(actor)),
        execution_chain: vec![exe("/bin/bash"), exe(chain_tail)],
        operation,
        target: path("$WORKSPACE/input.txt"),
        open_intent: Some(OpenIntent::ReadOnly),
    }
}

fn base_proposition() -> Proposition {
    proposition("/usr/bin/cat", FileOperation::Open, "/usr/bin/cat")
}

fn guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
        identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
        temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}

fn strong_guarantees() -> EvidenceGuarantees {
    let mut g = guarantees();
    g.observation_points
        .insert(ObservationPoint::SyscallResultPostOperation);
    g.temporal_bindings
        .insert(TemporalBinding::SuccessfulOperationResult);
    g
}

fn record_for(p: Proposition) -> ProofCarryingObservation {
    let mut record = ProofCarryingObservation::new(
        p,
        guarantees(),
        BackendSemanticProfile {
            name: "independent-review-ptrace".to_owned(),
            semantic_profile_version: 1,
        },
    );
    record.completeness = BTreeMap::from([
        (
            CompletenessDimension::SessionScope,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::Lifecycle,
            CompletenessState::Complete,
        ),
        (
            CompletenessDimension::ObjectIdentity,
            CompletenessState::Complete,
        ),
    ]);
    record
}

fn requirement_for(p: Proposition) -> ProofRequirement {
    ProofRequirement::for_proposition(
        p,
        guarantees(),
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Lifecycle,
        ]),
    )
}

#[test]
fn f01_deserialized_requirement_without_expected_proposition_fails_closed() {
    let json = r#"{
        "guarantees": {
            "observation_points": ["userspace_argument_pre_kernel"],
            "identity_bases": ["lexical_argument"],
            "temporal_bindings": ["pre_operation_intent"],
            "causal_bindings": ["direct_event"]
        },
        "required_complete": ["session_scope"]
    }"#;
    let requirement: ProofRequirement =
        serde_json::from_str(json).expect("parse legacy-shaped requirement");
    assert!(requirement.expected_proposition.is_none());
    assert!(!record_for(base_proposition()).satisfies(&requirement));
}

#[test]
fn f02_actor_substitution_fails_bound_contract() {
    let expected = base_proposition();
    let substituted = proposition("/usr/bin/python3", FileOperation::Open, "/usr/bin/cat");
    let requirement = requirement_for(expected.clone());
    assert!(record_for(expected).satisfies(&requirement));
    assert!(!record_for(substituted).satisfies(&requirement));
}

#[test]
fn f03_operation_substitution_fails_bound_contract() {
    let expected = base_proposition();
    let substituted = proposition("/usr/bin/cat", FileOperation::Read, "/usr/bin/cat");
    let requirement = requirement_for(expected.clone());
    assert!(record_for(expected).satisfies(&requirement));
    assert!(!record_for(substituted).satisfies(&requirement));
}

#[test]
fn f04_execution_chain_substitution_fails_bound_contract() {
    let expected = base_proposition();
    let substituted = proposition("/usr/bin/cat", FileOperation::Open, "/usr/bin/python3");
    let requirement = requirement_for(expected.clone());
    assert!(record_for(expected).satisfies(&requirement));
    assert!(!record_for(substituted).satisfies(&requirement));
}

#[test]
fn f05_future_schema_substitution_fails_closed() {
    let p = base_proposition();
    let mut record = record_for(p.clone());
    record.schema_version = 4;
    assert!(!record.satisfies(&requirement_for(p)));
}

#[test]
fn f06_required_completeness_downgrades_fail_closed() {
    let p = base_proposition();
    let requirement = ProofRequirement::for_proposition(
        p.clone(),
        guarantees(),
        BTreeSet::from([CompletenessDimension::ObjectIdentity]),
    );

    for state in [
        CompletenessState::NotRequired,
        CompletenessState::Incomplete {
            reason_code: "forced_incomplete".to_owned(),
        },
        CompletenessState::Ambiguous {
            reason_code: "forced_ambiguous".to_owned(),
        },
        CompletenessState::Unsupported {
            reason_code: "forced_unsupported".to_owned(),
        },
    ] {
        let mut record = record_for(p.clone());
        record
            .completeness
            .insert(CompletenessDimension::ObjectIdentity, state);
        assert!(!record.satisfies(&requirement));
    }
}

#[test]
fn f07_backend_profile_mutation_cannot_rescue_proposition_mismatch() {
    let expected = base_proposition();
    let requirement = requirement_for(expected.clone());
    let substituted = proposition("/usr/bin/python3", FileOperation::Open, "/usr/bin/python3");
    let mut record = record_for(substituted);
    record.backend_profile.name = "trusted-kernel-perfect-authority".to_owned();
    record.backend_profile.semantic_profile_version = u32::MAX;
    assert!(!record.satisfies(&requirement));
}

#[test]
fn f08_bound_completeness_only_contract_is_non_vacuous_and_valid() {
    let p = base_proposition();
    let record = record_for(p.clone());
    let requirement = ProofRequirement::for_proposition(
        p,
        EvidenceGuarantees::default(),
        BTreeSet::from([CompletenessDimension::SessionScope]),
    );
    assert!(record.satisfies(&requirement));
}

#[test]
fn f09_object_identity_conflict_blocks_object_identity_requirement() {
    let p = base_proposition();
    let mut record = record_for(p.clone());
    record
        .ambiguity_codes
        .insert("object_identity_conflict".to_owned());
    let requirement = ProofRequirement::for_proposition(
        p,
        guarantees(),
        BTreeSet::from([CompletenessDimension::ObjectIdentity]),
    );
    assert!(!record.satisfies(&requirement));
}

#[test]
fn f10_object_identity_conflict_does_not_poison_unrelated_requirement() {
    let p = base_proposition();
    let mut record = record_for(p.clone());
    record
        .ambiguity_codes
        .insert("object_identity_conflict".to_owned());
    let requirement = ProofRequirement::for_proposition(
        p,
        guarantees(),
        BTreeSet::from([CompletenessDimension::SessionScope]),
    );
    assert!(record.satisfies(&requirement));
}

#[test]
fn f11_unknown_ambiguity_code_fails_closed() {
    let p = base_proposition();
    let mut record = record_for(p.clone());
    record
        .ambiguity_codes
        .insert("future_unknown_ambiguity".to_owned());
    assert!(!record.satisfies(&requirement_for(p)));
}

#[test]
fn f12_stronger_guarantees_do_not_rescue_wrong_proposition() {
    let expected = base_proposition();
    let requirement = requirement_for(expected);
    let substituted = proposition("/usr/bin/python3", FileOperation::Open, "/usr/bin/python3");
    let mut record = ProofCarryingObservation::new(
        substituted,
        strong_guarantees(),
        BackendSemanticProfile {
            name: "stronger-evidence-review-profile".to_owned(),
            semantic_profile_version: 2,
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
    assert!(!record.satisfies(&requirement));
}
