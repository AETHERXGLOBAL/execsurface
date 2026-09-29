# ExecSurface — P4-B1 Ptrace Successful-Open Object Authority Protocol

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Predecessor: `P4_B0_EVIDENCE_CONTRACT_PASS_RESEARCH_ONLY`
Accepted B0 source: `2246f7b5016fd7e17ceeeaeaa5ff35b53cd99b95`
Accepted B0 workflow: `36593660113`
Accepted B0 artifact SHA-256: `36a254686911a13d6040a9936efa3a8b8a25c9a5b74eb7efe39cbf338eb81e1a`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — B1 IMPLEMENTATION MAY START**

## Scientific question

Can a Linux ptrace research observer establish bounded `P4.FILE.OPEN_OBJECT` authority from `open`/`openat`/supported `openat2` by causally pairing syscall entry with syscall exit and binding the returned file descriptor to a kernel-grounded opened-object identity at the open transition, without relying on later covered I/O and without laundering pathname intent into object authority?

## Fixed team

1. **Innovation Scientist / Linux Runtime Architect** — find the smallest proof-bearing ptrace mechanism that closes only the successful-open object gap.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks pathname-as-object laundering, success from unpaired exits, public-v2 reinterpretation, broad collector work, and post-hoc acceptance changes.
3. **Independent Falsifier / Red Team** — attacks entry/exit pairing, actor substitution, fd reuse, shared fd tables, PATH-TOCTOU, CLOEXEC/exec, observer loss, replay, and object-identity races.
4. **Independent Milestone Reviewer** — verifies source SHA, frozen tests, live Linux execution, exact evidence artifact, public isolation and allowed decision before closure.

## Dynamic specialists

- Linux ptrace syscall-entry/exit specialist
- Linux VFS/open/openat/openat2 specialist
- file-descriptor lifecycle and `/proc/<pid>/fd` specialist
- inode/device/object-identity specialist
- clone/thread/shared-fd concurrency specialist
- TOCTOU/adversarial filesystem specialist
- formal semantics / proof-obligation specialist
- CI reproducibility and evidence-sealing reviewer

## Frozen proposition

Only:

`P4.FILE.OPEN_OBJECT`

B1 does not evaluate rename/delete or connect success authority, and no result from B1 may be inherited by B2 or B3.

## Frozen authority states

A B1 observation must end in exactly one research state:

- `AttemptObserved`
- `SuccessObjectBounded`
- `FailureObserved`
- `Ambiguous`
- `Lost`

`SuccessObjectBounded` is stronger than pathname attempt authority but remains explicitly bounded research evidence. It is not public-v2 authority and does not imply universal file-operation completeness.

## Required success evidence

`SuccessObjectBounded` requires all of the following in one causally bound observation:

1. supported open-family syscall entry is observed;
2. originating actor/TID identity is retained;
3. entry arguments are retained/digested and normalized under the frozen research profile;
4. the matching syscall exit is observed for the same actor and entry identity;
5. syscall return is non-negative and interpreted as the returned fd;
6. while the relevant tracee state is still controlled at the exit transition, the returned fd is resolved to an opened-object identity;
7. object identity contains at minimum a kernel-derived `(st_dev, st_ino, file_type)` tuple or an equivalently strong explicit identity source;
8. the observation is complete and warning-free for all dependencies used by the proposition;
9. fd-table relation is known sufficiently to rule out authority transfer through unresolved shared-fd close/reuse;
10. evidence serialization and digest are deterministic.

A lexical pathname or `/proc/<pid>/fd/<fd>` pathname string by itself is insufficient for `SuccessObjectBounded`.

## Identity rule

The primary object identity is the kernel metadata of the opened fd, not the original pathname text.

A pathname may be retained only as contextual evidence. Hard links or rename after open must not change the object identity if the same opened object remains bound.

If the observer cannot obtain a stable kernel-derived object identity at the bounded observation point, the state is `Ambiguous`, not success.

## Shared-fd rule

If another thread/process may share the relevant fd table and the accepted research evidence cannot prove a safe fd-table relation for the open-to-object-binding interval, B1 must fail closed as `Ambiguous`/`Lost`.

