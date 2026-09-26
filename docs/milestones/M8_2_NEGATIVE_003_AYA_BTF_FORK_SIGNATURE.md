# M8.2 Negative Evidence 003 — Aya BTF fork-signature assumption

Date: 2026-09-26
Status: **PRESERVED NEGATIVE EVIDENCE — SEMANTIC ASSUMPTION REJECTED**
Workflow run: `36240310409`
Job: `Aya / build + lifecycle probe`

## Attempt

M8.2 extended the already working metadata-only Aya exec tracepoint with a BTF `sched_process_fork` program intended to emit parent -> child numeric process lineage.

The experimental program assumed the BTF tracepoint arguments exposed integer parent/child PIDs at the selected argument positions.

## Kernel-verifier result

The program reached the real kernel verifier, which reported:

```text
func 'sched_process_fork' arg1 has btf_id 70 type STRUCT 'task_struct'
R3 pointer arithmetic with <<= operator prohibited
```

The load failed before the BTF lineage program could attach.

## Classification

- the integer-PID signature assumption for this kernel: **KILLED**
- BTF program reached kernel verification: **PROVED**
- Aya eBPF stack itself is unusable: **NOT ESTABLISHED**
- stable cross-kernel lineage semantics: **OPEN**
- M8.2 E4 BTF/CO-RE portability: **OPEN / PARTIAL**
- M8.2 E9 descendant semantic probe: **OPEN / PARTIAL**

## Rejected workaround

Hard-coding `task_struct` field offsets is **KILLED** for ExecSurface. It would weaken kernel portability and evidence integrity and could silently misread data when kernel layouts change.

## Correct next path

Any retry must derive the relevant kernel type information from BTF/CO-RE-aware bindings or another kernel-supported stable semantic source, preserve numeric metadata-only privacy, and prove the resulting process lineage against a controlled root command.

Until that proof passes, the current ptrace backend remains the only correctness reference and the eBPF path has no PASS authority.
