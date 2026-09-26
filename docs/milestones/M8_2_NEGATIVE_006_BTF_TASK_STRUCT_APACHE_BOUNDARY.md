# M8.2 Negative Evidence 006 — BTF task_struct access conflicts with Apache-only probe path

Date: 2026-09-26
Status: **KILLED FOR M8.2 LINEAGE PATH UNDER CURRENT ASSUMPTIONS**
Tracking: #34

## Observation

The BTF-aware `sched_process_fork` experiment reached the verifier, but direct access to `struct task_struct` was rejected for the non-GPL-compatible eBPF program:

```text
Cannot access kernel 'struct task_struct' from non-GPL compatible program
```

The preceding `bpf_core_read()` attempt was also rejected because its underlying helper is GPL-restricted.

Evidence:

- M8 eBPF Feasibility run `36241553904`
- libbpf job `108402914579`
- source commit `b4a1c2085515cbb111429244a039e62c591d0485`
- preceding helper rejection: `M8_2_NEGATIVE_005_LIBBPF_GPL_RESTRICTED_CORE_READ.md`

## Cross-stack implication

This is not treated as a libbpf-only weakness. The Aya feasibility program was using the same semantic strategy: BTF `sched_process_fork` plus direct `task_struct::pid` access while declaring `Apache-2.0`.

Therefore both candidates must be evaluated under the same licensing constraint. Aya is not allowed to retain a path that the verifier would reject for the same reason.

## Decision

For the M8.2 minimum lineage proof, use an Apache-safe syscall tracepoint semantic:

- observe `sys_exit_clone`;
- parent identity = current process TGID;
- child identity = successful positive syscall return value;
- persist only numeric metadata.

This is sufficient for the controlled M8.2 descendant-scope experiment but is **not** claimed equivalent to the full ptrace spawn semantics.

## Boundary

BTF/CO-RE remains valuable for future eBPF work, but `task_struct` lineage under the current Apache-only program contract is **KILLED for this milestone** on the tested kernel. It may only be revisited with a legally and technically reviewed mechanism that does not disguise licensing requirements.
