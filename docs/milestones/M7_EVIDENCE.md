# M7 — External Real-World Proof Evidence

Date: 2026-09-26

Status: **CLOSED / ACCEPTED — COMPATIBILITY PROOF**

Issue: #23

External target:

`sharkdp/fd`

Pinned upstream commit:

`ce97e473ebaec49697c07daa50a7bc2b32f713d2`

Final M7 branch implementation commit:

`693fd55716fd8f9433f0019c281e5dd66a8394d9`

## Scope

M7 answers one question:

> Can the already-published ExecSurface product operate meaningfully on a real external open-source developer tool without weakening M6.5/M6.6 semantics?

It does **not** establish third-party adoption.

No change was pushed to the upstream repository.

No maintainer was contacted before compatibility proof completion.

## Dynamic team

Task-specific:

- External OSS Integration Engineer
- Rust Build / Reproducibility Engineer
- Linux Runtime Engineer
- Developer Tooling Engineer
- Evidence / CI Engineer
- independent Adversarial Tester

Fixed throughout:

- **Innovation Architect**
- **Deviation Prevention / Scientific Integrity**

## Target rationale

`fd` is a mature real Rust CLI and developer tool.

The selected upstream pin provided:

- a committed Cargo.lock;
- Linux CI;
- a reproducible locked Rust test suite;
- a real release binary;
- runtime filesystem behavior suitable for a controlled local fixture.

Target selection:

`docs/milestones/M7_TARGET_FD.md`

## Final successful workflow

Run:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36200690279

Result:

**PASS**

Associated ExecSurface CI:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36200690286

Result:

**PASS**

## External reproduction

The workflow checked out exactly:

`sharkdp/fd@ce97e473ebaec49697c07daa50a7bc2b32f713d2`

with checkout credentials not persisted.

Environment recorded in the evidence artifact:

- ExecSurface: `0.1.0-alpha.1`
- rustc: `1.98.1`
- cargo: `1.98.1`
- runner: Linux x86_64, Ubuntu-hosted GitHub runner
- upstream push: none

The pinned upstream suite:

`cargo test --locked --all-features`

completed with:

**118 passed, 0 failed**

The real upstream release binary built successfully:

`fd 10.5.0`

## ExecSurface release integrity in M7

M7 did not build the consumer CLI from the branch under test.

It downloaded the already-published public release:

`v0.1.0-alpha.1`

and verified:

- published SHA-256;
- GitHub build provenance with `gh attestation verify`;
- binary product version.

This keeps M7 tied to the public product path established by M6.6.

## Baseline

Real command:

`/bin/bash -lc 'cd external/fd && ./target/release/fd --hidden --type f . m7-fixture >/dev/null'`

Learned baseline digest:

`sha256:6d79d51c5bd9203a27005708991a82cf57511c983c14a9dc98b588fad5e658ac`

Canonical effects:

`517`

## No-drift stability

The same real `fd` command was checked twice against the same explicit baseline.

Both reports:

- verdict: **PASS**
- findings: **0**
- target exit: 0

This is computational compatibility evidence for the pinned workload/environment.

It is not a universal determinism claim for all `fd` modes or machines.

## Controlled runtime expansion

Controlled command:

`/bin/bash -lc 'cd external/fd && ./target/release/fd --hidden --type f . m7-fixture >/dev/null; /usr/bin/sha256sum m7-fixture/alpha.txt >/dev/null'`

Result:

- verdict: **REVIEW**
- exit code: **10**
- total findings: **38**
- all findings: **added**

Finding composition:

- `process_exec`: 1
- `file_open`: 32
- `file_read`: 4
- `file_write`: 1

The key process finding is:

`/usr/bin/sha256sum`

The controlled fixture input is present as an fd-attributed read:

`$WORKSPACE/external/fd/m7-fixture/alpha.txt`

with canonical execution chain:

`bash → sha256sum`

This demonstrates a meaningful observed execution-surface expansion rather than a string-only fixture assertion.

No claim is made that the controlled expansion is malicious.

## Stable public Action proof

The same external checkout/baseline was tested through:

`AETHERXGLOBAL/execsurface@v0.1`

### Accepted command

Result:

**PASS**

### Controlled expansion

Result:

**REVIEW**

Exit code:

`10`

Therefore M7 proves both the release CLI and the promoted public Action channel on the pinned external workload.

## Evidence artifact

Final artifact:

`m7-fd-evidence-36200690279-1`

Artifact ID:

`10891383482`

Contents:

- `environment.txt`
- `fd.lock.json`
- two PASS JSON reports
- two PASS Markdown reports
- controlled-drift JSON report
- controlled-drift Markdown report

The artifact was produced by the successful M7 run and retained by GitHub Actions.

## Preserved negative evidence

### 001 — Initial omitted explicit baseline path

`docs/milestones/M7_NEGATIVE_001_HARNESS_BASELINE_PATH.md`

Run:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36200186478

The CLI correctly returned ERROR because the harness learned to a non-default path and then omitted `--baseline`.

### 002 — Partial harness fix remained incomplete

`docs/milestones/M7_NEGATIVE_002_PARTIAL_BASELINE_FIX.md`

Run:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36200439290

The repeated no-drift checks still omitted the explicit baseline path.

The failure was preserved and the full workflow rerun after correction.

Neither failure justified a core semantic change.

## Red-Team findings

The final proof was reviewed against the following failure modes:

- accidental use of branch-built ExecSurface instead of public release — **not present**;
- unpinned upstream source — **not present**;
- upstream credential persistence — **disabled**;
- hidden upstream modification — only local untracked fixture files were created; tracked upstream state remained unchanged;
- PASS obtained by broad normalization — **not introduced**;
- controlled drift asserted only from exit status — **no; JSON finding content was verified**;
- Action-only proof without CLI proof — **no; both were exercised**;
- adoption claim without maintainer action — **not made**.

## Remaining scope limits

M7 proves one pinned external workload on Linux x86_64.

It does not prove:

- compatibility with all `fd` commands;
- compatibility with all Rust projects;
- third-party adoption;
- safety of `fd`;
- that no unobserved behavior exists;
- that ptrace overhead is acceptable for every external workload.

The core boundary remains:

> observed behavior ≠ all possible behavior

## Gate decision

**M7 EXTERNAL COMPATIBILITY PROOF — ACCEPTED**

A narrow maintainer proposal is now permitted.

Any maintainer acceptance, merge, endorsement or production use must be recorded separately and must not be inferred from this compatibility result.
