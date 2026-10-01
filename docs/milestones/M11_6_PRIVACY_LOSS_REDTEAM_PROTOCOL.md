# M11.6 — Privacy / Loss / Adversarial Red-Team Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — RED-TEAM AUTHORIZED**

## Objective

Attempt to break the M11 evidence boundary before any compatibility or release decision. The gate targets privacy leakage, false completeness, failed-operation promotion, capability fallback, session ambiguity, and deployment-boundary mistakes.

## Fixed roles

- **Innovative Systems Architect** — search for a stronger fail-closed design if a counterexample is found.
- **Anti-Deviation / Skeptical Reviewer** — prefer a preserved negative result over any weakened test, reinterpreted failure, or undocumented exception.

## Dynamic specialists

Linux ptrace semantics, privacy/data minimization, BPF loss accounting, container/user-namespace privilege boundaries, session isolation, Rust adversarial testing, and evidence semantics.

## Red-team attacks

### RT1 — argv privacy sentinel

Execute the public ptrace observer with a high-entropy sentinel supplied only as an argv value that the fixture does not otherwise emit. Serialized observation evidence must not contain the sentinel.

### RT2 — file-content privacy sentinel

Place a high-entropy sentinel in file content, read the file through the observer, and assert serialized observation evidence does not contain the content sentinel. Path metadata is allowed; content is not.

### RT3 — shared-FD ambiguity

The accepted M11.3 concurrent fixture must remain `complete=false` with `shared_fd_table_ambiguity`; no later gate may silently restore PASS eligibility.

### RT4 — producer loss

The accepted real-loss evidence from M10.6 remains admissible only with its exact bounded scope: 200,000 registered producer attempts, 127 accepted events, 199,873 producer reserve failures, exact accounting, `evidence_complete=false`, `pass_authority=false`.

M11.6 must also test the integrated health mapping: any non-zero producer drop count -> `IncompleteLoss` -> non-PASS.

### RT5 — malformed/session ambiguity

Malformed, wrong-session, or unknown-role hybrid records must map to `IncompleteAmbiguity` and remain non-PASS.

### RT6 — failed-operation non-promotion

The hybrid contract must keep candidate and success propositions distinct. Missing success-confirmation capability blocks adapter activation. Unsupported file-open success must not be inferred from file-object candidate evidence.

### RT7 — deployment boundary / privilege

The accepted M10.7 privilege matrix remains the real environment evidence:

- standard GitHub-hosted runner: BPF LSM not active in the tested host;
- default container: minimal BPF operation denied;
- privileged container: generic BPF available but host BPF LSM still absent;
- boot-controlled guest root: active BPF LSM + BTF + BPF operation available;
- unprivileged/user-namespace cases: tested BPF operation denied.

M11.6 must prove adapter behavior remains fail-closed for capability contexts representing unsupported platform, missing BPF LSM, missing BTF, denied BPF operation, unacknowledged privilege, or missing loss/success sources. There is no ptrace fallback from an explicitly requested hybrid contract.

## Acceptance criteria

1. argv sentinel absent from serialized observation;
2. file-content sentinel absent from serialized observation;
3. M11.3 ambiguity regression remains green;
4. nonzero hybrid producer loss -> non-PASS;
5. malformed/session/role ambiguity -> non-PASS;
6. missing hybrid prerequisites -> explicit error;
7. success propositions are not satisfied by candidate-only evidence;
8. ptrace ↔ hybrid cross-backend reuse remains rejected;
9. frozen baseline-v2 digest remains green;
10. full-workspace Clippy/tests remain green;
11. no public product mutation.

## Interpretation rule

Passing this gate proves only the tested privacy and fail-closed invariants. It does not prove universal absence of side channels, universal kernel coverage, zero loss, container portability, or production readiness.

## Stop rule

Any sentinel leak, incomplete evidence retaining PASS eligibility, candidate-to-success promotion, implicit fallback, or silent privilege assumption blocks M11.7.
