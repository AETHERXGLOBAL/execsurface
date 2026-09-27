# Post-M9 — Ptrace Fast-Path A2 External Value Result

Date: 2026-09-27
Status: **CLOSED — A2 REJECTED / DO NOT MERGE**
Parent: GitHub issue #84
Protocol: `docs/milestones/POST_M9_PTRACE_FASTPATH_A2_EXTERNAL_VALUE_PROTOCOL.md`
Candidate: A2
Source SHA: `ed234e1af0eae0b187ada57720ecde3c720cd4b0`
Workflow run: `36290023951`

## Decision

Candidate A2 does **not** satisfy the preregistered external value gate and is not eligible for merge or a public performance claim.

Two independent negative conditions were observed under the frozen protocol:

1. `ripgrep` passed the production semantic prerequisite but did not establish performance value. The candidate median was not lower than the reference median and the preregistered `>=10%` improvement threshold failed.
2. `fzf` did not pass the production semantic prerequisite. The A2 candidate terminated with a fail-closed observer protocol error before timing acceptance: `phase-aware A2 exit fast path reached unsafe state for tid 4805`.

The earlier A2 v5 semantic result remains valid only for the fixture scope it actually tested. The pinned external `fzf` workload exposed a runtime state not covered by that gate. The new evidence therefore prevents treating A2 as semantically eligible for integration.

No threshold, target, command, normalization rule, policy, sample rule, public backend authority, or runtime semantics were changed after observing these results.

## Harness status

This run occurred after the first external-value harness failure was preserved and the `rustfmt` prerequisite was repaired.

On source `ed234e1af0eae0b187ada57720ecde3c720cd4b0`:

- both pinned target builds completed;
- the reference binary built;
- the A2 patch applied;
- the A2 candidate binary built successfully.

Therefore the negative result below is **not classified as the previous harness failure**.

## PTRACE-A2-VALUE-RG-001 — ripgrep

Pinned target:

`BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`

Semantic prerequisite: **PASS**

- reference -> reference unchanged: zero diff findings;
- reference -> A2 unchanged: zero diff findings;
- A2 -> reference unchanged: zero diff findings;
- observations complete and warning-free;
- target outcome parity preserved.

Frozen 3 warmup + 15 measured samples completed for both reference and candidate.

Accepted value summary:

- reference median: `464.662820 ms`
- reference MAD: `0.049624 ms`
- A2 median: `464.672428 ms`
- A2 MAD: `0.047837 ms`
- median reduction: `-0.009608 ms`
- median reduction percent: `-0.00206773591225015%`
- candidate median lower than reference: **false**
- `>=10%` value threshold: **FAIL**
- evaluator classification: `SEMANTICALLY_VALID_VALUE_NOT_ESTABLISHED`

Artifact:

- artifact ID: `10921714481`
- digest: `sha256:335f35f31062e3932c1de5bdb0b251e10c086bac8bf453b2def1ed086f7ddb6e`

Interpretation: halving the candidate's `PTRACE_GET_SYSCALL_INFO` activation count on the synthetic activation check did not translate into a measurable end-to-end ptrace observer speedup on this pinned workload. The measured median difference is effectively zero at this scale and has the wrong sign for the frozen gate.

## PTRACE-A2-VALUE-FZF-001 — fzf

Pinned target:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Build/prerequisite setup: **PASS through candidate build**

Production semantic prerequisite: **FAIL**

The reference observation completed, but the candidate observation terminated fail-closed with:

`execsurface: observer protocol error: phase-aware A2 exit fast path reached unsafe state for tid 4805`

Per the frozen protocol, timing for this target is not accepted. The 3+15 measurement and value-gate steps were correctly skipped.

Artifact:

- artifact ID: `10921418256`
- digest: `sha256:1b93b6d22c9a8f6e54288770401e1dc3c0d841f96682069acdf825665645321f`

Interpretation: this is a candidate runtime/semantic safety failure, not a benchmark-value failure and not the previously repaired harness defect. The fail-closed behavior is preferable to silent evidence loss, but it kills A2 under the current gate.

## Frozen gate evaluation

The protocol requires both pinned targets independently to satisfy:

- production semantic prerequisite PASS;
- all accepted observations complete and warning-free;
- candidate median lower than reference;
- at least `10%` median ptrace observer wall-time reduction;
- no sample manipulation after observing results.

Result:

- ripgrep: **VALUE FAIL**;
- fzf: **SEMANTIC PREREQUISITE FAIL**;
- overall A2 gate: **FAIL**.

Classification:

`A2_REJECTED_DO_NOT_MERGE`

## Consequence

A2 must not be merged into the public native ptrace runtime in its tested form.

Do not rerun the same candidate merely to seek a favorable timing result. Any further experiment must be a prospectively defined new candidate or a narrowly justified semantic repair with its own adversarial gate before value measurement.

The evidence also changes the optimization hypothesis: reducing one `PTRACE_GET_SYSCALL_INFO` request on eligible exit stops is not sufficient by itself to establish useful end-to-end savings on the accepted `ripgrep` workload. Future work should return to measured attribution and target a larger cost component without weakening authority, completeness, lifecycle, fd/path, or fail-closed semantics.

Public release remains unchanged. Native ptrace remains the correctness-reference/public backend. No performance improvement claim is accepted from A2.