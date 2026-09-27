# Post-M9 — Ptrace Fast-Path A2 Split Semantic Parity v5 Result

Date: 2026-09-27
Status: **CLOSED / PASS — SEMANTICALLY ELIGIBLE FOR FROZEN EXTERNAL VALUE MEASUREMENT**
Parent: GitHub issue #84
Workflow run: `36289614124`
Source SHA: `239ade5fb6139acc635f907e3bfd4393e8ad7b59`
Artifact ID: `10921477605`
Artifact digest: `sha256:3d5908b9ca142db475058c22f850640ed50daa3e3e03db507d9ff58e7d699f58`
Protocol: `docs/milestones/POST_M9_PTRACE_FASTPATH_A2_PARITY_V5_PROTOCOL.md`

## Decision

Candidate A2 has passed the preregistered semantic-eligibility gate and may proceed to the separately frozen external performance-value experiment.

This result does **not** merge A2 into the public runtime, change the public release, or establish a public performance claim.

## Engineering regression gate

All passed on the runner-only A2 implementation:

- workspace Clippy with `-D warnings`;
- `execsurface-observe` tests;
- full workspace tests;
- release build of candidate binary.

The public `main` runtime source remained unchanged during the experiment.

## Lane A — strict corrected raw parity

All thirteen Lane A cases passed:

| Case | Reference/reference deterministic | Reference/A2 exact after TID-only projection | Complete | Warnings |
| --- | --- | --- | --- | --- |
| irrelevant | PASS | PASS | true | 0 |
| fileio | PASS | PASS | true | 0 |
| failed-open | PASS | PASS | true | 0 |
| signal | PASS | PASS | true | 0 |
| restart | PASS | PASS | true | 0 |
| restart-single | PASS | PASS | true | 0 |
| fork | PASS | PASS | true | 0 |
| vfork | PASS | PASS | true | 0 |
| exec | PASS | PASS | true | 0 |
| failed-exec | PASS | PASS | true | 0 |
| thread-exec | PASS | PASS | true | 0 |
| exit-group | PASS | PASS | true | 0 |
| fixture-spawn | PASS | PASS | true | 0 |

The raw comparator retained exact event count, order, sequence and every non-TID payload field. Only local runtime TID identity, including flattened spawn `child_tid`, was renamed consistently.

## Lane B — concurrent production semantic parity

Workload:

`execsurface-fixture thread-read <fixed-path> 8`

Health gate:

- reference observation 1: complete, zero warnings, exit 0;
- reference observation 2: complete, zero warnings, exit 0;
- A2 observation: complete, zero warnings, exit 0;
- backend metadata: identical.

Using the **unchanged production `learn` / canonicalization / diff path**:

1. reference baseline -> reference check 1: `added=0`, `removed=0`, `changed=0`;
2. reference baseline -> reference check 2: `added=0`, `removed=0`, `changed=0`;
3. reference baseline -> A2 check: `added=0`, `removed=0`, `changed=0`;
4. A2 baseline -> reference check: `added=0`, `removed=0`, `changed=0`.

No policy suppression and no new normalization rule was introduced.

The v4 raw interleaving counterexample remains preserved as raw negative evidence; this result does not reclassify it as raw-order PASS.

## Mechanism activation diagnostic

Synthetic addition: `+5000` raw `getpid` syscalls.

Measured `PTRACE_GET_SYSCALL_INFO` counts:

### Reference

- N=0: `59`
- N=5000: `10059`
- delta: **`10000`** — exactly `2N`

### Candidate A2

- N=0: `38`
- N=5000: `5038`
- delta: **`5000`** — exactly `N`

Both reference and candidate activation observations remained complete, warning-free and exit-0.

This proves the intended mechanism is active: A2 removes one exit-side syscall-info request for each eligible added irrelevant syscall while retaining every syscall stop/resume boundary.

Classification: **DIAGNOSTIC_MECHANISM_EVIDENCE_ONLY**. This is not yet a production speed claim.

## What v5 establishes

Within the declared adversarial and production-semantic test scope:

- A2 preserves exact raw evidence for reference-deterministic lifecycle, signal, restart, fork/vfork, exec, failed-exec, thread-exec, exit-group, fd/file and spawn cases;
- A2 preserves production canonical/baseline semantics on a deliberately concurrent thread-read workload whose reference raw interleaving is not stable;
- A2's intended GETINFO reduction is actually activated;
- no runtime source was merged to `main` by this gate.

## What remains OPEN

- whether A2 reduces ptrace wall time by the preregistered >=10% threshold on both pinned external workloads;
- whether the measured value generalizes beyond the exact runner/workloads;
- merge/release eligibility.

## Next gate

Run the frozen external value experiment on:

- `BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`;
- `junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`.

Acceptance remains:

- 3 warmups + 15 measured samples per reference/A2 mode;
- alternating order;
- no clean-sample removal;
- complete/warning-free/matching outcomes;
- production semantic parity before timing;
- A2 median ptrace wall time lower on both targets;
- >=10% median ptrace wall-time reduction on each target.

## Authority boundary

- public release remains `v0.1.0-alpha.3`;
- public runtime on `main` remains the existing ptrace implementation;
- eBPF authority is unchanged;
- A2 is **semantically eligible for value measurement, not yet merge-authorized**.
