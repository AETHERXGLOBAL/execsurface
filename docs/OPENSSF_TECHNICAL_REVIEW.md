# ExecSurface — OpenSSF Technical Review Pack

Tracking: #94, #114, #118
Current public release: `v0.1.0-alpha.5`
Release source: `9e73b925d55557e33de1b0813995609aaefdc037`
Audience: OpenSSF engineers, maintainers and researchers evaluating technical fit, overlap, reproducibility, architecture and interoperability.

The prior Alpha.4 review pack is retained unchanged at `docs/archive/engagement/OPENSSF_TECHNICAL_REVIEW_ALPHA4.md`.

## Review intent

This is a request for **technical criticism and independent reproducibility**, not endorsement. Negative evidence, reproduction failure, no-fit conclusions, architecture criticism, counterexamples and overlap findings are explicitly useful outcomes.

## Problem and product boundary

ExecSurface learns an accepted canonical **observed runtime execution surface** for a declared command, then reports later observed execution behavior that appeared, disappeared or changed under a recorded observer and explicit policy.

ExecSurface is an observer-backed runtime evidence generator, canonical baseline/deterministic diff system, explicit drift-policy evaluator and attestable verification-result producer.

It is not antivirus, EDR, SIEM, malware classification, sandboxing, general Linux enforcement/MAC or proof that software is safe.

`observed behavior != all possible behavior`

## Public architecture

`declared command -> native ptrace reference observer -> authority/completeness-aware evidence -> canonical execution surface -> deterministic semantic diff -> explicit policy -> PASS/REVIEW/BLOCK/ERROR -> verification evidence/attestation`

Native ptrace is the bounded public reference observer, not universal authority for every possible Linux proposition. BPF-LSM/kernel-hook and hybrid work remains research/managed and non-default.

## Current limitations reviewers should challenge

- Linux x86_64 public alpha only; ARM64 support is not claimed.
- ptrace availability depends on kernel/security/container/namespace configuration.
- ptrace can perturb workloads and has workload-dependent overhead.
- pathname userspace observations are not kernel-object identity.
- exact universal shared-FD attribution under concurrency is not claimed.
- mmap/io_uring and every possible Linux I/O path are not claimed as fully covered.
- metadata privacy intentionally excludes file contents, environment values, stdin, network payloads and full child argv.
- incomplete/ambiguous evidence must remain non-PASS-eligible.
- backend name, signature presence, provenance presence, frequency or similarity cannot silently create semantic authority.

## Independent installation

### Registry

```bash
cargo install execsurface --version "=0.1.0-alpha.5" --locked
execsurface --version
execsurface doctor
```

### GitHub Action

Stable public-alpha channel:

```yaml
uses: AETHERXGLOBAL/execsurface@v0.1
```

Immutable review pin:

```yaml
uses: AETHERXGLOBAL/execsurface@v0.1.0-alpha.5
```

## Minimal falsification walkthrough

```bash
rm -rf /tmp/execsurface-review
mkdir -p /tmp/execsurface-review
cd /tmp/execsurface-review
execsurface learn -- /bin/bash -lc 'true'
execsurface check --json-output pass-report.json -- /bin/bash -lc 'true'
set +e
execsurface check --json-output drift-report.json -- /bin/bash -lc 'true; /bin/echo controlled-drift >/dev/null'
status=$?
set -e
printf 'controlled drift exit status: %s\n' "$status"
```

In a supported complete run, unchanged execution should be PASS/0 and controlled expansion should be REVIEW/10. Any different result should be retained and reported, not normalized away.

## Evidence to return

Please report OS/kernel/architecture/container context, exact version/install path, `doctor` result, commands and exit codes, completeness/observer-health state, reproducible JSON evidence, setup/privilege friction, measured overhead, suspected false PASS/REVIEW/incompleteness, interoperability concerns, and evidence of redundancy/no-fit where applicable.

Do not send secrets or private payload data.

## Post-release validation governance

Alpha.5 was published under explicit owner authorization before P8 independent external validation closed. P8 remains open after release. Internal release proofs do not become independent evidence merely because the release is public.

Do not describe external interaction as OpenSSF validation, Linux Foundation approval, partnership, endorsement or adoption unless exact external evidence supports that wording.

Public review hub: issue #118. P8 evidence ledger/gate: issue #114.
