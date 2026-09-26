# M8.2 syscall lineage contract

The fair Apache-safe feasibility comparison uses `sys_exit_clone` for both candidate stacks.

For the current Linux x86_64 experiment:

- parent identity is the current TGID;
- a positive `clone` return value is the child identity;
- zero/negative return values produce no lineage event;
- the persisted event remains `{kind,pid,related_pid,reserved}` (16 bytes);
- this controlled lineage proof is not claimed equivalent to the full ptrace spawn semantics.

The BTF `task_struct` lineage approach was killed for M8.2 under the Apache-only probe contract after verifier evidence showed kernel-structure access was restricted to GPL-compatible programs on the tested kernel.
