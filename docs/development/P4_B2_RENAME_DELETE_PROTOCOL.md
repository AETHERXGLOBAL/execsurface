# ExecSurface — P4-B2 `P4.FILE.RENAME_DELETE` Targeted Ptrace Success-Authority Protocol

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Predecessor: `P4_B1_FILE_OPEN_OBJECT_PASS_BOUNDED_RESEARCH_ONLY`
Accepted B1 result commit: `d51267e032b547582cc690b0c47c1ce8fb8d3ab7`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — IMPLEMENTATION MAY START ONLY WITHIN THIS BOUNDED RESEARCH GATE**

## Research question

Can targeted ptrace syscall-entry/syscall-exit evidence establish bounded successful-effect authority for `P4.FILE.RENAME_DELETE` by proving that the exact originating rename/delete operation completed with `rc=0`, while keeping pathname evidence as attempt identity and never laundering a failed, mismatched, replayed, ambiguous, or lost operation into success?

B2 is research-only. It does not authorize public raw-v2 reinterpretation, collector promotion, a second backend, release movement, or `crates/` modification.

## Fixed team

1. **Innovation Scientist / Linux VFS Runtime Architect** — seek the smallest ptrace proof that closes only the successful rename/delete result gap.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks pathname-attempt laundering, rc semantics weakening, post-hoc flag scope expansion, public-v2 reinterpretation, test relaxation, and backend-first implementation.
3. **Independent Falsifier / Red Team** — attacks actor/entry/source/target substitution, replay, syscall-family confusion, failed operations, rename flags, path replacement, observer loss, restart/interruption, and evidence digest collisions.
4. **Independent Milestone Reviewer** — verifies source SHA, frozen fixtures, live syscall evidence, reproof counts, public isolation, artifact digest, and allowed decision before closure.

## Dynamic specialists

- Linux ptrace syscall entry/exit lifecycle specialist
- Linux VFS rename/renameat/renameat2 specialist
- unlink/unlinkat/rmdir semantics specialist
- dirfd/path-resolution specialist
- filesystem race / PATH-TOCTOU specialist
- restartable syscall / EINTR specialist
- formal semantics / proposition-authority specialist
- deterministic evidence / serialization reviewer
- CI evidence-engineering reviewer

## Frozen inputs

- `docs/development/P4_B_TARGETED_SUCCESS_AUTHORITY_PROTOCOL.md`
- `docs/development/P4_B1_FILE_OPEN_OBJECT_RESULT.md`
- `docs/development/P4_B0_SUCCESS_EVIDENCE_RESULT.md`
- `docs/development/P4_A2_AUTHORITY_GAP_RESULT.md`
- canonical research model: `experiments/p4-backend-authority`
- immutable alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

No proposition, operation scope, success condition, failure semantics, fixture, test count, or acceptance threshold may be weakened after execution begins.

## Frozen operation scope

B2 may classify only these syscall families when their required arguments are actually observed and bound:

- `rename`
- `renameat`
- `renameat2`
- `unlink`
- `unlinkat`
- `rmdir`

For `renameat2`, flags are part of the operation identity. Unknown/unreadable flags are not success-authoritative. `RENAME_NOREPLACE` and `RENAME_EXCHANGE` may be classified only when the live harness explicitly captures the flags and the fixture verifies the corresponding semantics. Unsupported or unclassified flags fail closed.

## Frozen successful-effect contract

`P4.FILE.RENAME_DELETE` may become bounded successful-effect authority only if all of the following hold:

1. a supported rename/delete syscall entry is observed;
2. the entry carries the complete operation identity required by the specific syscall: actor, syscall family, dirfd/path arguments, and flags when applicable;
3. a matching syscall exit is causally paired to the exact entry and actor;
4. the exit raw return is exactly `0`;
5. observation health is complete and warning-free for every dependency used by the proposition;
6. the canonical operation-context digest equals the argument digest frozen at entry;
7. source/target substitution, actor substitution, operation-family substitution, or replay changes identity or blocks authority;
8. failure, ambiguity, unsupported state, or observer loss remains non-success;
9. deterministic serialization and proof identity are stable.

A pathname attempt is still an attempt until a causally matched `rc=0` exit proves the bounded operation result.

## Required positive model fixtures

At minimum:

1. `rename` with exact source/target context and `rc=0`;
2. `renameat` with exact source/target dirfd/path context and `rc=0`;
3. supported `renameat2` context with explicit flags and `rc=0`;
4. `unlink` with exact target context and `rc=0`;
5. `unlinkat` with exact dirfd/path/flags context and `rc=0`;
6. `rmdir` with exact target context and `rc=0`;
7. identical evidence deterministically rebuilds identical proof identity and bytes.

