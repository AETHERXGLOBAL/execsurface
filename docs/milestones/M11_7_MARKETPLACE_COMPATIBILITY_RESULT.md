# M11.7 — Marketplace / Public Compatibility Regression Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Classification: **M11_7_PUBLIC_COMPATIBILITY_PASS_BOUNDED**
Status: **CLOSED**

## Team

Fixed roles:

- **Innovative Systems Architect** — preserved a path to stronger evidence authority without sacrificing the existing portable self-service path.
- **Anti-Deviation / Skeptical Reviewer** — treated public-surface drift, implicit privilege, baseline reinterpretation, or hybrid auto-selection as blockers.

Dynamic specialists:

- GitHub Actions / Marketplace release engineering
- CLI backward compatibility
- baseline-v2 serialization/digest compatibility
- Linux CI privilege-boundary review
- external-consumer smoke testing
- Rust integration regression testing

## Source-of-truth reference

- frozen public `main`: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- M11.7 test commit: `030eaad5a41637386d29c39371c6be9e30b9472f`
- public stable action channel: `AETHERXGLOBAL/execsurface@v0.1`
- release pointer: `v0.1.0-alpha.3`

No public tag, release, Marketplace publication, or `main` branch mutation was made by this gate.

## CI evidence

Workflow: **M11.7 Marketplace Compatibility**

Run: `36450566559`
Conclusion: **success**

### Job 1 — Research default / frozen public surface

**PASS**

- fetched frozen public `main`;
- asserted no diff for `action.yml`, `action/install.sh`, `action/run.sh`, or `action/release-tag.txt`;
- built the M11 branch CLI normally;
- `doctor` succeeded on the normal GitHub-hosted runner;
- learn/check on unchanged workload remained PASS;
- controlled drift remained REVIEW with exit code `10`;
- no BPF-LSM boot, privileged container, or managed-kernel setup was required;
- frozen baseline-v2 digest test passed;
- explicit hybrid adapter/red-team tests passed and hybrid remained non-default.

### Job 2 — Published release / no-Rust consumer path

**PASS**

- downloaded the current public release artifact;
- verified its published SHA-256;
- verified the release version;
- `doctor` passed;
- unchanged learn/check passed;
- controlled drift returned REVIEW with stable exit code `10`.

### Job 3 — Stable Marketplace Action / `v0.1`

**PASS**

- created a baseline with the current public release;
- `AETHERXGLOBAL/execsurface@v0.1` returned `pass` / `0` for unchanged execution;
- the same stable Action returned `review` / `10` for controlled drift;
- no hybrid/BPF configuration was required from the consumer.

## Anti-deviation review

M11.7 did not use compatibility success to promote the hybrid backend. It proves only that the tested current public paths remain intact while the authority-aware work stays isolated.

This result does not prove universal runner compatibility, production readiness, or hybrid deployability on arbitrary CI hosts.

## Gate decision

**M11.7 is CLOSED as `M11_7_PUBLIC_COMPATIBILITY_PASS_BOUNDED`.**

M11.8 is authorized for performance evidence and the final release/default-backend decision. Public `main` remains frozen until that decision is complete.
