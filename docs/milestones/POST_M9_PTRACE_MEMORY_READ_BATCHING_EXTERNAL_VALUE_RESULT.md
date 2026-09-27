# Post-M9 — Candidate B External Value Result

Date: 2026-09-27
Status: **CLOSED — SEMANTICALLY VALID / VALUE NOT ESTABLISHED**
Parent: GitHub issue #84
Candidate: B
Source SHA: `b8a134c1f14e6d64aeac473d0ccb159a22ab2d79`
Workflow run: `36299979602`
Acceptance threshold: at least `10%` median end-to-end ptrace observer wall-time reduction independently on both pinned targets.

## Decision

Candidate B is **rejected for public integration** under the preregistered engineering-value gate.

The candidate preserved the required production semantics on both external targets, but it did not achieve the frozen performance threshold on either target. The threshold is not relaxed after observing the results.

Candidate B remains experimental and unmerged. No public performance claim is authorized.

## Frozen protocol

Per target and mode:

- pinned target revision;
- GitHub-hosted Ubuntu 24.04 runner class;
- reference and Candidate B built and measured within the same matrix job;
- production semantic prerequisite before timing;
- 3 warmups + 15 measured samples;
- alternating reference/candidate order;
- no outlier removal;
- no sample replacement;
- acceptance requires candidate median lower than reference by at least `10%` on **each** target.

## Production semantic prerequisite

Both pinned targets passed before timing:

- observations complete;
- warnings empty;
- target exit `0`;
- reference baseline -> reference: zero added/removed/changed;
- reference baseline -> Candidate B: zero added/removed/changed;
- Candidate B baseline -> reference: zero added/removed/changed.

Thus the performance result is classified as a value failure, not a semantic failure.

## `ripgrep`

Target:

`BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`

Result:

- reference median: `514.911187 ms`
- reference MAD: `0.091084 ms`
- Candidate B median: `514.903474 ms`
- Candidate B MAD: `0.113352 ms`
- median reduction: `0.007713 ms`
- median reduction: `0.001497928%`
- threshold: `10%`
- threshold result: **FAIL**

Evidence artifact:

- artifact ID: `10925128779`
- digest: `sha256:4c9e9b62bf96d7d77e1e10be597ea348f3876f711075272e4f629ef1196e1296`

Classification:

`SEMANTICALLY_VALID_VALUE_NOT_ESTABLISHED`

## `fzf`

Target:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Result:

- reference median: `614.777601 ms`
- reference MAD: `0.104242 ms`
- Candidate B median: `564.899969 ms`
- Candidate B MAD: `0.110063 ms`
- median reduction: `49.877632 ms`
- median reduction: `8.113117966%`
- threshold: `10%`
- threshold result: **FAIL**

Evidence artifact:

- artifact ID: `10925261636`
- digest: `sha256:70d2207e9e5304755e4ec6bcded6c45930b8d93ce6a50c90a5bad18bc4aed082`

Classification:

`SEMANTICALLY_VALID_VALUE_NOT_ESTABLISHED`

## Interpretation boundary

The `fzf` result is retained as bounded evidence that batching selected tracee-memory reads can reduce end-to-end observer time for at least this pinned workload on this run. It is **not** generalized into a cross-workload or public speed claim.

The near-zero `ripgrep` effect shows that tracee-memory read batching is not a sufficiently strong common cost lever for the current acceptance objective.

No integration gate is authorized for Candidate B.

## Next action

Return to measured cost attribution and search for a higher-leverage cost source shared across the pinned workloads. Any next optimization candidate must be preregistered and pass semantic gates before external value measurement. Negative evidence from A2 and Candidate B remains part of the evidence ledger and must not be deleted or rewritten.
