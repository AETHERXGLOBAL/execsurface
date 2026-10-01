#![cfg(feature = "semantics-v3")]

use execsurface_model::canonical::{CanonicalPath, PathClass, PathResolution};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation,
    ProofRequirement, Proposition, TemporalBinding, SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION,
};
use execsurface_model::{BackendMetadata, FileOperation, Observation};
use std::collections::BTreeSet;

fn target_path() -> CanonicalPath {
    CanonicalPath {
        value: "$WORKSPACE/input.txt".to_owned(),
        class: PathClass::Workspace,
        resolution: PathResolution::Lexical,
    }
}
fn proposition() -> Proposition {
    Proposition::FilePathnameAttemptObserved {
        actor: None,
        execution_chain: vec![],
        operation: FileOperation::Open,
        target: target_path(),
        open_intent: None,
    }
}
fn guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
        identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
        temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}
fn record() -> ProofCarryingObservation {
    let mut r = ProofCarryingObservation::new(
        proposition(),
        guarantees(),
        BackendSemanticProfile {
            name: "linux-ptrace-semantics-v3-candidate".into(),
            semantic_profile_version: 1,
        },
    );
    r.completeness.insert(
        CompletenessDimension::SessionScope,
        CompletenessState::Complete,
    );
    r.completeness.insert(
        CompletenessDimension::Lifecycle,
        CompletenessState::Complete,
    );
    r
}
fn requirement(g: EvidenceGuarantees, c: BTreeSet<CompletenessDimension>) -> ProofRequirement {
    ProofRequirement::for_proposition(proposition(), g, c)
}

#[test]
fn a1_01_v2_shaped_payload_is_not_a_v3_proof_record() {
    let v2 = Observation::empty(BackendMetadata {
        name: "linux-ptrace".into(),
        platform: "linux".into(),
        architecture: "x86_64".into(),
        capabilities: vec!["process".into()],
        limitations: vec![],
    });
    let bytes = serde_json::to_vec(&v2).unwrap();
    assert!(serde_json::from_slice::<ProofCarryingObservation>(&bytes).is_err());
}
#[test]
fn a1_02_schema_version_two_cannot_satisfy_v3_requirement() {
    let mut r = record();
    r.schema_version = 2;
    let req = requirement(
        guarantees(),
        BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Lifecycle,
        ]),
    );
    assert!(!r.satisfies(&req));
    assert_eq!(SEMANTICS_V3_PROTOTYPE_SCHEMA_VERSION, 3);
}
#[test]
fn a1_03_missing_required_completeness_is_non_admissible() {
    assert!(!record().satisfies(&requirement(
        guarantees(),
        BTreeSet::from([CompletenessDimension::ObjectIdentity])
    )));
}
#[test]
fn a1_04_incomplete_required_completeness_is_non_admissible() {
    let mut r = record();
    r.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Incomplete {
            reason_code: "missing".into(),
        },
    );
    assert!(!r.satisfies(&requirement(
        guarantees(),
        BTreeSet::from([CompletenessDimension::ObjectIdentity])
    )));
}
#[test]
fn a1_05_ambiguous_required_completeness_is_non_admissible() {
    let mut r = record();
    r.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Ambiguous {
            reason_code: "ambiguous".into(),
        },
    );
    assert!(!r.satisfies(&requirement(
        guarantees(),
        BTreeSet::from([CompletenessDimension::ObjectIdentity])
    )));
}
#[test]
fn a1_06_unsupported_required_completeness_is_non_admissible() {
    let mut r = record();
    r.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Unsupported {
            reason_code: "unsupported".into(),
        },
    );
    assert!(!r.satisfies(&requirement(
        guarantees(),
        BTreeSet::from([CompletenessDimension::ObjectIdentity])
    )));
}
#[test]
fn a1_07_backend_name_cannot_upgrade_weak_authority() {
    let mut r = record();
    r.backend_profile.name = "kernel-super-authoritative-trusted-backend".into();
    let req = requirement(
        EvidenceGuarantees {
            identity_bases: BTreeSet::from([IdentityBasis::KernelObjectGrounded]),
            ..EvidenceGuarantees::default()
        },
        BTreeSet::new(),
    );
    assert!(!r.satisfies(&req));
}
#[test]
fn a1_08_same_behavior_with_different_proof_authority_is_distinct() {
    let a = record();
    let mut b = record();
    b.guarantees
        .identity_bases
        .insert(IdentityBasis::KernelObjectGrounded);
    assert_eq!(a.proposition, b.proposition);
    assert_ne!(a, b);
}
#[test]
fn a1_09_prototype_serialization_is_deterministic_for_set_order() {
    let mut a = record();
    a.ambiguity_codes.insert("zeta".into());
    a.ambiguity_codes.insert("alpha".into());
    let mut b = record();
    b.ambiguity_codes.insert("alpha".into());
    b.ambiguity_codes.insert("zeta".into());
    assert_eq!(
        serde_json::to_vec(&a).unwrap(),
        serde_json::to_vec(&b).unwrap()
    );
}
