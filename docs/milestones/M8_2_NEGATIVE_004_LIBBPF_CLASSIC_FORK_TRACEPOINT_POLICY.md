# M8.2 Negative Evidence 004 — libbpf classic sched_process_fork attachment denied

Date: 2026-09-26
Status: **NEGATIVE EVIDENCE — PRESERVED**
Tracking: #34

## Observation

On GitHub-hosted Ubuntu 24.04, the libbpf-rs feasibility probe built successfully on the product MSRV path and loaded far enough to attempt automatic attachment, but the classic tracepoint program for `sched/sched_process_fork` was rejected at attachment time.

Observed diagnostic:

```text
libbpf: prog 'execsurface_m8_fork': failed to create BPF link for perf_event ...: -EACCES
libbpf: prog 'execsurface_m8_fork': failed to attach to tracepoint 'sched/sched_process_fork': -EACCES
Error: Permission denied (os error 13)
```

Workflow evidence:

- M8 eBPF Feasibility run `36241343170`
- job `108402308622`
- branch head at failure: `c098772718698717380fec66a587599f110d3c37`

## Classification

This is **not** evidence that libbpf cannot represent the required lineage semantic.

The same runner had already demonstrated that a syscall tracepoint could attach, and the failure is specifically at the classic `sched_process_fork` perf-event attachment boundary under the hosted-runner policy.

Therefore the failure is classified as an **attachment-mechanism / host-policy limitation**, not a semantic failure and not a PASS.

## Corrective experiment

Replace the classic sched tracepoint lineage hook with a BTF-aware `tp_btf/sched_process_fork` program using CO-RE field access for the process identifiers.

This is intentionally a stronger experiment because it:

- exercises host BTF materially;
- avoids freezing the kernel `task_struct` layout;
- aligns the lineage experiment more closely with the Aya BTF-aware path;
- preserves the 16-byte metadata-only event schema;
- does not weaken the loss, privacy, or lineage gates.

## Anti-drift ruling

Do not hide or rewrite this failure after the BTF-aware experiment succeeds. It remains evidence that attachment mechanism and host policy are part of the backend compatibility contract.
