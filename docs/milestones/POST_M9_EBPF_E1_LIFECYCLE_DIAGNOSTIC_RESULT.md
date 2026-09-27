# Post-M9 — eBPF E1 Lifecycle Diagnostic Result

Date: 2026-09-27
Status: **E1 CLOSED — HISTORICAL INTERMITTENT LIFECYCLE FAILURE NOT REPRODUCED / OLD COLLECTOR ARCHITECTURE REMAINS DISQUALIFIED**
Tracking: GitHub issue #85
Source SHA: `1b3daa83798f3475da4fd4576b87e4d2dd198cb5`
Workflow run: `36300797733`

## Decision

The six frozen diagnostic repetitions did **not** reproduce `incomplete_lifecycle`:

- `casey/just`: 3/3 lifecycle-drain complete;
- `junegunn/fzf`: 3/3 lifecycle-drain complete;
- dropped events: `0` in all six runs;
- target outcome: exit `0` in all six runs;
- completeness: `incomplete_capability` in all six runs, which the frozen M9.1 harness does not classify as a hard lifecycle failure.

Therefore E1 does **not** authorize a timeout increase, tracker cleanup heuristic, or any other speculative repair of the old per-invocation collector.

The historical M9.1 `incomplete_lifecycle` failures remain valid negative evidence and are not relabeled or erased. They are classified as intermittent failures that were not reproduced by this preregistered six-run diagnostic.

Independently, static architecture evidence remains decisive: the old M8.4d collector used by M9.1 relies on syscall-exit descendant membership propagation, while M8.7 had already rejected that boundary and established a stronger lifecycle discipline based on session epochs, a root-registration barrier and `sched_process_fork` propagation. The old collector is therefore not an acceptable basis for renewed performance qualification merely because this small diagnostic sample happened to be clean.

## Frozen diagnostic protocol

Targets:

1. `casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
   - `cargo +1.90.0 test --all >/dev/null 2>&1`
2. `junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`
   - `go test ./... >/dev/null 2>&1`

Per target:

- 3 repetitions;
- original `2000 ms` lifecycle timeout;
- no timeout threshold change;
- no event-loss or lag injection;
- no public collector source change;
- runner-only metadata diagnostic instrumentation;
- no performance-value measurement.

The runner-only diagnostic would have emitted remaining task identities and `/proc/<tid>/status` metadata only if the normal lifecycle timeout branch was reached. It was never reached in any accepted repetition.

## `casey/just` evidence

Artifact:

- ID: `10925149811`
- digest: `sha256:d3cb40bed68ba57f129b1ead448bd7e4b18275e6bdfda6cb566988ac117d152f`

Runs:

1. event count `52882`, lifecycle complete, dropped `0`, `incomplete_capability`;
2. event count `52945`, lifecycle complete, dropped `0`, `incomplete_capability`;
3. event count `52926`, lifecycle complete, dropped `0`, `incomplete_capability`.

Aggregate classification: `NO_TIMEOUT_REPRODUCED`.

## `junegunn/fzf` evidence

Artifact:

- ID: `10925267846`
- digest: `sha256:47073c39c2caab249e9e00fa76ea331a785c0a86a9c83393717e583b4207a31e`

Runs:

1. event count `4276`, lifecycle complete, dropped `0`, `incomplete_capability`;
2. event count `4294`, lifecycle complete, dropped `0`, `incomplete_capability`;
3. event count `4292`, lifecycle complete, dropped `0`, `incomplete_capability`.

Aggregate classification: `NO_TIMEOUT_REPRODUCED`.

## Relation to historical M9.1 evidence

Historical evidence remains unchanged:

- `just` previously produced `incomplete_lifecycle`, lifecycle drain false, dropped `0`, event count `52813`, and paid approximately the explicit 2-second drain timeout;
- `fzf` previously produced one clean lifecycle run and then one `incomplete_lifecycle` run with dropped `0`.

The new diagnostic shows only that those failures are intermittent on the current GitHub-hosted environment under the frozen workload revisions. It does not prove the old collector lifecycle design correct.

## E1 conclusion

### PROVED

- `incomplete_capability` is not the M9.1 hard blocker.
- The historical value screen was blocked by intermittent lifecycle-drain failure.
- The old value screen used the pre-M8.7 per-invocation collector.
- M8.7 had already superseded the old child-membership propagation boundary.
- The six-run diagnostic did not reproduce the timeout.

### OPEN

The exact task identity responsible for the historical timeout remains unknown because no timeout occurred during the preregistered diagnostic window.

### KILLED

- increasing the lifecycle timeout as a repair;
- inferring that clean 3/3 runs make the old collector lifecycle architecture acceptable;
- changing capability semantics merely to make value measurement admissible.

## E2 direction

Do not patch the killed syscall-exit membership architecture.

E2 should construct a **research-only value-requalification collector** by carrying forward the lifecycle discipline already established in M8.7:

- kernel-side session epoch membership;
- root-registration barrier before workload release;
- `sched_process_fork` descendant propagation;
- exec/exit continuity;
- explicit routing/drop/decode accounting;
- map cleanup and lifecycle-drain checks;
- stale-epoch fail-closed behavior.

The first E2 gate is lifecycle/health only on the same pinned `just` and `fzf` workloads. No performance timing is authorized until that gate passes repeatedly with complete lifecycle evidence.

## Authority after E1

Unchanged:

- public/default backend: native ptrace;
- eBPF: research-only;
- eBPF `learn/check`: not authorized;
- eBPF PASS authority: not authorized;
- backend auto-selection: not authorized;
- baseline interchangeability: not authorized;
- public eBPF performance claim: not authorized.