The existing C1/C6R shared-fd certificate may be reused as a research prerequisite if it is causally bound and revalidated. B1 must not weaken the public M11 `shared_fd_table_ambiguity` contract.

## Positive fixtures

At minimum:

1. successful `open` of a regular file;
2. successful `openat(AT_FDCWD, ...)`;
3. successful `openat` using a real directory fd;
4. supported `openat2` case where available in the runner kernel;
5. successful open followed immediately by close with no covered read/write — success object evidence must still exist;
6. open of the same inode through a hard link — pathname differs, object identity matches;
7. open followed by pathname rename while fd remains open — object identity remains stable;
8. deterministic repeated serialization/digest for identical evidence.

## Negative / falsification fixtures

At minimum:

1. `ENOENT` never becomes success;
2. reproducible permission/semantic open failure never becomes success;
3. generic negative return never becomes success;
4. entry without matching exit cannot become success;
5. exit without matching entry cannot become success;
6. mismatched actor/TID blocks pairing;
7. mismatched entry sequence blocks pairing;
8. duplicate/replayed exit cannot inflate authority;
9. fd close + numeric fd reuse cannot transfer prior object identity;
10. pathname substitution/TOCTOU cannot cause lexical path identity to override opened-object identity;
11. object replaced between independent opens must produce different kernel identity when the kernel object differs;
12. `O_CLOEXEC` + exec must not retroactively erase the already-proved open transition, but any post-exec fd claim must respect CLOEXEC;
13. unresolved shared-fd close/reuse interaction must fail closed;
14. clone without sufficient fd-table evidence must fail closed for dependent object binding;
15. observer warning/loss/truncation blocks success authority;
16. unreadable `/proc/<pid>/fd/<fd>` or failed object metadata acquisition at the bounded binding point cannot be treated as success-object authority;
17. pathname-only evidence remains `AttemptObserved`;
18. backend/profile name substitution alone cannot upgrade authority.

## Live Linux requirement

B1 must include a live Linux ptrace harness in the isolated research experiment. Model-only fixtures are insufficient for closure.

The live harness must demonstrate at least:

- a successful open-family syscall entry/exit pair;
- returned-fd extraction from the tracee syscall result;
- object metadata binding from the returned fd while the tracee is in a controlled ptrace state;
- successful-open-with-immediate-close;
- at least one failed open;
- at least one fd reuse adversarial case;
- at least one hard-link or rename identity case;
- an explicit fail-closed shared-fd/concurrency case or a causally valid certificate proving the relation.

## Required regression reproof

The accepted B1 run must also reprove:

- B0 success-evidence contract;
- A2 authority-gap matrix;
- A1 attempt-authority and adversarial mapping corpus;
- Semantics v3;
- public M11 shared-fd contract;
- public alpha.4/stable tags unchanged;
- no public/raw-v2/default-observer semantic change.

## Kill / escalation criteria

B1 is not allowed to add a second backend merely because ptrace implementation is inconvenient.

A second-backend prototype for `P4.FILE.OPEN_OBJECT` becomes eligible only if live B1 evidence shows that the required object identity cannot be obtained or safely bound under ptrace within the frozen correctness criteria, and the failure is retained with a concrete missing-evidence statement.

## Allowed B1 decisions

- `P4_B1_PTRACE_OPEN_OBJECT_AUTHORITY_ESTABLISHED_BOUNDED`
- `P4_B1_PTRACE_OPEN_OBJECT_PARTIAL_GAP_REMAINS`
- `P4_B1_SECOND_BACKEND_PROTOTYPE_JUSTIFIED_BOUNDED`
- `P4_B1_FALSE_AUTHORITY_PATH_FOUND`
- `P4_B1_INCOMPLETE_EVIDENCE`

Only the first outcome closes the A2 `P4.FILE.OPEN_OBJECT` gap for the bounded research proposition. None of these outcomes authorize public integration or release.

## Public isolation

B1 must not modify:

- `main`;
- `v0.1.0-alpha.4`;
- stable `v0.1`;
- public raw-v2 schema/bytes/meaning;
- default observer behavior;
- public learn/check/baseline/policy semantics.

Any later integration requires a separate compatibility and release gate.