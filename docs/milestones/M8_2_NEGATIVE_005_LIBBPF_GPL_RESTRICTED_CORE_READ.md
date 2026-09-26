# M8.2 Negative Evidence 005 — GPL-restricted helper in BTF lineage probe

Date: 2026-09-26
Status: **NEGATIVE EVIDENCE — PRESERVED**
Tracking: #34

## Observation

After replacing the classic `sched_process_fork` tracepoint with a BTF-aware `tp_btf/sched_process_fork` program, the libbpf probe reached the kernel verifier but was rejected because `bpf_core_read()` compiled to `bpf_probe_read_kernel`, which this kernel classifies as GPL-restricted.

Verifier evidence included:

```text
cannot call GPL-restricted function from non-GPL compatible program
```

Workflow evidence:

- M8 eBPF Feasibility run `36241468126`
- job `108402664623`
- source commit: `927041b3cc533600871a8723d74f70ac6d63e000`

## Governance ruling

Do **not** change the BPF program license string to `GPL` merely to make the experiment pass while the source remains under the repository's Apache-2.0 licensing model.

A feasibility optimization does not justify weakening licensing clarity.

## Corrective experiment

Keep the BTF-aware `tp_btf/sched_process_fork` attachment, but read `task_struct::pid` through direct trusted-pointer field loads carrying CO-RE relocations via `preserve_access_index`.

This avoids the GPL-restricted probe-read helper while retaining:

- BTF-aware relocation;
- metadata-only lineage evidence;
- the existing 16-byte event schema;
- Apache-2.0 source licensing;
- the same loss/privacy/lineage hard gates.

## Anti-drift ruling

If direct CO-RE access is not accepted by the verifier, record that result separately. Do not silently fall back to a GPL-only helper or relax lineage evidence.
