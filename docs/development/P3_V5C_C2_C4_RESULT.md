# ExecSurface — P3 V5-C C2–C4 Certification Bridge Result

Date: 2026-09-29
Tracking: #104
Parent P3: #103
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — V5C_CERTIFICATION_PATH_ESTABLISHED_BOUNDED**

## Scope

This result closes only the research-only shared-FD completeness certification bridge. It does not modify the frozen failed V5 Stage L campaign, public `v0.1.0-alpha.4`, raw/canonical/baseline v2 semantics, stable `@v0.1`, or release claims.

The frozen V5 Stage L outcome remains `P3_INCOMPLETE_EVIDENCE` and its six failed learning attempts are never rerun, replaced, or relabeled.

## Team

Fixed roles retained throughout:
- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team

Dynamic specialists used for C2–C4:
- Linux ptrace child-creation event semantics
- clone/clone3, `CLONE_FILES`, `CLONE_THREAD`, `CLONE_VFORK`
- fd-table lifecycle / exec / CLOEXEC / dup / close / reuse
- Rust state-machine correctness
- Go build concurrency
- reproducibility / CI evidence engineering

## C2 — internal certificate implementation

An internal, non-public `CloneFdCertification` was added behind the ptrace backend boundary. It records:
- child-creation transitions whose origin is clone/clone3;
- causally correlated clone flags;
- shared fd-table transitions (`CLONE_FILES`);
- independent copied fd-table transitions;
- `CLONE_THREAD` transitions;
- ambiguity state.

Positive certification requires every relevant clone-origin transition to be causally correlated and classified exactly once. Missing/unreadable evidence remains fail-closed.

Public raw schema v2 was not extended.

## C3 — falsification and event-routing correction

The red team identified a material defect in the first certificate design: Linux ptrace event labels are not equivalent to originating syscall identity. A clone-origin call can be reported as `PTRACE_EVENT_FORK` or `PTRACE_EVENT_VFORK` depending on clone flags / exit-signal semantics.

The first attempted hardening therefore was not accepted as final.

### Preserved failures

- run `36562783087`: all technical tests passed, but final push was rejected non-fast-forward because the development branch advanced concurrently. No force push or history rewrite was used.
- run `36563013194`: clone→FORK and clone→VFORK adversarial tests passed, but full `execsurface-observe` regression failed because the candidate silently changed public raw-v2 `ProcessSpawn.mechanism` from historical event-label semantics to syscall-origin semantics. This candidate was rejected.

The anti-drift correction separated the two truths:
- **internal certificate authority follows originating syscall evidence**;
- **public raw-v2 `ProcessSpawn.mechanism` remains event-label based forever**.

### Accepted C2/C3 run

Workflow run: `36563263708`

Accepted implementation commit:
`ed442deb162426aa30a6d30788ae6bb6fbc53b22`

Accepted gates:
- rustfmt PASS;
- clippy `-D warnings` PASS;
- live clone-origin routed as FORK falsifier PASS;
- live clone-origin routed as VFORK falsifier PASS;
- ordinary private/shared clone live harness PASS;
- full `execsurface-observe` regression PASS;
- full workspace regression PASS;
- public legacy guard boundary check PASS.

Formal C2/C3 result:

`C2C3_CERTIFICATE_SURVIVES_BOUNDED`

No public guard suppression is authorized by C2/C3 alone.

## C4 — pinned FZF internal certificate diagnostic

Pinned workload:
`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Host class:
GitHub-hosted Ubuntu 24.04

Execution source:
`a8020ff071e79fa6000272b851f7c30b46808105`

Workflow run:
`36563705496`

Artifact:
`11031150503`

Artifact digest:
`sha256:d1df08921c06f7f872ea75884895a39a3bc9b8c03333f1b0c68eae9302482d69`

Protocol execution:
- exactly 3 direct priming runs: PASS;
- exactly 1 diagnostic-only internal ptrace observation;
- no V5 learning sample was created or replaced.

Observed internal evidence before the legacy post-collection guard:
- raw events: **9,327**;
- observation complete: **true**;
- warnings: **0**;
- clone-origin transitions: **214**;
- correlated clone flags: **214/214**;
- shared fd-table transitions: **90**;
- independent copied-table transitions: **124**;
- `CLONE_THREAD` transitions: **90**;
- clone origins routed through FORK-style ptrace events: **104**;
- clone origins routed through VFORK-style ptrace events: **20**;
- internal certificate: **fully certified**.

Diagnostic marker:
`C4_FZF_INTERNAL_PASS`

Formal C4 result:

`C4_PINNED_FZF_INTERNAL_CERTIFICATE_PASS_BOUNDED`

### Preserved C4 plumbing failure

Run `36563571730` failed before any FZF measurement because the pinned Rust 1.90 toolchain did not yet have the rustfmt component installed. No priming or diagnostic sample was executed. The follow-up changed only toolchain-component setup and retained all scientific acceptance criteria.

## Scientific interpretation

The original FZF V5 failure is now more precisely localized.

The current ptrace state machine can, on the pinned workload and declared host class, retain enough causally paired clone/clone3 information to certify fd-table sharing semantics internally with zero unknown transitions and zero independent observer-health warnings.

The public alpha.4 false-incompleteness was therefore caused by a boundary mismatch: public raw v2 discards the proof needed by the conservative post-collection shared-FD guard.

This does **not** prove universal ptrace completeness, universal clone coverage, production readiness, or whole-surface equivalence with another backend.

## C5 decision

`V5C_CERTIFICATION_PATH_ESTABLISHED_BOUNDED`

This authorizes only the next isolated gate:

**C6 — Certificate-Aware Legacy Guard Integration / False-Completeness Red Team**

C6 may test suppressing only the generic post-collection `shared_fd_table_ambiguity` warning when the internal certificate is positively complete. It must remain fail-closed when certification is absent, incomplete, contradictory, or when any independent observer-health warning exists.

C6 must preserve:
- public raw-v2 bytes and interpretation;
- all other warning/incompleteness semantics;
- PATH-TOCTOU boundary;
- event/resource-loss fail-closed behavior;
- frozen failed V5 campaign;
- public alpha.4 unchanged.

Only after C6 survives independent falsification may a **newly preregistered** V5 requalification campaign be created from a new source hash.
