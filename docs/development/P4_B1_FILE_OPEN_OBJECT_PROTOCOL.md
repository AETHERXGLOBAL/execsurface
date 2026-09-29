# ExecSurface — P4-B1 `P4.FILE.OPEN_OBJECT` Targeted Ptrace Success-Authority Protocol

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Predecessor: `P4_B0_EVIDENCE_CONTRACT_PASS_RESEARCH_ONLY`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — IMPLEMENTATION MAY START ONLY WITHIN THIS BOUNDED RESEARCH GATE**

## Research question

Can the existing ptrace research path establish proposition-scoped successful-open authority for `P4.FILE.OPEN_OBJECT` by causally pairing `open`/`openat`/supported `openat2` entry evidence with syscall-exit result and post-open FD/object binding, without laundering pathname attempts into object authority, without requiring later read/write IO, and without weakening observer completeness or the public raw-v2 contract?

B1 is research-only. It does not authorize a public collector/schema change, backend promotion, release, tag movement, or reinterpretation of alpha.4/v2 evidence.

## Fixed team

1. **Innovation Scientist / Systems Architect** — find the smallest evidence construction that proves successful-open authority while reusing existing ptrace syscall/FD state; explicitly prefer targeted evidence over a broad new collector.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks path-attempt laundering, post-hoc identity weakening, dependency on later IO, test/threshold relaxation, and public-v2 reinterpretation.
3. **Independent Falsifier / Red Team** — attacks entry/exit pairing, actor substitution, PATH-TOCTOU, FD reuse, close/dup/CLOEXEC, clone/shared-FD interaction, openat dirfd identity, openat2 argument uncertainty, loss/truncation, and replay.
4. **Independent Milestone Reviewer** — verifies remote source SHA, frozen inputs, executable tests, preserved failures, deterministic evidence, and allowed decision before closure.

## Dynamic specialists

- Linux ptrace and syscall entry/exit lifecycle specialist
- Linux VFS / `open` / `openat` / `openat2` specialist
- FD lifecycle / close / dup / reuse / CLOEXEC specialist
- filesystem identity / pathname-versus-open-object / TOCTOU specialist
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

No proposition, identity requirement, success condition, or completeness rule may be weakened during B1.

## Frozen success proposition

A B1 successful-open record may satisfy `P4.FILE.OPEN_OBJECT` only if all required evidence is present and mutually consistent:

1. a supported open-family syscall entry is observed;
2. the matching syscall exit belongs to the same actor and originating entry;
3. the exit result is non-negative and therefore yields a returned FD;
4. observation health is complete and warning-free for required dimensions;
5. a post-open FD/object binding is obtained at the open transition, not inferred merely from the pathname argument and not deferred until later read/write IO;
6. actor, target, return, sequence, and causal identities are included in deterministic proof identity;
7. any unresolved FD-table sharing or object-binding ambiguity that can affect the proposition blocks complete success authority.

A pathname or dirfd-resolved pathname remains attempt/intention evidence. It must never by itself become successful-open object authority.

## Bounded object identity hypothesis

B1 may test whether a causally paired post-success descriptor binding (for example a tracee-FD binding obtained immediately after successful return) is sufficient for a **bounded opened-object identity** within the declared ptrace research scope.

B1 must not claim inode-level, mount-stable, race-free kernel object identity unless that stronger identity is directly evidenced and falsified.

If a race-robust post-open binding cannot be established under the frozen criteria, B1 must classify the proposition as incomplete/insufficient rather than weaken the identity requirement.

## Required positive fixtures

At minimum:

1. successful `open` with `rc >= 0`, same actor/entry pairing, healthy observation;
2. successful `openat` with explicit dirfd/path attempt and paired returned FD;
3. supported successful `openat2` case with readable/validated argument structure;
4. successful open followed immediately by close, with no read/write: successful-open evidence must still exist;
5. successful open followed by later covered IO: B1 open authority must not depend on that later IO;
6. deterministic reserialization of the same evidence must produce identical proof identity/digest.

