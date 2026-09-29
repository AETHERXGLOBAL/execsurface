use std::collections::BTreeSet;

use execsurface_model::canonical::{
    CanonicalExecutable, CanonicalNetworkEndpoint, CanonicalPath, PathClass, PathResolution,
};
use execsurface_model::semantics_v3::{
    BackendSemanticProfile, CausalBinding, CompletenessDimension, CompletenessState,
    EvidenceGuarantees, IdentityBasis, ObservationPoint, ProofCarryingObservation,
    ProofRequirement, Proposition, TemporalBinding,
};
use execsurface_model::FileOperation;
use execsurface_p4_backend_authority::{
    compare_under_requirement, AdapterRecord, AuthorityState, ComparisonResult, EvidenceReference,
    PropositionCompleteness,
};

fn digest(byte: char) -> String {
    format!("sha256:{}", byte.to_string().repeat(64))
}

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

fn profile() -> BackendSemanticProfile {
    BackendSemanticProfile {
        name: "linux-ptrace-metadata-v2".to_owned(),
        semantic_profile_version: 1,
    }
}

fn attempt_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::UserspaceArgumentPreKernel]),
        identity_bases: BTreeSet::from([IdentityBasis::LexicalArgument]),
        temporal_bindings: BTreeSet::from([TemporalBinding::PreOperationIntent]),
        causal_bindings: BTreeSet::from([CausalBinding::DirectEvent]),
    }
}

fn success_object_guarantees() -> EvidenceGuarantees {
    EvidenceGuarantees {
        observation_points: BTreeSet::from([ObservationPoint::SyscallResultPostOperation]),
        identity_bases: BTreeSet::from([IdentityBasis::RuntimeFdPathCorrelated]),
        temporal_bindings: BTreeSet::from([TemporalBinding::SuccessfulOperationResult]),
        causal_bindings: BTreeSet::from([CausalBinding::StateMachineCorrelated]),
    }
}

fn attempt_requirement() -> ProofRequirement {
    ProofRequirement {
        guarantees: attempt_guarantees(),
        required_complete: BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Capability,
        ]),
    }
}

fn success_object_requirement() -> ProofRequirement {
    ProofRequirement {
        guarantees: success_object_guarantees(),
        required_complete: BTreeSet::from([
            CompletenessDimension::SessionScope,
            CompletenessDimension::Capability,
            CompletenessDimension::ObjectIdentity,
        ]),
    }
}

fn record(
    proposition_id: &str,
    proposition: Proposition,
    authority: AuthorityState,
    guarantees: EvidenceGuarantees,
    digest_char: char,
) -> AdapterRecord {
    let mut proof = ProofCarryingObservation::new(proposition, guarantees, profile());
    proof.completeness.insert(
        CompletenessDimension::SessionScope,
        CompletenessState::Complete,
    );
    proof.completeness.insert(
        CompletenessDimension::Capability,
        CompletenessState::Complete,
    );
    AdapterRecord {
        proposition_id: proposition_id.to_owned(),
        proof,
        authority,
        completeness: PropositionCompleteness::Complete,
        evidence: EvidenceReference {
            digest: digest(digest_char),
            derivation: None,
        },
        reason_codes: BTreeSet::new(),
    }
}

fn pathname_attempt_record() -> AdapterRecord {
    record(
        "P4.PATH.ACCESS_ATTEMPT",
        Proposition::FilePathnameAttemptObserved {
            actor: Some(exe("/usr/bin/cat")),
            execution_chain: vec![exe("/bin/bash"), exe("/usr/bin/cat")],
            operation: FileOperation::Open,
            target: path("$WORKSPACE/input.txt"),
            open_intent: None,
        },
        AuthorityState::AttemptOnly,
        attempt_guarantees(),
        'a',
    )
}

#[test]
fn a1_3u_exact_attempt_authority_is_usable_for_attempt_requirement() {
    let attempt = pathname_attempt_record();
    assert!(attempt.validate().is_ok());
    assert!(attempt.admissible_for(&attempt_requirement()));
}

#[test]
fn a1_3u_attempt_authority_cannot_satisfy_success_object_requirement() {
    let attempt = pathname_attempt_record();
    assert!(!attempt.admissible_for(&success_object_requirement()));
}

#[test]
fn a1_3u_attempt_authority_on_non_attempt_proposition_is_rejected() {
    let malformed = record(
        "P4.EXEC.SUCCESS",
        Proposition::ProcessExecSucceeded {
            from: Some(exe("/bin/bash")),
            executable: exe("/usr/bin/cat"),
        },
        AuthorityState::AttemptOnly,
        attempt_guarantees(),
        'b',
    );
    assert!(malformed.validate().is_err());
}

#[test]
fn a1_3u_rename_and_connect_attempts_do_not_gain_success_product_authority() {
    let rename = record(
        "P4.FILE.RENAME_DELETE",
        Proposition::FileRenameAttemptObserved {
            actor: Some(exe("/bin/mv")),
            execution_chain: vec![exe("/bin/bash"), exe("/bin/mv")],
            from: path("$WORKSPACE/from"),
            to: path("$WORKSPACE/to"),
        },
        AuthorityState::AttemptOnly,
        attempt_guarantees(),
        'c',
    );
    let connect = record(
        "P4.NET.CONNECT_DESTINATION",
        Proposition::NetworkConnectDestinationAttemptObserved {
            actor: Some(exe("/usr/bin/curl")),
            execution_chain: vec![exe("/bin/bash"), exe("/usr/bin/curl")],
            endpoint: CanonicalNetworkEndpoint::Inet {
                ip: "203.0.113.10".to_owned(),
                port: 443,
            },
        },
        AuthorityState::AttemptOnly,
        attempt_guarantees(),
        'd',
    );

    assert!(!rename.admissible_for(&attempt_requirement()));
    assert!(!connect.admissible_for(&attempt_requirement()));
}

#[test]
fn a1_3u_attempt_and_success_object_records_never_compare_as_equivalent() {
    let attempt = pathname_attempt_record();
    let mut success = record(
        "P4.FILE.OPEN_OBJECT",
        Proposition::FileOpenObjectObserved {
            actor: Some(exe("/usr/bin/cat")),
            execution_chain: vec![exe("/bin/bash"), exe("/usr/bin/cat")],
            target: path("$WORKSPACE/input.txt"),
        },
        AuthorityState::Direct,
        success_object_guarantees(),
        'e',
    );
    success.proof.completeness.insert(
        CompletenessDimension::ObjectIdentity,
        CompletenessState::Complete,
    );

    assert_eq!(
        compare_under_requirement(&attempt, &success, &attempt_requirement()),
        ComparisonResult::DifferentProposition
    );
}
