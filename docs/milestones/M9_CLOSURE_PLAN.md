# M9 Project Closure Plan

Status: **ACTIVE / EVIDENCE-GATED**

This plan defines the final closure path for ExecSurface from the current public alpha and M9 hardening state to a release decision supported by real-workload and independent-adoption evidence.

## Non-negotiable authority

- `ptrace` remains the public correctness reference backend.
- eBPF remains research-only unless a separately authorized evidence gate reopens it.
- No `ESRCH`, `ECHILD`, lifecycle, completeness, or event-budget failure may be suppressed merely to obtain PASS.
- Baseline, diff, policy and report semantics are not weakened to improve compatibility or performance.
- Existing failures and counterexamples remain preserved.
- `ZERO_CONTACT_EXTERNAL_REPRO` is compatibility/performance evidence, not independent adoption.
- `main` receives a hardening change only after the unchanged semantic gates and the pinned external counterexample pass.

## Closure team

### Fixed roles

1. **Innovation Scientist / Systems Architect**
   - seeks the smallest lifecycle-correct design;
   - challenges unnecessary architecture expansion;
   - keeps product value tied to runtime execution-surface drift.

2. **Deviation Prevention / Scientific Integrity Lead**
   - prevents threshold lowering, evidence relabeling and history rewriting;
   - enforces PROVED / PARTIAL / OPEN / KILLED status discipline;
   - verifies that negative evidence remains visible.

3. **Independent Red Team**
   - attempts to make every proposed lifecycle fix lose a tracee, misreport root outcome, corrupt fd state, or silently convert incomplete evidence into PASS;
   - does not author the candidate it reviews.

### Dynamic specialists for the remaining work

4. **Linux ptrace / Process-Lifecycle Specialist**
   - owns fork/clone/thread-group/exec/exit identity correctness;
   - validates `PTRACE_EVENT_EXEC`, `PTRACE_GETEVENTMSG`, TID/TGID transitions and terminal wait semantics.

5. **Runtime Semantics Engineer**
   - verifies descendant completeness, causal process identity, root exit/signal outcome, fd-table semantics and event-budget fail-closed behavior.

6. **Rust Reliability Engineer**
   - converts the proved lifecycle model into the smallest maintainable Rust change and deterministic regression coverage.

7. **CI / Release Engineer**
   - runs unchanged workspace, M6.5 and M7 gates;
   - preserves artifacts and hashes;
   - prepares the next signed release and stable Action update only after acceptance.

8. **External Workload Validation Lead**
   - expands zero-contact tests across pinned, unmodified real projects;
   - maintains the frozen M9 performance protocol.

9. **Independent Adoption / Evidence Lead**
   - collects third-party initiated/executed evidence under the frozen M9 schema;
   - keeps adoption claims separate from AETHER X-run reproductions.

10. **Public Surface / Documentation Lead**
    - synchronizes public claims, limitations, installation paths, release notes and adopter guidance with accepted evidence.

## Current blocker

The canonical pinned `casey/just` workload reproduced the public-alpha observer failure fail-closed. Investigation progressed beyond the original raw `ESRCH`: a diagnostic candidate reached outer `waitpid(..., __WALL) -> ECHILD` while internal tracee state still contained live-looking entries.

Current conclusion: **OPEN — lifecycle bookkeeping defect; blanket ESRCH suppression is KILLED.**

The active candidate now tracks thread-group identity and reconciles Linux exec collapse semantics. It is not accepted until the gate proves it.

## Closure gates

### C1 — ptrace lifecycle closure

Acceptance requires all of the following:

- exact lifecycle condition explained;
- deterministic/minimized regression where practical;
- no stale tracee state at terminal ECHILD;
- root exit/signal semantics preserved;
- descendant/event/fd semantics preserved;
- pinned `casey/just` observation succeeds repeatedly;
- failures remain fail-closed.

Outcome: `PROVED` or `KILLED / REWORK REQUIRED`.

### C2 — core integration

Only after C1 = PROVED:

- commit the minimal product fix, not diagnostic-only code;
- run formatting, clippy, workspace tests and existing semantic gates unchanged;
- run M6.5 and M7 regression/evidence gates unchanged;
- Red Team review the final diff;
- merge by normal history into `main`; no force push or history rewrite.

### C3 — M9.1 real-workload evidence

- rerun the same pinned `casey/just` workload under the frozen canonical M9 protocol;
- preserve pre-fix and post-fix evidence;
- complete exactly 3 warmups and 15 measured samples per mode with alternating order;
- expand the frozen external workload cohort without modifying upstream projects;
- record failures as failures; do not convert zero-contact reproductions into adoption claims.

### C4 — M9.2 independent adoption

Acceptance evidence must satisfy the frozen `INDEPENDENT_USER` provenance rules: third party initiated, third party executed, no material AETHER X project modification, and non-empty attestation.

### C5 — M9.3 synthesis and release decision

- summarize compatibility, performance, correctness and independent-adoption evidence;
- decide the next release scope from evidence;
- keep eBPF research-only unless its separate trigger and authority gate are actually satisfied.

## Public distribution surfaces to synchronize after accepted release

Verified current public product surfaces:

1. GitHub repository / README: `AETHERXGLOBAL/execsurface`
2. GitHub Releases: current public alpha line
3. crates.io: `execsurface`
4. Stable GitHub Action channel: `AETHERXGLOBAL/execsurface@v0.1`
5. GitHub Issue #28: public Early Adopters call

A prior LinkedIn ExecSurface draft exists in project history, but publication is **not verified** by the repository evidence available to this closure plan. It must not be treated as an already-published channel without a verifiable post reference.

## Public-update rule

Do not publish a stronger reliability claim before C1-C3 evidence is accepted. After acceptance, update all verified public surfaces from one release evidence summary so version, limitations, install commands, compatibility statements and claims remain consistent.

## Definition of project closure

ExecSurface is not considered closed merely because the core compiles or a single external workload passes. Closure requires:

- C1 lifecycle correctness closed;
- C2 merged core with unchanged regression gates passing;
- C3 accepted real-workload evidence;
- C4 independent-user evidence or an explicit documented decision that independent adoption remains open;
- C5 evidence synthesis and release decision;
- public product surfaces synchronized with the accepted state.
