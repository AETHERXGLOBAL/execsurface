# M11.6 — Privacy / Loss / Adversarial Red-Team Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Classification: **M11_6_PRIVACY_LOSS_REDTEAM_PASS_BOUNDED**
Status: **CLOSED**

## Team

Fixed roles:

- **Innovative Systems Architect** — searched for stronger fail-closed alternatives whenever the existing evidence contract could be broken.
- **Anti-Deviation / Skeptical Reviewer** — treated every leak, silent fallback, false-completeness path, or unsupported promotion as a blocking failure.

Dynamic specialists for this gate:

- Linux ptrace semantics
- BPF/LSM loss-accounting semantics
- privacy and data-minimization review
- session-isolation / malformed-record review
- Rust invariant and adversarial-test engineering
- CI reproducibility / compatibility review

## Source-of-truth state

- Research HEAD tested: `68da983fc080dacd731625bb41a354a253b453ac`
- Public `main` remained unchanged at `db11761e2d75ebca7d4458dc39094f4aed6a26bb` during closeout.
- No release, Marketplace action, public tag, or public default backend was changed by this gate.

## Red-team outcomes

### RT1 — argv privacy sentinel

**PASS (bounded).** A high-entropy sentinel supplied only as an argv value was not present in serialized ptrace observation evidence.

### RT2 — file-content privacy sentinel

**PASS (bounded).** A high-entropy sentinel stored only in file content and read under observation was not present in serialized observation evidence. File-path metadata remained intentionally observable.

### RT3 — shared-FD ambiguity

**PASS.** The M11.3 fail-closed invariant remained active: clone-based FD-sharing ambiguity is represented as incomplete evidence and is not PASS-eligible.

### RT4 — producer loss

**PASS.** The hybrid health mapping treats any non-zero producer drop count as incomplete loss and non-PASS. This preserves the accepted M10.6 bounded loss result rather than reinterpreting loss as completeness.

### RT5 — malformed/session ambiguity

**PASS.** Malformed, wrong-session, and unknown-role records map to non-complete evidence health and remain non-PASS.

### RT6 — failed-operation non-promotion

**PASS.** Candidate evidence and success evidence remain distinct. Missing exec/connect success-confirmation capabilities reject hybrid activation. File-object candidate evidence does not satisfy `FileOpenSuccess`.

### RT7 — deployment / privilege boundary

**PASS at the contract layer.** Unsupported platform, inactive BPF LSM, missing BTF, denied BPF operation, unacknowledged privilege, or missing producer-loss accounting each fail explicitly. The explicit hybrid contract does not silently fall back to ptrace.

The real environment boundary remains the accepted M10.7 evidence; M11.6 does not claim broader deployment support.

## Compatibility and authority checks

- ptrace ↔ hybrid baseline interchangeability remains rejected unless semantic equivalence is separately proven.
- frozen baseline-v2 compatibility remained green.
- full-workspace Clippy and tests remained green.
- public product state was not mutated.

## CI evidence

GitHub Actions run: `36449724228`

Conclusion: **success**

Successful steps included:

- format authority crates
- format observer changes
- full-workspace Clippy
- authority tests
- explicit hybrid adapter tests
- proposition comparability red-team
- hybrid fail-closed red-team
- ptrace authority bridge tests
- shared-FD ambiguity tests
- ptrace privacy sentinel red-team
- frozen baseline-v2 compatibility tests
- full-workspace tests
- lockfile integrity

## Anti-deviation review

No acceptance criterion was weakened after observing test results. The privacy tests demonstrate absence of the tested sentinel values from serialized evidence; they do **not** prove universal absence of side channels or all possible sensitive metadata leakage.

The fail-closed tests demonstrate the tested loss/session/capability invariants; they do **not** prove universal kernel coverage, zero loss, container portability, or production readiness.

## Gate decision

**M11.6 is CLOSED as `M11_6_PRIVACY_LOSS_REDTEAM_PASS_BOUNDED`.**

M11.7 is authorized to test Marketplace/public-user compatibility. Hybrid remains explicit/managed/research-only and cannot become the public default as a consequence of M11.6 alone.