## Required negative/adversarial model fixtures

At minimum:

1. `ENOENT` remains failure;
2. `EEXIST`/`ENOTEMPTY`/permission or other reproducible semantic failure remains failure;
3. any negative return remains failure;
4. unexpected positive return for a zero-success operation is ambiguous, not success;
5. entry without matched exit is attempt-only;
6. wrong actor/TID blocks success;
7. wrong originating entry sequence blocks success;
8. source substitution changes proof identity or blocks context validation;
9. target substitution changes proof identity or blocks context validation;
10. rename-vs-delete operation-family substitution is rejected;
11. duplicate/replayed pairing is rejected;
12. non-monotonic entry/exit ordering is ambiguous;
13. unknown/unreadable `renameat2` flags fail closed;
14. unknown/unreadable `unlinkat` flags fail closed;
15. observer warning or `complete=false` yields lost/incomplete;
16. success authority for rename/delete cannot satisfy `FILE.OPEN_OBJECT` or network-connect propositions;
17. backend/profile name substitution alone cannot upgrade authority;
18. context serialization/digest remains deterministic.

## Mandatory live Linux ptrace gate

**Model-only evidence is insufficient for B2 closure.**

The accepted B2 workflow must execute a live Linux x86_64 ptrace harness and demonstrate from actual traced syscalls:

- at least one successful rename-family operation with matched `rc=0`;
- at least one successful delete-family operation with matched `rc=0`;
- `rename` source/target entry arguments bound to the successful exit;
- `renameat` or `renameat2` dirfd/path context bound when exercised;
- one failed rename/delete operation retained as failure;
- one source or target substitution/replay attack that cannot preserve success authority;
- one rename flag case (`RENAME_NOREPLACE` or `RENAME_EXCHANGE`) when supported by the current runner kernel, otherwise explicit unsupported classification;
- observer incompleteness/fail-closed behavior represented in the model reproof;
- filesystem postcondition checks only as test corroboration, never as a substitute for the paired syscall result.

The live harness must not infer success from later filesystem state alone.

## Cross-gate anti-drift invariants

- no entry-only success;
- no negative-return success;
- no positive-return success for zero-success operations;
- no pathname existence/nonexistence used as substitute success proof;
- no actor/entry/source/target substitution;
- no replay authority inflation;
- no unknown flags silently treated as known;
- no observer-loss success;
- no broad backend implementation;
- no `crates/` changes;
- public M11 fail-closed contract remains unchanged;
- alpha.4/stable tags remain pinned to the immutable source;
- no failed fixture is removed, replaced, or weakened.

## Kill / escalation criteria

B2 closes incomplete or failed if:

- ptrace cannot reliably pair the required rename/delete entry and exit under the frozen scope;
- required source/target/flags context cannot be bound to the successful result;
- a failed/mismatched/replayed operation can become success;
- a sufficiently bounded success result would require public alpha.4 semantic weakening;
- the live harness can only infer success from post-hoc filesystem state rather than the syscall result;
- observer loss can still produce success authority.

Only a concrete proposition-specific missing-evidence result after this gate may make a tiny second-backend prototype eligible for `P4.FILE.RENAME_DELETE`.

## Required reproof before closure

The accepted B2 workflow must re-prove:

- B2 model corpus at the frozen accepted count;
- B1 hardened model: **24/24 PASS**;
- B1 mandatory live ptrace corpus: **10/10 PASS** or an immutable accepted B1 result/source check plus equivalent boundary reproof;
- B0: **18/18 PASS**;
- A2: **16/16 PASS**;
- A1.3U: **5/5 PASS**;
- A1 adversarial: **18/18 PASS**;
- A1.2 mapping: **10/10 PASS**;
- Semantics v3: **7/7 PASS**;
- public M11: **6/6 PASS**;
- immutable alpha.4/stable-tag source boundary.

## Allowed B2 decisions

- `P4_B2_FILE_RENAME_DELETE_PASS_BOUNDED_RESEARCH_ONLY`
- `P4_B2_FILE_RENAME_DELETE_INCOMPLETE_EVIDENCE`
- `P4_B2_FILE_RENAME_DELETE_FALSE_AUTHORITY_PATH_FOUND`
- `P4_B2_INCOMPLETE_FOR_SAFE_PTRACE_AUTHORITY`

Only bounded PASS may authorize preregistration of B3. It does not authorize public integration or a second backend.