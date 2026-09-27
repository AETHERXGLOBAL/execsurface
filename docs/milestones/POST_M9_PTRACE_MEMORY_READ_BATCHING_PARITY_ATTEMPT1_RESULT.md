# Post-M9 — Candidate B Parity Attempt 1 Result

Date: 2026-09-27
Status: **PARTIAL PASS / DIAGNOSTIC HARNESS FAILURE PRESERVED**
Parent: GitHub issue #84
Source SHA: `cd1aa788c367fc6d02860bf3215fc5a5ab87faac`
Workflow run: `36299043013`
Artifact: `10924404754`
Artifact digest: `sha256:5daeb947a738986483e0e85fc550a4591d988fc824f5ce5bf8136a2c3c44c0ce`

## Result

Candidate B was **not rejected** by this run and is **not yet accepted**.

The run established the following positive evidence before the diagnostic step:

- candidate patch application succeeded;
- clippy/build/tests succeeded;
- dedicated adversarial memory-read parity completed successfully;
- reference/reference determinism and reference/candidate equality held for the exercised deterministic adversarial fixtures;
- intentionally invalid/unreadable fixtures preserved the reference fail-closed/incomplete behavior rather than being relabeled success;
- concurrent production semantic parity passed with zero added/removed/changed findings in both directions under the existing production baseline/diff machinery.

The run failed only in `Activation and privacy-footprint diagnostic` before an activation conclusion could be accepted.

## Preserved diagnostic failure

The diagnostic invoked:

```text
strace -f ... execsurface observe -- <helper>
```

The retained artifact shows the followed child was already under `strace` ptrace control when it attempted the observer's normal `PTRACE_TRACEME` setup:

```text
ptrace(PTRACE_TRACEME) = -1 EPERM (Operation not permitted)
ExecSurface: ERROR
execsurface: observer protocol error: tracee did not enter the expected initial stop
```

This is a nested-ptrace instrumentation conflict introduced by the diagnostic harness. It is not evidence that Candidate B changed runtime semantics, because the same conflict occurred on the frozen reference before Candidate B activation was reached.

## Authorized correction

The correction is limited to the diagnostic harness:

- remove `strace -f` descendant following;
- trace only the ExecSurface observer process, where the observer-side `ptrace(...)` and `process_vm_readv(...)` calls of interest occur;
- keep the same candidate, fixtures, semantic gates, privacy bounds, activation predicates and value threshold;
- do not alter public runtime source or Candidate B merely to make the diagnostic pass.

The failed run and artifact remain authoritative negative harness evidence and must not be deleted or relabeled PASS.

## Next gate

Re-run the unchanged Candidate B semantic/parity workflow with the corrected diagnostic scope. Candidate B may proceed to external value measurement only if activation and privacy-footprint checks then pass in addition to the already-required semantic gates.
