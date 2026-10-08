# ExecSurface — Post-R8 Merge-Authorization Destruction Record

Date: 2026-10-08  
Parent: #165 / PR #166  
Research scope: runtime behavioral verification / execution semantics / semantic-evidence correctness — **not cybersecurity research**

## Status

**MERGE_AUTHORIZATION_DESTRUCTION_PASS_BOUNDED**

This record is post-R8. It does not rewrite the R7 or R8 preregistrations.

This result closes only the dedicated post-R8 destruction round described here. PR #166 remains Draft and no merge or release is authorized by this record alone.

## Initial discovery — write lineage through rename

During independent merge-authorization destruction, a new composed counterexample was added without changing product code:

`merge_auth_written_object_renamed_onto_report_must_not_be_overwritten`

Fail-first source:

`cc54d34c603d8a221b052ccc026ae8b2e98e28fd`

Observed result:

- expected: ERROR / exit 2 before verdict report materialization;
- actual: REVIEW / exit 10;
- the workload first wrote `WORKLOAD-STATE` to one pathname;
- the workload then renamed that same object onto the selected JSON report pathname;
- ExecSurface subsequently overwrote the workload-written object with its verdict report.

This was a semantic ownership/materialization failure. It was not a cybersecurity finding.

## Minimal cause

The original postflight output guard compared recorded successful fd-write paths only against the final report pathname.

That was insufficient when an object changed pathname later in the same observed execution.

Therefore:

`write(A) -> rename(A, report) -> materialize(report)`

could incorrectly treat `report` as unrelated to the workload-written object.

## First bounded correction

The correction did not add a public schema and did not invent unavailable kernel identity.

For report-materialization protection only it:

1. replays raw observation events in sequence order;
2. tracks absolute paths receiving successful fd writes;
3. propagates tracked paths through later `FileRename { from, to }` transitions with component-safe rewriting;
4. compares verdict outputs against the final propagated workload-mutated paths;
5. retains protected-input object-identity checks.

## Destruction expansion — mutation semantics wider than write(2)

The destruction team then expanded the corpus beyond fd writes and found three additional semantic gaps at source:

`ccb56ac66055e6421c37e6fc0ba70f7e6e37d391`

The corpus result was:

**7 PASS / 3 FAIL**

Failing cases:

- `merge_auth_deleted_report_path_must_not_be_recreated`;
- `merge_auth_created_report_path_without_write_must_not_be_overwritten`;
- `merge_auth_ftruncate_only_report_object_must_not_be_overwritten`.

These showed that report-materialization ownership is broader than successful `write(2)` calls.

The correction therefore also treats observable create/truncate state and preflight/postflight output-state transitions as ownership-relevant for this internal materialization guard.

At source:

`a80a335a982ad7d224accbc117eeab9a899c5780`

the then-current ten-case merge-authorization corpus and all required workflows were GREEN.

## Destruction expansion — verdict-output symlink indirection

The destruction team did not stop after the ten-case corpus passed.

Three additional output-indirection counterexamples were then frozen:

- preexisting broken symlink output;
- workload-created broken symlink output;
- preexisting live symlink output.

After formatting-only cleanup, semantic fail-first evidence was obtained at source:

`58cca0ac26f26b06f812d0f9f7a547a54b24a0eb`

Result:

**10 PASS / 3 FAIL**

The failure proved that `std::fs::write` could follow a verdict-output symlink even when the selected pathname itself did not denote an independently materializable regular file.

The bounded correction rejects verdict output symlinks both at preflight and when the same preflight validation is replayed postflight.

## Destruction expansion — hardlink and non-regular output objects

The destruction team then tested whether a non-symlink output could still alias collateral state or be invalid before target execution.

At source:

`269a4132dab96b0cf3ebe54afcbe576d0e3d3395`

the expanded corpus produced:

**13 PASS / 2 FAIL**

Failing cases:

- `merge_auth_preexisting_hardlinked_report_is_rejected_without_overwriting_peer`;
- `merge_auth_preexisting_directory_report_is_rejected_before_target_execution`.

This showed two additional requirements:

1. an existing verdict output object must not be multiply linked;
2. an existing verdict output must be a regular file, so an invalid materialization target is rejected before the workload runs.

The selected output-materialization contract is now:

> A verdict output path must either be absent, or already designate a non-symlink regular file with link count 1 on the qualified Unix boundary.

This is an internal report-materialization correctness rule. It does not redefine public observation semantics.

## Final hostile corpus

Qualified product source for this destruction round:

`51c6479f8bc5940afa61176526df8eb830ee57f6`

`merge_authorization_destruction.rs`:

**15/15 PASS**

Coverage includes:

1. direct write then rename onto report;
2. chained rename lineage;
3. parent-directory rename lineage;
4. disjoint report positive control;
5. workload-written hardlink then source removal;
6. truncate-only report mutation;
7. ftruncate-only report mutation;
8. delete-only final state;
9. create-without-write final state;
10. untouched preexisting regular-file positive control;
11. preexisting broken symlink output;
12. target-created broken symlink output;
13. preexisting live symlink output;
14. preexisting hardlinked output;
15. preexisting directory output rejected before target execution.

The dedicated hardlink raw-v2 boundary corpus also remains GREEN.

## Exact-source qualification

All required workflows completed SUCCESS on `51c6479f8bc5940afa61176526df8eb830ee57f6`:

- CI — `37722850606` — SUCCESS
- Stage-2 Final Internal Gate — `37722850605` — SUCCESS
- Adversarial Regression — `37722850573` — SUCCESS
- Ptrace Lifecycle Regression — `37722850607` — SUCCESS
- P9.3 Compatibility Contract — `37722850565` — SUCCESS
- Registry Packaging Gate — `37722850570` — SUCCESS
- Public Consumer Smoke — `37722850663` — SUCCESS
- P8 A3.4 consumer contract red team — `37722850614` — SUCCESS
- P8 A3 typed report prototype — `37722850661` — SUCCESS
- P8 A3 typed evidence output — `37722850687` — SUCCESS

The Stage-2 Final Internal Gate replays R1-R8 plus the post-R8 merge-authorization destruction corpus.

## Retained RED history

The following negative evidence is intentionally preserved:

- initial rename-lineage counterexample at `cc54d34c603d8a221b052ccc026ae8b2e98e28fd`;
- mutation-semantic failures at `ccb56ac66055e6421c37e6fc0ba70f7e6e37d391`;
- formatting-only RED runs while new tests were being added;
- symlink semantic failures at `58cca0ac26f26b06f812d0f9f7a547a54b24a0eb`;
- hardlink/non-regular semantic failures at `269a4132dab96b0cf3ebe54afcbe576d0e3d3395`;
- formatting-only RED at `6eac84df757dc88d7a78dbca07504f8e4905c555`.

Formatting-only failures are not counted as semantic falsification evidence.

## Anti-drift decision

Confirmed:

- PR #166 remains Draft;
- Alpha.5 remains immutable historical evidence;
- no R0-R8 acceptance assertion was weakened;
- no release or merge is authorized here;
- no public schema was added for the internal materialization guard;
- project classification remains behavioral/semantic verification, not cybersecurity research.

## Decision

**MERGE_AUTHORIZATION_DESTRUCTION_PASS_BOUNDED**

The destruction round is GREEN at the qualified product source above.

A separate final merge-authorization governance decision is still required against the exact current PR head before any merge into `main`.
