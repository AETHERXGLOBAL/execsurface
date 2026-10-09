#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::Path;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
}

fn read(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap_or_else(|e| panic!("cannot read {path}: {e}"))
}

#[test]
fn external_evidence_is_waived_not_falsely_marked_pass() {
    let readiness = read("docs/PRODUCTION_READINESS.md");
    let governance = read("docs/development/V1_INTERNAL_QUALIFICATION_GOVERNANCE.md");

    assert!(
        readiness.contains("WAIVED AS RELEASE BLOCKER / EVIDENCE STILL INCOMPLETE"),
        "P9 must distinguish waiver from evidence PASS"
    );
    assert!(
        governance.contains("does **not** convert missing external evidence into PASS"),
        "governance must explicitly reject false PASS relabeling"
    );
    assert!(
        governance.contains("external independent validation is not claimed"),
        "future v1 claim boundary must remain explicit"
    );
}

#[test]
fn strengthened_internal_chain_and_role_separation_are_mandatory() {
    let governance = read("docs/development/V1_INTERNAL_QUALIFICATION_GOVERNANCE.md");

    for gate in [
        "IQ0", "IQ1", "IQ2", "IQ3", "IQ4", "IQ5", "IQ6", "IQ7", "IQ8", "IQ9",
    ] {
        assert!(
            governance.contains(gate),
            "strengthened internal gate {gate} must remain present"
        );
    }

    assert!(
        governance.contains("Team F — Independent Internal Critical Review Board"),
        "final release decision must remain separated from implementation"
    );
    assert!(
        governance.contains("must not author the candidate fix being reviewed"),
        "critical reviewers must remain separated from candidate implementation"
    );
    assert!(
        governance.contains("any valid counterexample invalidates the RC SHA"),
        "falsification must remain fail-first"
    );
}

#[test]
fn public_claim_boundary_cannot_inflate_internal_review_into_external_validation() {
    let governance = read("docs/development/V1_INTERNAL_QUALIFICATION_GOVERNANCE.md");

    for forbidden in [
        "independently validated;",
        "externally certified;",
        "proven in production;",
        "industry validated;",
        "universally production-ready;",
    ] {
        assert!(
            governance.contains(forbidden),
            "anti-marketing rule must retain forbidden phrase {forbidden}"
        );
    }

    assert!(
        governance.contains("internally qualified under AETHER X GLOBAL's published compatibility, adversarial, artifact, rollback and productization gates"),
        "allowed bounded v1 wording must remain explicit"
    );
}
