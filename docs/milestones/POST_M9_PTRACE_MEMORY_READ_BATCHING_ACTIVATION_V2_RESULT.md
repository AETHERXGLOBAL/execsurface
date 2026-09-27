# Post-M9 — Candidate B Activation v2 Result

Date: 2026-09-27
Status: **CLOSED — PASS / VALUE TESTING AUTHORIZED**
Parent: GitHub issue #84
Candidate: B
Source SHA: `22fe7db920daadcc8fe774609891d429fb1d87ec`
Workflow run: `36299794461`
Artifact: `10924009518`
Artifact digest: `sha256:ab1353c1cc02cd79c9ea40403ac67d3ac2a930f8c7c2e7179499e9324f04ea78`
Prior preserved harness failure: `docs/milestones/POST_M9_PTRACE_MEMORY_READ_BATCHING_PARITY_ATTEMPT1_RESULT.md`

## Decision

Candidate B passes the corrected activation and privacy-footprint diagnostic and is authorized to proceed to the already-preregistered external value gate.

This result does **not** authorize merge and is **not** a public performance claim.

## Corrected diagnostic scope

The corrected workflow intentionally traces only the ExecSurface observer process with `strace` and does not follow the tracee child. This avoids the nested-ptrace conflict preserved in Attempt 1 while leaving Candidate B, the fixtures, the activation predicates and the privacy bounds unchanged.

## Activation evidence

### `string-short`

Reference:

- `PTRACE_PEEKDATA`: `13`
- `process_vm_readv`: `0`

Candidate B:

- `PTRACE_PEEKDATA`: `0`
- `process_vm_readv`: `13`

### `fixed-openat2-normal`

Reference:

- `PTRACE_PEEKDATA`: `16`
- `process_vm_readv`: `0`

Candidate B:

- `PTRACE_PEEKDATA`: `0`
- `process_vm_readv`: `14`

The activation requirement is therefore satisfied: Candidate B is exercised and reduces observer-side `PTRACE_PEEKDATA` requests on the controlled semantic workloads.

## Privacy-footprint evidence

- maximum observed C-string iovec length: `8` bytes;
- preregistered maximum for that diagnostic: `8` bytes;
- fixed-size `24` byte metadata batch: observed;
- all activation observations: complete;
- warnings: none;
- command outcomes: exit `0`;
- public runtime source: unchanged by the experiment.

Classification:

`ACTIVATION_AND_PRIVACY_DIAGNOSTIC_PASS`

## Semantic evidence carried forward

Attempt 1 already established before its harness-only diagnostic failure:

- candidate build/test gate PASS;
- dedicated adversarial memory-read parity PASS;
- concurrent production baseline/diff parity PASS in both directions;
- invalid/unreadable cases retained the reference fail-closed behavior.

The Attempt 1 diagnostic failure remains preserved and is not relabeled PASS.

## Next gate

Execute the frozen Candidate B external value experiment on the same pinned targets used by Phase A and A2:

- `BurntSushi/ripgrep@3fce3b5bb0236da2df6d99672afb8a719642eca7`
- `junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Per target and mode:

- same GitHub-hosted Ubuntu 24.04 host class;
- reference and candidate in the same job;
- 3 warmups + 15 measured samples;
- alternating order;
- no outlier removal or sample replacement;
- production semantic prerequisite must pass first;
- Candidate B must reduce median end-to-end ptrace observer wall time by at least `10%` independently on **both** targets.

Until that gate passes, Candidate B remains experimental and unmerged.
