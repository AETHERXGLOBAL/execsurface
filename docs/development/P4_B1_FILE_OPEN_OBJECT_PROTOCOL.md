# ExecSurface — P4-B1 `P4.FILE.OPEN_OBJECT` Targeted Ptrace Success-Authority Protocol

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Predecessor: `P4_B0_EVIDENCE_CONTRACT_PASS_RESEARCH_ONLY`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — IMPLEMENTATION MAY START ONLY WITHIN THIS BOUNDED RESEARCH GATE**

This file is the **single canonical B1 protocol**. A parallel preliminary protocol created during concurrent execution is retired after its stricter identity/live-test requirements are consolidated here; Git history preserves that evidence.

## Research question

Can the existing ptrace research path establish proposition-scoped successful-open authority for `P4.FILE.OPEN_OBJECT` by causally pairing `open`/`openat`/supported `openat2` entry evidence with syscall-exit result and binding the returned file descriptor to a kernel-grounded opened-object identity at the open transition, without laundering pathname attempts into object authority, without requiring later read/write IO, and without weakening observer completeness or the public raw-v2 contract?

B1 is research-only. It does not authorize a public collector/schema change, backend promotion, release, tag movement, or reinterpretation of alpha.4/v2 evidence.

## Fixed team

1. **Innovation Scientist / Linux Runtime Architect** — find the smallest proof-bearing ptrace mechanism that proves successful-open object identity while reusing existing syscall/FD state.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks path-attempt laundering, identity weakening, dependency on later IO, test/threshold relaxation, duplicate protocol drift, and public-v2 reinterpretation.
3. **Independent Falsifier / Red Team** — attacks entry/exit pairing, actor substitution, PATH-TOCTOU, FD reuse, close/dup/CLOEXEC, clone/shared-FD interaction, openat dirfd identity, openat2 uncertainty, loss/truncation, replay, and object-identity races.
4. **Independent Milestone Reviewer** — verifies remote source SHA, frozen inputs, executable live tests, preserved failures, deterministic evidence, artifact digest, and allowed decision before closure.

## Dynamic specialists

- Linux ptrace and syscall entry/exit lifecycle specialist
- Linux VFS / `open` / `openat` / `openat2` specialist
- FD lifecycle / close / dup / reuse / CLOEXEC specialist
- filesystem inode/device/object identity specialist
- pathname-versus-open-object / TOCTOU specialist
- clone/shared-FD concurrency specialist
- formal semantics / proof-obligation specialist
- reproducibility / deterministic evidence reviewer
- CI evidence-engineering reviewer

## Frozen inputs

- `docs/development/P4_B_TARGETED_SUCCESS_AUTHORITY_PROTOCOL.md`
- `docs/development/P4_B0_SUCCESS_EVIDENCE_RESULT.md`
- `docs/development/P4_A2_AUTHORITY_GAP_RESULT.md`
- `docs/development/P4_A1_3_ADVERSARIAL_RESULT.md`
- `docs/development/P4_A1_4_PUBLIC_ANTIDRIFT_RESULT.md`
- canonical research model: `experiments/p4-backend-authority`
- accepted B0 source: `2246f7b5016fd7e17ceeeaeaa5ff35b53cd99b95`
- immutable alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

No proposition, identity requirement, success condition, completeness rule, test inventory, or acceptance threshold may be weakened during B1.

## Frozen success proposition

A B1 successful-open record may satisfy `P4.FILE.OPEN_OBJECT` only if all required evidence is present and mutually consistent:

1. a supported open-family syscall entry is observed;
2. the matching syscall exit belongs to the same actor and originating entry;
3. the exit result is non-negative and therefore yields a returned FD;
4. observation health is complete and warning-free for every dependency used by the proposition;
5. a post-open FD/object binding is obtained at the open transition, not inferred from pathname text and not deferred until later read/write IO;
6. actor, target, return, entry/exit sequence, FD generation, and causal identities participate in deterministic proof identity;
7. any unresolved FD-table sharing or object-binding ambiguity that can affect the proposition blocks complete success authority;
8. object identity is kernel-derived and stronger than a pathname string.

A pathname or dirfd-resolved pathname remains attempt/intention evidence. It must never by itself become successful-open object authority.

## Kernel object identity requirement

For the accepted live B1 path, `SuccessObjectBounded` requires an object identity obtained from the successfully returned descriptor while the relevant tracee state is ptrace-controlled at the syscall-exit transition.

The minimum accepted identity is a kernel-derived tuple equivalent to:

- `st_dev`;
- `st_ino`;
- file type (`st_mode & S_IFMT` or equivalent).

A `/proc/<pid>/fd/<fd>` symlink string alone is contextual evidence and **cannot** satisfy this identity requirement. It may be retained in addition to the kernel metadata.

If kernel metadata cannot be obtained at the bounded binding point, or can be raced because the FD-table relation is unresolved, the result is `Ambiguous`/`Lost`, not successful-object authority.

Hard links or pathname rename after open must not change the already-bound object identity when the same opened kernel object remains referenced.

## Shared-FD rule

If another task may share the relevant FD table and accepted research evidence cannot prove a safe relation for the interval from successful return through object binding, B1 must fail closed.

The accepted C1/C6R shared-FD certificate may be reused only if it is causally bound and revalidated for B1. B1 must not weaken the public M11 `shared_fd_table_ambiguity` contract.

## Required positive fixtures

At minimum:

