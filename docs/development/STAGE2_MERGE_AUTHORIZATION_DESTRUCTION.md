# ExecSurface — Post-R8 Merge-Authorization Destruction Record

Date: 2026-10-08  
Parent: #165 / PR #166  
Research scope: runtime behavioral verification / execution semantics / semantic-evidence correctness — **not cybersecurity research**

## Status

**MERGE_AUTHORIZATION_RED — NEW SEMANTIC COUNTEREXAMPLE**

This record is post-R8. It does not rewrite the R7 or R8 preregistrations.

## Discovery

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

This is a semantic ownership/materialization failure. It is not a cybersecurity finding.

## Minimal cause

The postflight output guard checks each recorded `FileDescriptorAccess::Write` path against the final report path.

That is insufficient when the written object changes pathname later in the same observed execution.

Current logic retains the historical write pathname but does not propagate written-path lineage through subsequent `FileRename` events.

Therefore:

`write(A) -> rename(A, report) -> materialize(report)`

can incorrectly treat `report` as unrelated to the workload-written object.

## Gate consequence

The previous post-R8 readiness decision is superseded.

PR #166 has been returned to Draft.

No merge or release is authorized while this counterexample remains unresolved.

## Selected bounded correction

Do not add a new public schema and do not invent kernel object identity that raw evidence does not contain.

Instead, for report-materialization protection only:

1. replay raw observation events in sequence order;
2. track absolute paths that received successful fd writes;
3. when a later `FileRename { from, to }` occurs, propagate every tracked written path equal to or beneath `from` onto `to` using component-safe path rewriting;
4. compare report outputs against the final propagated written paths at postflight;
5. retain the existing protected-input object-identity checks.

This corrects path lineage inside the evidence already available.

## Destruction expansion before requalification

The correction must pass at minimum:

1. direct write then rename onto report;
2. write then chained renames onto report;
3. write below a directory then rename the parent directory so the written child becomes the report path;
4. a distinct report path remains usable and does not become a false positive;
5. all R0-R8 corpora remain unchanged and GREEN;
6. all required workflows remain GREEN on the exact corrected head.

Any new counterexample returns the merge-authorization gate to RED.
