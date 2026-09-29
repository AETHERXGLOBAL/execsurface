# ExecSurface — Semantics v3 S5 Shared-FD Exactness Result

Date: 2026-09-29
Tracking: #101
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Execution source: `0294fdd63ea7968c6527eb135f94cf778f3f1f04`
Protocol: `docs/development/SEMANTICS_V3_S5_SHARED_FD_PROTOCOL.md`

## Formal result

**S5 CLOSED — PASS_RESTRICTED**

S5 establishes a bounded improvement in fd-table relationship semantics. It does not establish whole-observer completeness, public Semantics v3 readiness, ptrace/eBPF equivalence, or authority for a new public release.

## Accepted evidence

Accepted workflow:
- `Semantics v3 shared-FD experiment CI`
- run `36493951241`
- source `0294fdd63ea7968c6527eb135f94cf778f3f1f04`
- conclusion: SUCCESS

The accepted run contained two independent jobs:

### Classifier job

- formatting: PASS
- clippy with `-D warnings`: PASS
- classifier tests: **10 passed / 0 failed**

The executable classifier suite proved the declared relation vocabulary:
- fork -> independent copy;
- vfork -> independent copy;
- clone with `CLONE_FILES` -> shared;
- clone with known flags and no `CLONE_FILES` -> independent copy;
- unavailable clone flags -> unknown;
- `CLONE_THREAD` without `CLONE_FILES` does not launder shared-fd authority;
- only the shared relation reuses the parent fd-table identity;
- the retained research record exposes only the semantic clone bits required by the declared scope;
- unknown retained evidence does not invent semantic bits.

### Live ptrace adversarial job

- workspace formatting: PASS
- `execsurface-observe` clippy with `-D warnings`: PASS
- live shared-FD adversarial tests: **6 passed / 0 failed**

The live suite covered:
1. real clone-based threading remains fail-closed under public v2;
2. known clone without `CLONE_FILES` reaches the live ptrace state machine with readable clone flags but public v2 remains fail-closed because those flags are not retained in raw v2;
3. no-clone control does not invent shared-fd ambiguity;
4. shared fd close/reopen/reuse/`dup2` replacement is attributed to the replacement path and not stale old-path state;
5. thread-originated exec preserves the inherited fd identity in the declared fixture while public v2 remains fail-closed;
6. resource/event truncation remains incomplete regardless of exact fd-table relation evidence.

## Adversarial C0–C6 disposition

- **C0 no-clone control — PASS.** No false ambiguity introduced.
- **C1 real threaded shared fd — PASS_RESTRICTED.** Live clone flags are available internally; alpha.4 remains fail-closed because v2 does not retain relation evidence.
- **C2 known clone without `CLONE_FILES` — PASS.** It is not classified as shared; the live child retains inherited fd identity under copied-table semantics.
- **C3 flags unavailable — PASS for executable relation model.** It classifies `unknown` and does not gain exact-relation authority. No claim is made that every live unavailable-flags mechanism has been exhaustively induced.
- **C4 close/dup/reuse — PASS in declared fixture.** The shared-table fixture closes the old descriptor, opens a replacement, uses `dup2` when required, then reads through the reused fd number. Evidence follows the replacement path and does not retain the stale old path.
- **C5 exec from shared table — PASS_RESTRICTED.** The declared thread-originated exec fixture preserves the inherited fd identity and keeps public v2 incomplete; no claim is made for every Linux exec/de-thread edge case.
- **C6 lifecycle/transport/resource loss — PASS for resource truncation prerequisite.** Exact fd-table relation cannot override explicit event-budget incompleteness. Wider lifecycle/transport loss semantics remain governed by S2/S3 and later red-team gates.

## Red-team / authority-laundering review

The declared S5 suite was reviewed specifically for false authority promotion. No unclassified authority-laundering path was found within the declared scope.

The review preserves these boundaries:
- `CLONE_FILES` relation evidence says only that the fd table is shared at the spawn transition; it does not prove later fd attribution is globally complete;
- known absence of `CLONE_FILES` supports independent-copy relation only for the retained spawn transition;
- missing flags remain `unknown`;
- resource/lifecycle/transport incompleteness dominates relation exactness where required;
- pathname evidence is not upgraded to kernel-object identity;
- raw v2 remains raw v2 and is not reinterpreted as v3;
- no backend gains authority merely from being kernel/eBPF-based.

## Preserved failures

Earlier CI failures in the S5 construction sequence remain part of the record. They were not deleted or relabeled. The final accepted run followed formatting/fixture corrections without weakening semantic assertions, completeness rules, or thresholds.

## Scientific interpretation

The E4 blocker is now more precisely localized:

**A material part of alpha.4 `shared_fd_table_ambiguity` is an evidence-retention/contract limitation, not an inability of the current ptrace state machine to observe clone flags.**

The state machine already receives enough information in the tested paths to distinguish shared vs known-independent fd-table transitions. Public raw v2 discards the decisive relation evidence, so its conservative fail-closed guard remains correct and unchanged.

This result supports Semantics v3 retaining proposition-scoped fd-table relation evidence, but does not authorize replacing the alpha.4 guard until later compatibility, integration, and red-team gates close.

## Authority after S5

Unchanged public authority:
- public release: `v0.1.0-alpha.4`;
- public/default correctness-reference backend: ptrace;
- public raw/canonical/baseline semantics: v2;
- eBPF/hybrid: research-only;
- no public v3 `learn` / `check` / PASS;
- no cross-backend baseline interchangeability;
- no public promotion from S5 alone.

## Next authorized gate

**S6 — Cross-Backend Proposition Mapping**

S6 must map only propositions that each existing backend can actually support, preserve incomparability where evidence differs, and prohibit whole-surface equivalence or scalar backend-quality scoring.