1. successful `open` with `rc >= 0`, same actor/entry pairing, healthy observation;
2. successful `openat(AT_FDCWD, ...)`;
3. successful `openat` with an actual directory FD;
4. supported successful `openat2` case with readable/validated arguments, or explicit kernel unsupported classification if unavailable;
5. successful open followed immediately by close, with no read/write — authority must still derive from the open transition;
6. successful open followed by later covered IO — B1 authority must not depend on that IO;
7. same inode opened through a hard link — different pathname, same kernel object identity;
8. open followed by pathname rename while FD remains open — object identity remains stable;
9. deterministic reserialization of identical evidence produces identical proof identity/digest.

## Required negative and adversarial fixtures

At minimum:

1. `ENOENT` remains failure;
2. reproducible permission/semantic failure remains failure;
3. generic negative return remains failure;
4. entry without matched exit remains non-success;
5. exit without matching entry remains non-success;
6. wrong actor/TID blocks pairing;
7. wrong originating entry sequence blocks pairing;
8. non-monotonic/replayed/duplicate pairing is rejected or ambiguous;
9. PATH-TOCTOU: lexical pathname substitution cannot establish object identity;
10. object replacement between independent opens changes kernel identity when the kernel object changes;
11. close then numeric FD reuse cannot inherit the prior object identity;
12. `dup`/`dup2` aliasing cannot rewrite the original open proof identity;
13. `O_CLOEXEC` + exec cannot retroactively erase the proved open transition, while any post-exec FD claim must respect CLOEXEC;
14. clone/shared-FD relation unknown or contradictory blocks dependent object completeness;
15. known independent FD-table transition must not be treated as shared;
16. `openat` dirfd substitution/mismatch changes or invalidates proof identity;
17. unreadable/truncated/unsupported `openat2` argument state fails closed;
18. observer warning, resource truncation, or `complete=false` yields `Lost`/incomplete;
19. forged returned-FD/object binding mismatch is rejected;
20. inability to obtain kernel object metadata at the binding point yields ambiguous/incomplete, never success;
21. backend/profile name substitution alone cannot upgrade authority.

## Mandatory live Linux ptrace gate

**Model-only fixtures are insufficient for B1 closure.**

The accepted B1 workflow must execute a live Linux x86_64 ptrace harness on the pinned CI environment and demonstrate, from actual traced syscalls:

- open-family syscall entry and exit pairing;
- returned-FD extraction from the tracee syscall result;
- kernel object metadata binding from the returned FD while the tracee is in a controlled ptrace stop;
- immediate-close-without-IO success retention;
- at least one failed open;
- at least one numeric FD reuse attack;
- at least one hard-link or pathname-rename identity case;
- an explicit shared-FD/concurrency fail-closed case or a causally revalidated certificate proving the required relation.

A workflow that passes only `b1_open_object` model tests must record at most `P4_B1_MODEL_CONTRACT_PASS_RESEARCH_ONLY / LIVE_PTRACE_REQUALIFICATION_STILL_REQUIRED` and **cannot close B1**.

## Anti-drift invariants

- no later IO prerequisite for successful-open authority;
- no pathname-attempt -> object-success promotion;
- no path-only object identity;
- no broad filesystem/process whitelist;
- no absence of evidence interpreted as evidence of absence;
- no second backend in B1;
- no public/raw-v2 bytes or semantics changed;
- no alpha.4 M11 fail-closed guard weakened;
- no failed fixture removed, replaced, or reclassified merely to obtain PASS;
- no lint/test suppression used to hide a contract defect.

## Implementation boundary

B1 implementation remains under isolated research paths, preferably `experiments/p4-backend-authority` plus a dedicated workflow.

No `crates/` modification is authorized by this protocol. If a live test requires a public/runtime crate change, stop and record `P4_B1_INCOMPLETE_FOR_SAFE_PTRACE_AUTHORITY` before proposing a separately preregistered integration experiment.

## Kill criteria

B1 must stop insufficient/incomplete if any of the following is true:

- pathname-only identity is required for success;
- object binding requires later IO;
- FD reuse can inherit stale authority;
- actor/entry/causal substitution can preserve the same success proof;
- incomplete/lost observation can still produce success;
- `openat2` uncertainty is silently treated as certainty;
- shared-FD uncertainty is ignored to obtain completeness;
- a sufficiently race-bounded kernel object binding cannot be demonstrated under ptrace;
- public alpha.4 semantics would need weakening or reinterpretation.

A second-backend prototype for this proposition becomes eligible only if live B1 retains a concrete proposition-specific missing-evidence result under these frozen criteria.

## Required reproof

Before B1 can close, the accepted workflow must also re-prove:

- B0 success-evidence contract: 18/18 PASS;
- A2 matrix: 16/16 PASS;
- A1 attempt/adversarial/mapping contracts at accepted counts;
- Semantics v3: 7/7 PASS;
- public M11 shared-FD contract: 6/6 PASS;
- immutable alpha.4/stable-tag source boundary.

## Allowed B1 decisions

- `P4_B1_FILE_OPEN_OBJECT_PASS_BOUNDED_RESEARCH_ONLY`
- `P4_B1_FILE_OPEN_OBJECT_INCOMPLETE_EVIDENCE`
- `P4_B1_FILE_OPEN_OBJECT_FALSE_AUTHORITY_PATH_FOUND`
- `P4_B1_INCOMPLETE_FOR_SAFE_PTRACE_AUTHORITY`

Only the bounded PASS decision may authorize preregistration of B2. It still does **not** authorize public integration or a second backend.