## Required negative and adversarial fixtures

At minimum:

1. failed open (`-errno`) remains failure, never success;
2. entry without matched exit remains attempt/non-success;
3. wrong actor on exit cannot acquire authority;
4. wrong originating entry sequence cannot acquire authority;
5. non-monotonic/replayed/duplicate pairing is rejected or ambiguous;
6. PATH-TOCTOU: lexical pathname substitution cannot establish object identity;
7. post-open FD binding that cannot be obtained or is contradictory blocks success authority;
8. close then FD-number reuse cannot retroactively transfer the original open proof to the replacement object;
9. `dup`/`dup2` aliasing must not rewrite the original opened-object proof identity;
10. CLOEXEC + exec transition must not silently transfer a closed descriptor identity;
11. clone/shared-FD relation unknown or contradictory must block dependent completeness where relevant;
12. known independent FD-table transition must not be treated as shared;
13. `openat` dirfd substitution/mismatch must change or invalidate proof identity;
14. unreadable/truncated/unsupported `openat2` argument state must fail closed;
15. observer warning, resource truncation, or `complete=false` yields `Lost`/incomplete rather than success;
16. actor or causal-chain substitution must change proof identity or block authority;
17. forged returned-FD/object binding mismatch must be rejected;
18. successful open with no subsequent IO must remain representable if all B1 evidence is otherwise complete.

## Anti-drift invariants

- No use of later IO as a prerequisite for successful-open authority.
- No pathname-attempt -> object-success promotion.
- No broad `$TMP`, cache, filesystem, or process whitelist.
- No absence of evidence interpreted as evidence of absence.
- No second backend is introduced in B1.
- No public/raw-v2 bytes or semantics are changed.
- No alpha.4 M11 fail-closed guard is weakened.
- No failed fixture is removed, replaced, or reclassified merely to obtain PASS.
- No `allow`/lint suppression may hide an evidence-contract defect.

## Implementation boundary

B1 implementation must remain under research-only paths, preferably `experiments/p4-backend-authority` plus a dedicated GitHub Actions workflow. Any live ptrace harness used for falsification must be isolated from the public/default observer path.

No `crates/` modification is authorized by this protocol. If implementation proves that a live collector change is unavoidable even to test the hypothesis, stop and record `P4_B1_INCOMPLETE_FOR_SAFE_PTRACE_AUTHORITY` before proposing a separately preregistered integration experiment.

## Kill criteria

B1 must stop with an insufficient/incomplete decision if any of the following is true under the frozen tests:

- successful-open authority requires pathname-only identity;
- object binding requires later IO;
- FD reuse can inherit stale authority;
- actor/entry/causal substitution can preserve the same success proof;
- incomplete/lost observation can still produce success;
- `openat2` uncertainty is silently treated as certainty;
- shared-FD uncertainty can be ignored to obtain completeness;
- a race-robust enough post-open binding for the declared bounded proposition cannot be demonstrated;
- public alpha.4 semantics would need to be weakened or reinterpreted.

## Required reproof

Before B1 can close, the accepted workflow must also re-prove:

- B0 success-evidence contract: 18/18 PASS;
- A2 matrix: 16/16 PASS;
- A1 attempt/adversarial/mapping contracts at their accepted counts;
- Semantics v3: 7/7 PASS;
- public M11 shared-FD contract: 6/6 PASS;
- immutable alpha.4/stable-tag source boundary.

## Allowed B1 decisions

- `P4_B1_FILE_OPEN_OBJECT_PASS_BOUNDED_RESEARCH_ONLY`
- `P4_B1_FILE_OPEN_OBJECT_INCOMPLETE_EVIDENCE`
- `P4_B1_FILE_OPEN_OBJECT_FALSE_AUTHORITY_PATH_FOUND`
- `P4_B1_INCOMPLETE_FOR_SAFE_PTRACE_AUTHORITY`

Only the bounded PASS decision may authorize preregistration of B2. It still does **not** authorize public integration or a second backend.
