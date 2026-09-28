# ExecSurface — Semantics v3 S5 Shared-FD Exactness Protocol

Date: 2026-09-29
Tracking: #101
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED BEFORE S5 IMPLEMENTATION**

## Question

Can ExecSurface retain and classify enough spawn/clone evidence to distinguish fd-table relationships more precisely than alpha.4's conservative `shared_fd_table_ambiguity`, without reintroducing the known false-completeness class?

## Existing evidence

The current ptrace implementation already captures `clone` and `clone3` flags transiently in `PendingSyscall::Clone { flags }` and uses `CLONE_FILES` to choose whether a child shares the parent's internal fd-table model.

However raw observation v2 emits only `ProcessSpawn { child_tid, mechanism }`; the clone flags / exact fd-table relation are not retained in the evidence contract. Alpha.4 therefore cannot certify the relation from retained raw evidence and deliberately fails closed under clone-based concurrency.

E4 showed the operational consequence on both JUST and FZF: `shared_fd_table_ambiguity` blocked complete evidence at the first ptrace warmup.

## Immutable boundaries

S5 MUST NOT:
- change public alpha.4;
- remove or narrow the existing alpha.4 ambiguity warning;
- make current raw v2 complete where it is currently incomplete;
- authorize v3 public learn/check/PASS;
- authorize eBPF/hybrid public authority;
- claim all fd-table races are solved merely by retaining `CLONE_FILES`.

## Hypothesis H-S5

For the spawn transition itself, a Linux fd-table relationship can be classified as:

- `shared` when a clone/clone3 relationship is observed with `CLONE_FILES` set;
- `independent_copy` for fork/vfork or clone evidence with known flags and `CLONE_FILES` clear;
- `unknown` when a clone event lacks readable/paired flags.

This classification is necessary but not sufficient for complete fd attribution. Correct close/dup/reuse/exec semantics must still survive adversarial testing.

## S5-A — Pure classification gate

Before changing retained evidence, implement one internal research classifier and test:

1. fork -> `independent_copy`;
2. vfork -> `independent_copy`;
3. clone + `CLONE_FILES` -> `shared`;
4. clone + known flags without `CLONE_FILES` -> `independent_copy`;
5. clone + unavailable flags -> `unknown`.

The production state-machine behavior must remain equivalent:
- shared -> reuse parent fd-table identity;
- independent/unknown -> current copied-table behavior;
- unknown remains fail-closed via existing warning path.

## S5-B — Live retained-evidence prototype

Only after S5-A passes, prototype a research-only spawn evidence record that can retain:
- spawn mechanism;
- whether clone flags were available;
- privacy-safe semantic bits required for fd-table/thread-group interpretation (`CLONE_FILES`, and where required `CLONE_THREAD`), not an unrestricted raw flag dump unless justified;
- derived fd-table relation.

This prototype must not replace raw v2.

## S5-C — Adversarial fixtures

Required cases:

### C0 — no clone control
No fd-table ambiguity invented.

### C1 — real threaded shared fd
Existing `thread-read` fixture (Rust threads) must demonstrate a shared relation where clone flags are available, while alpha.4 remains fail-closed.

### C2 — known clone without `CLONE_FILES`
Construct or synthesize a clone relation with known flags and no `CLONE_FILES`; classifier must not call it shared.

### C3 — flags unavailable
Must classify `unknown` and remain non-admissible for propositions requiring exact fd-table relation.

### C4 — close/dup/reuse under shared relation
Adversarial fixture must try to make one task close/dup/reuse an fd while another task performs a covered read/write. The research model must not attribute through stale fd state.

### C5 — exec from a shared table
Verify the existing copy-on-exec / close-on-exec handling does not mutate sibling tasks' shared table incorrectly.

### C6 — lifecycle/transport loss interaction
Even exact `CLONE_FILES` evidence must not override lifecycle/transport/resource incompleteness.

## Acceptance

S5 can close `PASS_RESTRICTED` only if:

1. classifier tests pass under fmt/clippy/tests;
2. live or executable fixtures cover shared, independent and unknown relationships;
3. close/dup/reuse + exec adversarial cases do not create a false-complete attribution in the declared suite;
4. exact relation evidence is retained in a research-only v3 record;
5. alpha.4 guard remains unchanged;
6. red-team review identifies no unclassified authority laundering path in the declared scope.

Otherwise record the narrow failure:
- `S5_CLASSIFICATION_INSUFFICIENT`;
- `S5_LIVE_CAPTURE_INSUFFICIENT`;
- `S5_FD_LIFECYCLE_COUNTEREXAMPLE`;
- or another explicit evidence-backed result.

## Non-claim

Even an S5 PASS would prove only a bounded improvement in **fd-table relation semantics**. It would not prove whole-observer completeness or justify public v3 promotion by itself.