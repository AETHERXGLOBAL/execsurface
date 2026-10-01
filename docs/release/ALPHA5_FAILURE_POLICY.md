# Alpha.5 failure policy

Every failed workflow/run is evidence and must remain discoverable.

A failure is classified before correction as one of:
- SCIENTIFIC: invariant or acceptance condition violated; blocks promotion and requires design/code correction.
- HARNESS: runner/workflow/fixture defect prevented the intended test from executing; preserve run, correct wrapper only, rerun unchanged scientific assertions.
- ENVIRONMENT: prerequisite unavailable; classify unsupported/blocked, never PASS.
- DISTRIBUTION: packaging/install/tag/action mismatch; blocks release even if runtime science passes.

Forbidden responses to failure:
- weakening assertions or thresholds;
- deleting/replacing negative evidence;
- changing test data to an easier distribution;
- claiming PASS from a partially executed gate;
- treating unsupported as absence or success.
