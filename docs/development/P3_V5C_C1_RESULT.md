# ExecSurface — P3 V5-C C1 Real-Workload Clone Relation Census Result

Date: 2026-09-29
Tracking: #104
Parent P3: #103
Protocol: `docs/development/P3_V5C_C0_C1_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — C1_RELATION_OBSERVABLE_ALL_RUNS**

## Team

Fixed roles retained:
- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team

Dynamic C1 specialists:
- Linux ptrace + clone/clone3 semantics
- fd-table / `CLONE_FILES` semantics
- Go build concurrency
- Rust systems implementation
- reproducibility / evidence sealing

## Preserved predecessor result

The original V5 learning campaign remains frozen as `P3_INCOMPLETE_EVIDENCE` and is not replaced by this census.

Independent diagnostic evidence had already attributed that failure to public v2 `shared_fd_table_ambiguity` on the pinned FZF workload. C1 asks only whether the missing fd-table relation is observable under an explicitly syscall-paired research census.

## Frozen workload

- repository: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- host: GitHub-hosted Ubuntu 24.04
- exactly 3 direct priming runs
- exactly 3 census runs, no replacement
- command: `/bin/bash -lc "cd '$WORK_ROOT' && go test ./... >/dev/null 2>&1"`

## Accepted execution

Workflow run:
`36562227407`

Job:
`109385567368`

Execution source:
`8e4b81259c08727e191d8787c745d8d98821f9d8`

Artifact:
`11030691637`

Artifact digest:
`sha256:2aa3d79716adbb72d877f0e2abaa27c69ac2d0b0ea92bbe16554750e52c2694d`

Static gates before measurement:
- Rust 1.90.0 install/components: PASS
- isolated lock generation: PASS
- rustfmt: PASS
- clippy `-D warnings`: PASS
- build: PASS

All 3 direct priming runs completed successfully.

## Census results

### Run 1
- process rc: 0
- root exit: 0
- child-creation transitions: 203
- clone-origin transitions: 203
- `Shared`: 80
- `IndependentCopy`: 123
- `Unknown`: 0
- clone3 flag-read failures: 0
- unpaired creation events: 0
- recorded fork/vfork-origin mismatch count: 0
- transcript SHA-256: `58b82d34cf6586466ae0c2e5e3320379c564ee1eef08addf580a8d97b6e64134`

### Run 2
- process rc: 0
- root exit: 0
- child-creation transitions: 200
- clone-origin transitions: 200
- `Shared`: 77
- `IndependentCopy`: 123
- `Unknown`: 0
- clone3 flag-read failures: 0
- unpaired creation events: 0
- recorded fork/vfork-origin mismatch count: 0
- transcript SHA-256: `087d529cd5eec6d40d0242c11896fb4247083ecce9c45a2b411a5536ed2a87d6`

### Run 3
- process rc: 0
- root exit: 0
- child-creation transitions: 193
- clone-origin transitions: 193
- `Shared`: 72
- `IndependentCopy`: 121
- `Unknown`: 0
- clone3 flag-read failures: 0
- unpaired creation events: 0
- recorded fork/vfork-origin mismatch count: 0
- transcript SHA-256: `ce90f295e57ae7387af97d3989c116e7e64a012ae781028bbdf29a79b39ade90`

Aggregate classification:

`C1_RELATION_OBSERVABLE_ALL_RUNS`

Across the three admitted runs there were 596 clone-origin transitions, including 229 transitions with `CLONE_FILES` set and 367 known-independent copies. No transition was classified `Unknown`.

## Interpretation

C1 establishes a bounded but high-value fact: on this pinned real FZF/Go workload, the tracer can observe and causally pair enough syscall-entry information to distinguish true shared fd-table transitions from known-independent clone transitions without weakening fail-closed semantics.

Therefore the public alpha.4 false-incompleteness on this workload is not evidence that the relationship is intrinsically unobservable. It is consistent with an evidence-retention/certification gap in the public v2 contract.

This result does **not** authorize removal of `shared_fd_table_ambiguity`, public v3 integration, or replay of the frozen V5 samples.

## Red-team caveat discovered during C1 review

The standalone census pairs every child-creation ptrace event to the pending originating syscall. The current internal observer certificate prototype added separately on the development branch records certification on `PTRACE_EVENT_CLONE` specifically.

That difference matters because Linux ptrace event type alone is not a proof of originating syscall semantics. A clone-origin call may be routed through a fork/vfork ptrace event under particular clone flag / exit-signal combinations.

The C1 aggregate field `origin_event_mismatch` currently detects mismatches for explicit fork/vfork origins; it does not by itself prove that every clone-origin transition used `PTRACE_EVENT_CLONE`. Therefore no stronger event-type claim is made from this result.

This caveat becomes a mandatory C2/C3 falsification target.

## Preserved engineering failures

Before the accepted run, two non-scientific workflow failures were retained:
- `36561726992`: execution plumbing/toolchain setup failed before a scientific census result;
- `36561997662`: rustfmt-only gate failure before the census.

Neither failure caused any protocol, threshold, relation definition, or workload change.

## Decision

`C1_RELATION_OBSERVABLE_ALL_RUNS`

C2 collector-side certificate refinement and C3 adversarial certification are authorized.

C2 MUST align certificate truth with originating syscall evidence rather than ptrace event label alone, or prove by adversarial evidence that the distinction cannot affect the declared certificate. Unknown/unpaired origin remains fail-closed.
