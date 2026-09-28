# Post-M9 — eBPF E4 Same-Workload Value Requalification Result

Date: 2026-09-29
Tracking: GitHub issue #85
Development branch: `development/post-alpha4-behavioral-integrity`
Execution source: `3c21521efd7b0f150d3785993916562c908f2f2c`
Workflow run: `36490724891`

## Formal result

**CLOSED — `E4_BLOCKED_BY_INCOMPLETE_EVIDENCE`**

The preregistered E4 protocol was executed without changing target revisions, commands, warmup/sample counts, the 10% threshold, health gates, authority boundaries, or sample-retention rules.

Both pinned targets were blocked during the first `ptrace` warmup because alpha.4 correctly reported incomplete evidence with `shared_fd_table_ambiguity`.

No measured 15-sample timing set was admitted. No failed sample was replaced. No rerun was used to seek a cleaner outcome.

This result does **not** establish that persistent eBPF is faster or slower than ptrace on these targets. It establishes that the value comparison is inadmissible under the current completeness contract because the public ptrace reference could not certify complete shared-FD lifecycle attribution on the tested concurrent workloads.

## Frozen protocol

Protocol: `docs/milestones/POST_M9_EBPF_E4_SAME_WORKLOAD_VALUE_PROTOCOL.md`

- 3 warmups per mode;
- 15 measured samples per mode;
- direct / ptrace / persistent-eBPF-two-session modes;
- no outlier removal;
- no sample replacement;
- health/completeness before timing acceptance;
- 10% per-target value threshold;
- no cross-target averaging;
- no public eBPF authority regardless of timing outcome.

The only pre-execution plumbing change was extending the existing workflow push trigger to the isolated development branch. The experiment and thresholds were unchanged.

## E4-FZF

Target:

`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Command:

`go test ./... >/dev/null 2>&1`

Direct warmups completed successfully:

- 455.265010 ms
- 422.508049 ms
- 434.319590 ms

First ptrace warmup:

- target exit: `0`;
- signal: none;
- backend: `linux-ptrace-metadata-v2`;
- observed event count: `8555`;
- `complete=false`;
- warning: `shared_fd_table_ambiguity`;
- wall time: `1950.578072 ms`;
- target classification: `E4_BLOCKED_BY_INCOMPLETE_EVIDENCE`.

The warning states that clone-based concurrency was observed and raw v2 does not retain `CLONE_FILES` flags, so shared-FD lifecycle attribution cannot be certified complete for the session.

Evidence artifact:

- ID: `11001308264`
- name: `post-m9-ebpf-e4-E4-FZF-36490724891-1`
- digest: `sha256:611038a250936981d478d808c04e748b6389ed13c7602a76b085ab6a19fb69d1`

## E4-JUST

Target:

`casey/just@5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Command:

`cargo +1.90.0 test --all >/dev/null 2>&1`

Direct warmups completed successfully:

- 3670.339684 ms
- 3601.484327 ms
- 3626.344707 ms

First ptrace warmup:

- target exit: `0`;
- signal: none;
- backend: `linux-ptrace-metadata-v2`;
- observed event count: `214283`;
- `complete=false`;
- warning: `shared_fd_table_ambiguity`;
- wall time: `13343.190487 ms`;
- target classification: `E4_BLOCKED_BY_INCOMPLETE_EVIDENCE`.

The same alpha.4 conservative shared-FD ambiguity guard prevented complete evidence certification.

Evidence artifact:

- ID: `11001373432`
- name: `post-m9-ebpf-e4-E4-JUST-36490724891-1`
- digest: `sha256:4c3e64ed009fb29e998a72de43b6881d4f3ded994dd541754f9b0000d0a242dd`

## Scientific interpretation

This is a useful negative result, not an infrastructure failure.

The target commands themselves exited successfully. Environment setup, workload priming, public ptrace CLI build, unchanged M8.7 persistent observer build, and barrier-adapter build all completed successfully for both targets. The gate stopped because the evidence contract rejected incomplete ptrace observations.

The result reinforces the existing alpha.4 boundary:

- fail-closed semantics are functioning as designed;
- clone-based concurrency can cause conservative false incompleteness because raw observation v2 does not retain enough `CLONE_FILES` information;
- incomplete evidence must not be treated as timing-admissible complete evidence;
- weakening/removing the guard merely to obtain benchmark numbers is prohibited.

## Authority decision

Unchanged:

- native ptrace remains the public/default correctness-reference backend;
- eBPF remains research-only;
- eBPF `learn` is not authorized;
- eBPF `check` is not authorized;
- eBPF PASS authority is not authorized;
- backend auto-selection is not authorized;
- ptrace/eBPF baseline interchangeability is not authorized;
- `full_surface_comparable=false`;
- no bounded eBPF performance value claim was established by E4.

## Next research consequence

Do **not** rerun E4 until the evidence model itself changes under an independently gated semantics/authority program.

The blocker is now an input to the post-alpha.4 Semantics v3 program: model exact or proposition-scoped shared-FD authority explicitly, preserve ambiguity where it cannot be proved, and test whether stronger observer evidence can remove false incompleteness without reintroducing false completeness.

This result closes P1 of the post-alpha.4 advancement program and authorizes P2 design work. It does not authorize any public alpha.4 change.