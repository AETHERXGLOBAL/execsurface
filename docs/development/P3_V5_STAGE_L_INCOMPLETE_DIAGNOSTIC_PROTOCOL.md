# ExecSurface — P3 V5 Stage L Incompleteness Diagnostic Protocol

Date: 2026-09-29
Tracking: #103
Frozen parent protocol: `docs/development/P3_V5_REAL_WORKLOAD_PROTOCOL.md`
Failed Stage L run: `36559896295`
Artifact: `11028623425`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **DIAGNOSTIC ONLY — NOT A REPLACEMENT LEARNING RUN**

## Reason for this diagnostic

The frozen V5 protocol requires exactly six trusted learning attempts and forbids replacement of inadmissible samples.

Stage L executed all six attempts after the required three direct priming runs. Every `execsurface learn` attempt returned exit code 2, produced no lockfile, and reported:

`raw observation is incomplete and cannot form a trusted canonical surface`

The Stage L classification is therefore retained as:

`P3_INCOMPLETE_EVIDENCE`

This diagnostic does **not** reopen, replace, rerun or relabel those six samples. Its sole purpose is to identify the current observer warning(s) that made the real workload incomplete.

## Historical/current semantic delta already identified

Historical discriminator source `bf2835013ffcc1d2d5f8af26d237846bcf44f04c` did not contain a generic shared-fd ambiguity guard at the backend handoff.

Current development source applies `apply_shared_fd_ambiguity_guard` after ptrace collection. If any raw `ProcessSpawn` uses `SpawnMechanism::Clone`, and no prior ambiguity warning exists, current code marks the observation incomplete with warning code:

`shared_fd_table_ambiguity`

The warning states that raw v2 does not retain `CLONE_FILES` flags, so shared-fd lifecycle attribution cannot be certified complete for the session.

Separately, the current ptrace collector already reads clone/clone3 flags internally when available and uses `CLONE_FILES` to choose whether a child shares or clones the parent fd table. That internal fact does not by itself authorize removing the fail-closed guard because raw v2 does not preserve the proof needed at the backend handoff.

## Frozen diagnostic workload

Use the same source and target command as Stage L:
- `junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`
- GitHub-hosted `ubuntu-24.04`
- Rust `1.90.0` to build current ExecSurface
- exactly three direct `go test ./...` priming runs
- one subsequent `execsurface observe` of:
  `/bin/bash -lc "cd '$WORK_ROOT' && go test ./... >/dev/null 2>&1"`

The observation is diagnostic evidence only and cannot enter the V5 learning set.

## Required evidence

Retain:
- full raw observation JSON;
- observation `complete` value;
- warning codes/messages;
- event count;
- target outcome;
- environment and exact source revisions;
- sealed SHA-256 manifest.

## Diagnostic classifications

Exactly one classification should be emitted:
- `P3_V5_DIAG_SHARED_FD_AMBIGUITY_CONFIRMED`
- `P3_V5_DIAG_EVENT_LIMIT_CONFIRMED`
- `P3_V5_DIAG_OTHER_INCOMPLETENESS`
- `P3_V5_DIAG_UNEXPECTED_COMPLETE`
- `P3_V5_DIAG_OBSERVER_EXECUTION_FAILED`

No classification changes the frozen Stage L result. Any observer hardening refinement must be opened as a separate correctness gate with its own adversarial tests before V5 can be attempted under a new preregistered program state.
