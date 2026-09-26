# M9-H1.3 — GET_SYSCALL_INFO residual race status

Status: **CAUSE LOCALIZED / FIX NOT YET AUTHORIZED**

This record preserves the H1.3 diagnostic evidence collected after the child pre-registration ordering candidate had already demonstrated substantial improvement on the pinned `casey/just` workload.

## Frozen workload

- Repository: `casey/just`
- Pinned SHA: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- Rust: `1.90.0`
- Backend under investigation: Linux `ptrace`

## Prior fact entering H1.3

H1.2 had already localized a residual failure to:

`PTRACE_GET_SYSCALL_INFO -> ESRCH`

This occurred after the same TID had been buffered and reconciled through the new child pre-registration path. Therefore H1.3 was not allowed to treat all ESRCH as equivalent to the already-bounded `PTRACE_SYSCALL` restart race.

## Run A — syscall-info lifecycle diagnostic

- Run: `36274203010`, attempt 1
- Head: `95bee9888e630d20f25129b4f14ecf302bf2917c`
- Artifact ID: `10917095325`
- Artifact SHA-256: `345316b3cfcc0eb33fc9b4032968035e136d360375dd66aaa650459d17a5ecf2`

Observed pinned-workload results:

- attempt 1: `rc=0`, `prereg_reconciled=6227`
- attempt 2: `rc=0`, `prereg_reconciled=6216`
- attempt 3: `rc=0`, `prereg_reconciled=6265`
- attempt 4: `rc=0`, `prereg_reconciled=6268`
- attempt 5: `rc=0`, `prereg_reconciled=6259`
- `syscall_info_esrch_localized=0`

The workflow job is marked failed only because its diagnostic acceptance condition required at least one `GET_SYSCALL_INFO ESRCH` observation. No workload attempt failed.

## Run B — post-ESRCH lifecycle classifier

- Run: `36274247640`
- Head: `7fe1add097950b3d33a0d91e376bf0c8e203c38f`
- Artifact ID: `10916403969`
- Artifact SHA-256: `f6452368c204c415b00a297dafcfecbcb76d3d96a9e3ce92ae1e7f37eb865c07`

Observed pinned-workload results:

- attempts 1 through 6: all `rc=0`
- `localized=0`
- `unexpected_failure=0`

Again, the workflow job is marked failed only because this diagnostic was designed to fail when the rare ESRCH could not be captured. All six product observations completed successfully.

## Run C — exact exit-group causal capture

The same H1.3 workflow was re-run without changing the candidate semantics.

- Run: `36274203010`, workflow attempt 2
- Job: `108494298519`
- Triggering diagnostic head: `95bee9888e630d20f25129b4f14ecf302bf2917c`
- Artifact ID: `10916597128`
- Artifact SHA-256: `ddedc90c27d1f1ba8f02f7b7ef3fc0ff37c02c40a6d98f6aa8c96b9b52991dfe`

Observed pinned-workload results before localization:

- attempt 1: `rc=0`, `prereg_reconciled=6019`
- attempt 2: `rc=0`, `prereg_reconciled=5973`
- attempt 3: `rc=2`, `prereg_reconciled=2312`

The failing trace captured the following ordered lifecycle facts for one thread group:

- thread-group leader `34223` entered `exit_group(0)`;
- the tracked group membership at that moment was `[34223, 34228]`;
- sibling TID `34228` later reached a syscall-stop where `PTRACE_GET_SYSCALL_INFO` returned `ESRCH`;
- TID `34228` still belonged to TGID `34223`;
- the group was already marked `exit_group_pending=true` from the observed leader syscall;
- TID `34228` had last entered x86_64 syscall number `204` (`sched_getaffinity`);
- the leader had last entered x86_64 syscall number `231` (`exit_group`);
- TID `34228` had no ExecSurface-selected pending syscall result (`selected_pending=false`);
- no `PTRACE_EVENT_EXIT` had yet been received for TID `34228`;
- the TID was not retired by exec and had not previously hit the restart-ESRCH recovery path;
- `/proc/34228/status` still existed at the instant of classification.

Exact classifier record:

`M9_GET_SYSCALL_INFO_ESRCH_STATE tid=34228 tgid=Some(34223) last_syscall_nr=Some(204) exit_group_pending=true selected_pending=false exit_event_seen=false retired_by_exec=false restart_esrch_seen=false proc_status_exists=true group_members=[(34223, Some(231), true, false, false, false, false), (34228, Some(204), true, false, false, false, false)]`

## Current interpretation

`PROVED`: the pre-registration ordering candidate can run the pinned external workload repeatedly while reconciling thousands of child-first stops per run.

`PROVED`: the residual `PTRACE_GET_SYSCALL_INFO -> ESRCH` is not an unidentified generic ptrace failure in the captured case. It occurred on a sibling thread after an observed `exit_group` entry had already armed teardown for the same TGID.

`PROVED`: in the captured case ExecSurface had no selected pending syscall result for the dying sibling, so declining to fabricate a syscall result would not discard one of the currently modeled selected syscall outcomes.

`COMPUTATIONAL_EVIDENCE`: eleven full pinned-workload observations across the earlier H1.3 diagnostics completed successfully without reproducing the rare residual failure; the same unchanged diagnostic then localized the failure on the third attempt of the repeat run.

`OPEN`: a bounded product fix still requires a targeted/minimized reproducer or equivalent regression certificate showing that a thread in an already-observed `exit_group` teardown may refuse `PTRACE_GET_SYSCALL_INFO` before its `PTRACE_EVENT_EXIT`/terminal wait is delivered.

## Safety decision

Do **not** implement blanket `GET_SYSCALL_INFO ESRCH` recovery and do **not** merge the diagnostic stack to `main` yet.

Any authorized recovery must be narrower than `ESRCH` itself. At minimum it must require:

1. the TID is already tracked;
2. its TGID has explicit, previously observed `exit_group` entry evidence;
3. the TID is at an observed syscall-stop;
4. ExecSurface has no selected pending syscall result whose exit semantics would be lost;
5. the TID is not being retired through an exec identity transition;
6. the tracer does not fabricate a syscall result or silently mark the tracee complete;
7. lifecycle completion is still obtained through subsequent exit-event / terminal-wait evidence, with unresolved state remaining fail-closed.

## Next gate

M9-H1.4B targets the exact captured case: one thread repeatedly executes `sched_getaffinity` while the thread-group leader enters `exit_group`. The objective is to reproduce the refusal window under controlled conditions, then prove a lifecycle-specific recovery without sleeps, arbitrary retries, guessed state, or evidence suppression.
