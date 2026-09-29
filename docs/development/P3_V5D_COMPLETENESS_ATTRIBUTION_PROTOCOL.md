# ExecSurface — P3 V5-D Completeness Attribution Diagnostic

Date: 2026-09-29
Tracking: #103
Parent result: `docs/development/P3_V5_STAGE_L_RESULT.md`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED DIAGNOSTIC — NO SAMPLE REPLACEMENT**

## Question

Which raw observer warning class prevents the pinned FZF workload from forming trusted canonical surfaces in V5 Stage L?

This diagnostic cannot replace, rerun, or retroactively admit any V5 learning sample. V5 Stage L remains formally `P3_INCOMPLETE_EVIDENCE`.

## Team

Fixed roles:
- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team

Dynamic specialists:
- Linux ptrace concurrency and clone semantics
- fd-table lifecycle / `CLONE_FILES` semantics
- observer completeness classification
- build-system concurrency
- reproducibility/evidence provenance

## Frozen workload

- `junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`
- GitHub-hosted Ubuntu 24.04
- 3 direct priming runs
- command: `/bin/bash -lc "cd '$WORK_ROOT' && go test ./... >/dev/null 2>&1"`
- one raw observer diagnostic execution after priming

## Required output

Record without canonicalization:
- `Observation.complete`;
- raw event count;
- count of clone-based spawn events;
- every observer warning code, TID and message.

## Classification

- `V5D_SHARED_FD_AMBIGUITY_CONFIRMED` if the observation is incomplete and includes `shared_fd_table_ambiguity`;
- `V5D_OTHER_INCOMPLETENESS_CONFIRMED` if incomplete for another explicit warning class;
- `V5D_MULTI_CAUSE_INCOMPLETENESS` if multiple independent warning classes are present;
- `V5D_NOT_REPRODUCED` if the diagnostic observation is complete.

No classification changes public alpha.4 semantics, removes the guard, or authorizes V5 rerun.
