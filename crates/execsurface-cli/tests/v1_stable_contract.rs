#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::fs;
use std::path::Path;

fn repo_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root")
}

fn read(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).unwrap_or_else(|error| {
        panic!("cannot read {path}: {error}");
    })
}

#[test]
fn stable_contract_freezes_target_outcome_and_policy_v3_without_semantic_expansion() {
    let compatibility = read("docs/COMPATIBILITY.md");

    assert!(
        compatibility.contains("V1_STABLE_CONTRACT_ACTIVE_BOUNDED"),
        "compatibility document must declare the active bounded stable-v1 contract"
    );
    assert!(
        compatibility.contains("target outcome is not verdict-bearing")
            && compatibility.contains("PASS does not mean the wrapped target command succeeded"),
        "v1 must explicitly freeze target outcome as separate from the ExecSurface verdict"
    );
    assert!(
        compatibility.contains("schema 2 remains the default authored policy schema")
            && compatibility.contains("schema 3 is a stable opt-in extension"),
        "v1 must explicitly freeze policy-v2 default behavior and policy-v3 opt-in stability"
    );
    for matcher in [
        "path_resolution",
        "open_intent",
        "rename_from_class",
        "rename_from_prefix",
        "rename_from_resolution",
    ] {
        assert!(
            compatibility.contains(matcher),
            "stable policy-v3 contract must name matcher {matcher}"
        );
    }
}

#[test]
fn stable_action_contract_includes_alpha6_custody_inputs_and_current_outputs() {
    let compatibility = read("docs/COMPATIBILITY.md");
    let action = read("action.yml");

    for input in [
        "command",
        "baseline",
        "policy",
        "expected-baseline-digest",
        "expected-policy-sha256",
        "require-custody",
        "fail-on-review",
        "upload-artifact",
        "artifact-name",
    ] {
        assert!(
            action.contains(&format!("  {input}:")),
            "action.yml must expose stable candidate input {input}"
        );
        assert!(
            compatibility.contains(&format!("`{input}`")),
            "v1 contract must explicitly classify Action input {input}"
        );
    }

    for output in [
        "verdict",
        "exit-code",
        "report-json",
        "summary-markdown",
        "sarif-status",
        "artifact-url",
        "artifact-digest",
    ] {
        assert!(
            action.contains(&format!("  {output}:")),
            "action.yml must expose stable candidate output {output}"
        );
        assert!(
            compatibility.contains(&format!("`{output}`")),
            "v1 contract must explicitly classify Action output {output}"
        );
    }
}

#[test]
fn stable_contract_narrows_workload_performance_and_backend_claims() {
    let compatibility = read("docs/COMPATIBILITY.md");

    assert!(
        compatibility.contains("highly nondeterministic build/test graphs")
            && compatibility.contains("may legitimately produce REVIEW"),
        "v1 must state the bounded build/test nondeterminism contract"
    );
    assert!(
        compatibility.contains("no universal low-overhead promise")
            && compatibility.contains("native ptrace overhead is workload-dependent"),
        "v1 must freeze a bounded performance claim"
    );
    assert!(
        compatibility.contains("Linux x86_64")
            && compatibility.contains("native `ptrace`")
            && compatibility.contains("eBPF/BPF-LSM")
            && compatibility.contains("not part of the stable v1 contract"),
        "experimental backend exclusion must remain explicit"
    );
}

#[test]
fn stable_contract_uses_alpha6_as_upgrade_source_and_preserves_alpha5_boundary() {
    let compatibility = read("docs/COMPATIBILITY.md");
    let cargo = read("Cargo.toml");

    assert!(
        compatibility.contains("Alpha.6 profile-4")
            && compatibility.contains("qualified upgrade source"),
        "v1 upgrade contract must preserve Alpha.6/profile-4 as the qualified upgrade source"
    );
    assert!(
        compatibility.contains("Alpha.5 profile-3")
            && compatibility.contains("explicitly rejected as semantically incomparable"),
        "Alpha.5 profile-3 must remain an explicit semantic boundary"
    );
    assert!(
        cargo.contains("rust-version = \"1.82\""),
        "declared source MSRV changed unexpectedly"
    );
}

#[test]
fn stable_contract_preserves_install_route_specific_environment_boundary() {
    let compatibility = read("docs/COMPATIBILITY.md");

    assert!(
        compatibility.contains("exact public prebuilt artifact: PASS on Ubuntu 24.04 x86_64"),
        "v1 contract must retain the qualified public-binary environment"
    );
    assert!(
        compatibility.contains("FAIL on Ubuntu 22.04") && compatibility.contains("GLIBC_2.39"),
        "v1 contract must retain the current Alpha.6 prebuilt incompatibility as negative evidence"
    );
    assert!(
        compatibility.contains("exact-version local build/install")
            && compatibility.contains("Ubuntu 22.04 x86_64")
            && compatibility.contains("Ubuntu 24.04 x86_64"),
        "v1 contract must distinguish local-build evidence from prebuilt-binary evidence"
    );
}

#[test]
fn stable_support_policy_freezes_channel_deprecation_and_rollback_rules() {
    let support = read("docs/SUPPORT_POLICY.md");

    assert!(
        support.contains("exact releases such as `v1.0.0`")
            && support.contains("immutable release identities"),
        "stable exact-release immutability must be explicit"
    );
    assert!(
        support.contains("AETHERXGLOBAL/execsurface@v1")
            && support.contains("moving stable GitHub Action channel"),
        "stable Action channel semantics must be explicit"
    );
    assert!(
        support.contains("No fixed calendar maintenance period or response-time SLA"),
        "support policy must not invent an organizational SLA"
    );
    assert!(
        support.contains("move the `@v1` channel back")
            && support.contains("never rewrite user baselines or policies"),
        "bad-release rollback must preserve user evidence"
    );
}
