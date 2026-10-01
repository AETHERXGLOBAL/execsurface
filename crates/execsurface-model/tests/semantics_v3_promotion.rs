use std::collections::BTreeSet;

use execsurface_model::canonical::{CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation, ProofRequirement,
    Proposition, TemporalBinding, SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION,
};
use execsurface_model::{BackendMetadata, FileOperation, Observation};

fn target_path() -> CanonicalPath {
    CanonicalPath {
        value: "$WORKSPACE/input.txt".to_owned(),
        class: PathClass::Workspace,
        resolution: PathResolution::Lexical,
    }
}

fn pathname_proposition() -> Proposition {
    Proposition::FilePathnameAttemptObserved {
        actor: None,
        execution_chain: vec![],
        operation: FileOperation::Open,
        target: target_path(),
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

fn base_record() -> ProofCarryingObservation {
    let mut record = ProofCarryingObservation::new(
        pathname_proposition(),
        weak_path_guarantees(),
        BackendSemanticProfile {
            name: "linux-ptrace-semantics-v3-prototype".to_owned(),
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

fn weak_requirement_with(required: CompletenessDimension) -> ProofRequirement {
    ProofRequirement {
        guarantees: weak_path_guarantees(),
        required_complete: BTreeSet::from([required]),
    }
}

#[test]
fn a1_01_v2_shaped_payload_is_not_a_v3_proof_record() {
    let v2 = Observation::empty(BackendMetadata {
        name: "linux-ptrace".to_owned(),
        platform: "linux".to_owned(),
        architecture: "x86_64".to_owned(),
        capabilities: vec!["process".to_owned()],
        limitations: vec![],
    });
    let bytes = serde_json::to_vec(&v2).expect("serialize v2 observation");
    let parsed = serde_json::from_slice::<ProofCarryingObservation>(&bytes);
    assert!(parsed.is_err(), "v2 observation must not parse as a v3 proof record");
}

#[test]
fn a1_02_schema_version_two_cannot_satisfy_v3_requirement() {
    let mut record = base_record();
    record.schema_version = 2;
    let requirement = ProofRequirement {
        guarantees: weak_path_guarantees(),
        required_complete: BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Lifecycle,
        ]),
    };
    assert!(!record.satisfies(&requirement));
    assert_eq!(SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION, 3);
}

#[test]
fn a1_03_missing_required_completeness_is_non_admissible() {
    let record = base_record();
    let requirement = weak_requirement_with(CompletenessDimension::ObjectIdentity);
    assert!(!record.satisfies(&requirement));
}

#[test]
fn a1_04_incomplete_required_completeness_is_non_admissible() {
    let mut record = base_record();
    record.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Incomplete {
            reason_code: "test_missing_object_evidence".to_owned(),
        },
    );
    let requirement = weak_requirement_with(CompletenessDimension::ObjectIdentity);
    assert!(!record.satisfies(&requirement));
}

#[test]
fn a1_05_ambiguous_required_completeness_is_non_admissible() {
    let mut record = base_record();
    record.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Ambiguous {
            reason_code: "test_object_identity_ambiguity".to_owned(),
        },
    );
    let requirement = weak_requirement_with(CompletenessDimension::ObjectIdentity);
    assert!(!record.satisfies(&requirement));
}

#[test]
fn a1_06_unsupported_required_completeness_is_non_admissible() {
    let mut record = base_record();
    record.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Unsupported {
            reason_code: "test_backend_unsupported".to_owned(),
        },
    );
    let requirement = weak_requirement_with(CompletenessDimension::ObjectIdentity);
    assert!(!record.satisfies(&requirement));
}

#[test]
fn a1_07_backend_name_cannot_upgrade_weak_authority() {
    let mut record = base_record();
    record.backend_profile.name = "kernel-super-authoritative-trusted-backend".to_owned();
    let requirement = ProofRequirement {
        guarantees: EvidenceGuarantees {
            identity_bases: BTreeSet::from([IdentityBasis::KernelObjectGrounded]),
            ..EvidenceGuarantees::default()
        },
        required_complete: BTreeSet::new(),
    };
    assert!(!record.satisfies(&requirement));
}

#[test]
fn a1_08_same_behavior_with_different_proof_authority_is_distinct() {
    let first = base_record();
    let mut second = base_record();
    second
        .guarantees
        .identity_bases
        .insert(IdentityBasis::KernelObjectGrounded);
    assert_eq!(first.proposition, second.proposition);
    assert_ne!(first, second);
}

#[test]
fn a1_09_prototype_serialization_is_deterministic_for_set_order() {
    let mut first = base_record();
    first.ambiguity_codes.insert("zeta".to_owned());
    first.ambiguity_codes.insert("alpha".to_owned());

    let mut second = base_record();
    second.ambiguity_codes.insert("alpha".to_owned());
    second.ambiguity_codes.insert("zeta".to_owned());

    assert_eq!(
        serde_json::to_vec(&first).expect("serialize first"),
        serde_json::to_vec(&second).expect("serialize second")
    );
}
