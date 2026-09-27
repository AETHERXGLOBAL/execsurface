# Post-M9 — Ptrace Fast-Path A2 Split Semantic Parity v5 Protocol

Date: 2026-09-27
Status: **PREREGISTERED BEFORE v5 EXECUTION**
Parent: GitHub issue #84
Candidate: A2
Prior result: `docs/milestones/POST_M9_PTRACE_FASTPATH_A2_PARITY_V4_RESULT.md`
Raw comparator authority: `docs/milestones/POST_M9_PTRACE_FASTPATH_PARITY_COMPARATOR.md`
Production parity authority: `docs/milestones/POST_M9_PTRACE_FASTPATH_PARITY_METHOD.md`

## Purpose

Complete the semantic eligibility gate without weakening either comparison model.

v4 established exact raw parity for twelve reference-deterministic adversarial cases, then proved that the reference itself has valid raw event-order/interleaving variance on an eight-thread read fixture. v5 therefore separates:

- **Lane A:** strict raw parity where raw reference determinism is empirically valid; and
- **Lane B:** unchanged existing production canonical/baseline semantics for the intentionally concurrent fixture.

This split is frozen before execution. It does not introduce a candidate-specific normalizer or retroactively convert the v4 raw failure into a raw PASS.

## Lane A — strict raw parity

The comparator remains exactly:

- local numeric TID identity may be renamed consistently;
- flattened `process_spawn.child_tid` uses the same local TID map;
- warning TIDs use the same local TID map;
- event count exact;
- event order exact;
- event sequence exact;
- every non-TID event field exact;
- backend, warnings, completeness and target outcome exact.

Lane A cases:

1. `irrelevant`
2. `fileio`
3. `failed-open`
4. `signal`
5. `restart`
6. `restart-single`
7. `fork`
8. `vfork`
9. `exec`
10. `failed-exec`
11. `thread-exec`
12. `exit-group`
13. `fixture-spawn`

For every Lane A case:

- reference/reference must match first;
- only then reference/A2 may be compared;
- any mismatch blocks v5.

No raw sorting or interleaving normalization is allowed in Lane A.

## Lane B — concurrent production semantic parity

Workload:

`execsurface-fixture thread-read <fixed-path> 8`

The v4 raw-order failure remains preserved. Lane B does **not** claim raw equality.

Lane B acceptance requires all of the following using the same fixed path/content and same helper/fixture binary:

### Health/outcome

For independent reference and A2 observations:

- complete = true;
- warnings = 0;
- exit code/signal identical;
- backend identity/capability declaration unchanged.

### Reference production stability

Create a reference baseline with the current public `learn` implementation and run at least two subsequent reference `check --diff-only --json` executions of the same thread-read workload.

Both must be comparable and produce:

- added = 0;
- removed = 0;
- changed = 0.

This proves the existing production canonicalizer/diff contract treats the reference interleaving variance as semantically stable.

### Cross-implementation parity

Require both directions:

1. reference baseline -> A2 check = zero semantic findings;
2. A2 baseline -> reference check = zero semantic findings.

No policy is used to suppress differences. The comparison is `--diff-only`.

### Canonicalizer immutability

v5 may not modify:

- `execsurface-normalize`;
- normalization profile/version;
- baseline schema/digest rules;
- diff rules;
- policy/verdict rules;
- semantic roots merely to hide candidate differences.

Any required change to these invalidates v5 and requires a new protocol.

## Activation gate after Lane A + Lane B

Only if both semantic lanes pass:

Run the frozen synthetic `getpid` activation diagnostic.

For `+5000` added `getpid` syscalls:

- reference `PTRACE_GET_SYSCALL_INFO` delta must be `10000`;
- A2 delta must be `5000`;
- observations must remain complete and warning-free.

This is diagnostic evidence that the intended exit-side request reduction is active. It is not a production speed claim.

## Value gate remains separate

v5 does not evaluate the external performance threshold.

Only after v5 passes may the already-preregistered value experiment run on pinned `ripgrep` and `fzf` with:

- 3 warmups + 15 measured samples;
- alternating reference/candidate order;
- no clean sample exclusion;
- same runner/job per target;
- exact health/outcome/canonical acceptance;
- candidate median ptrace wall time lower on both targets;
- at least `10%` median ptrace wall-time reduction on each target.

## Failure classification

- Lane A reference/reference mismatch: `REFERENCE_RAW_NONDETERMINISM / GATE BLOCKED`.
- Lane A reference/A2 mismatch: `CANDIDATE_RAW_SEMANTIC_MISMATCH`.
- Lane B reference canonical instability: `REFERENCE_PRODUCTION_SEMANTIC_INSTABILITY`.
- Lane B cross-implementation findings: `CANDIDATE_PRODUCTION_SEMANTIC_MISMATCH`.
- activation count mismatch: `CANDIDATE_MECHANISM_NOT_ESTABLISHED`.

All failures remain preserved.

## Authority boundary

A v5 PASS establishes only **semantic eligibility for the separately frozen value measurement**. It does not merge A2, change the public runtime, change ptrace/eBPF authority, create a release, or establish a public performance claim.
