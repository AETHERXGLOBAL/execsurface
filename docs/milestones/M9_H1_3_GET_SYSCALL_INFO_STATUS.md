# M9-H1.3 — GET_SYSCALL_INFO residual race status

Status: **OPEN / residual race not yet reproduced deterministically**

This record preserves the results of two independent H1.3 diagnostic gates that ran from the hardening branch after the pre-registration ordering candidate had already demonstrated substantial improvement on the pinned `casey/just` workload.

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

- Run: `36274203010`
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

## Current interpretation

`PROVED`: the pre-registration ordering candidate can run the pinned external workload repeatedly while reconciling thousands of child-first stops per run.

`PROVED`: a separate residual `PTRACE_GET_SYSCALL_INFO -> ESRCH` was observed in earlier H1.2 evidence.

`COMPUTATIONAL_EVIDENCE`: after the current lifecycle and pre-registration candidate, eleven additional full pinned-workload observations completed successfully across H1.3 without reproducing the residual failure.

`OPEN`: whether the rare `GET_SYSCALL_INFO` ESRCH is caused by same-thread-group teardown from a concurrent `execve`, `exit_group`, or another ptrace lifecycle transition.

## Safety decision

Do **not** broaden ESRCH recovery and do **not** merge the diagnostic stack to `main` yet. A `GET_SYSCALL_INFO` failure can occur at a syscall-stop where evidence-relevant syscall phase information may otherwise be lost. The next gate must produce a targeted reproducer or equivalent causal certificate before this class can be downgraded from fail-closed.

## Next gate

M9-H1.4 will target a multithreaded lifecycle where one thread can disappear while another thread in the same group executes `execve`/group teardown. The objective is to deterministically classify the `GET_SYSCALL_INFO` race, not to make the workload pass by retrying or ignoring ESRCH.
