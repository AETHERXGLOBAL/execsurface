# ExecSurface — P3 V5 Stage L Incompleteness Diagnostic Result

Date: 2026-09-29
Tracking: #103
Frozen V5 protocol: `docs/development/P3_V5_REAL_WORKLOAD_PROTOCOL.md`
Frozen failed Stage L run: `36559896295`
Diagnostic protocol: `docs/development/P3_V5_STAGE_L_INCOMPLETE_DIAGNOSTIC_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Formal diagnostic classification

`P3_V5_DIAG_SHARED_FD_AMBIGUITY_CONFIRMED`

This result **does not replace or reopen Stage L**. The frozen Stage L result remains:

`P3_INCOMPLETE_EVIDENCE`

All six preregistered learning attempts remain inadmissible and are not rerun or substituted.

## Diagnostic execution

Workflow run:
`36560525523`

Job:
`109380025207`

Source used by the diagnostic:
`615369ab1692176bb00b4a4a9d46d807fd98bcc2`

Pinned workload:
`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

The workflow reproduced the frozen setup sufficiently for attribution:
- current ExecSurface built successfully;
- exactly three direct `go test ./...` priming runs passed;
- one diagnostic `execsurface observe` completed with exit code 0;
- classification, sealing, artifact upload and final classification gate all passed.

## Observed evidence

Diagnostic raw observation:
- observer execution result: `rc=0`;
- `complete=false`;
- event count: **8,601**;
- warning codes: exactly `shared_fd_table_ambiguity` in the diagnostic summary;
- target observation was therefore rejected for trusted canonicalization by current fail-closed semantics.

Artifact:
`11028709084`

Artifact name:
`p3-v5-stage-l-diagnostic-36560525523-1`

Artifact upload digest:
`sha256:6e743ffafe846f1fa68ac78bccea4576c888891411c007c5729e4b3a24386e74`

## Root-cause boundary

The result establishes the immediate cause of the Stage L incompleteness in the declared workload: the current backend handoff sees clone-based concurrency and applies the conservative `shared_fd_table_ambiguity` guard.

Current source marks an observation incomplete when any raw `ProcessSpawn` has `SpawnMechanism::Clone` because raw observation schema v2 does not retain `CLONE_FILES` flags at the backend handoff. This avoids the previously demonstrated false-completeness class.

At the same time, the current ptrace collector internally attempts to read clone/clone3 flags and already uses `CLONE_FILES` to decide whether a child shares the parent fd table or receives a cloned table. That transient internal evidence is not retained as an explicit completeness certificate across the current backend boundary.

Historical discriminator source `bf2835013ffcc1d2d5f8af26d237846bcf44f04c` predates this generic post-collection guard, which explains why historical #62 could produce baselines while the hardened current source fails closed on the same class of concurrent workload.

## Scientific interpretation

The frozen V5 experiment did **not** falsify the V1/V2/V3 variance model. It failed before variance analysis because the current reference observer refused to certify the workload complete.

The diagnostic also does not justify removing the guard. Doing so would recreate an already known false-PASS risk.

The next admissible engineering target is therefore a separate observer-correctness gate: preserve fail-closed behavior on unknown clone/fd-sharing semantics while allowing completeness only when the ptrace collector can positively certify every relevant clone/clone3 fd-table transition.

## Next gate

Open a separate shared-FD completeness-certification gate before any new real-workload requalification campaign.

Requirements for that gate:
- no modification of frozen V5 Stage L history;
- no public alpha.4 integration by default;
- internal proof/certification of clone flag availability and fd-table transition semantics;
- fail closed on unreadable/missing clone flags or lifecycle ambiguity;
- adversarial concurrency/clone/clone3/fd lifecycle tests;
- real fzf diagnostic after synthetic falsification passes;
- new source hash and new preregistered requalification campaign only after observer correctness is separately accepted.
