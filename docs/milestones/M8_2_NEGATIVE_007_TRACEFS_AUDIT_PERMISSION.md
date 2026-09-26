# M8.2 Negative Evidence 007 — tracefs format audit required privilege

Date: 2026-09-26
Status: **NEGATIVE EVIDENCE — SETUP DEFECT, PRESERVED**
Tracking: #34

## Observation

M8 eBPF Feasibility run `36241993106` proved the Aya and libbpf-rs backend jobs independently, but the separate host ABI audit failed before reading `/sys/kernel/tracing/events/syscalls/sys_exit_clone/format`.

The runner user could see the tracefs mount, while an unprivileged `test -r` on the event format returned false. The backend lifecycle probes themselves run their privileged attach path through explicit `sudo` and both reached their semantic gates.

## Classification

This is not a backend semantic failure and it is not evidence that the tracepoint is absent. It is an audit-permission mismatch: the ABI evidence reader must use the same explicit privileged boundary required to inspect the protected tracefs files on this hosted runner.

## Corrective action

Read the tracepoint format with `sudo cat`, persist the text only inside the CI workspace, and validate the `ret` field layout from that captured format.

Required invariant remains unchanged:

- the relevant syscall-exit tracepoint must exist;
- `ret` must be demonstrated at offset 16 with size 8 on the tested Linux x86_64 host;
- no hidden layout assumption is accepted.

## Anti-drift ruling

Do not delete this failed run after the privileged audit passes. It remains evidence that tracefs inspection has its own host-permission boundary distinct from BPF program attachment.
