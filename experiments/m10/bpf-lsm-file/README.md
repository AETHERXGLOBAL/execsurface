# M10.3 — BPF LSM File Authority Prototype

Research-only prototype for `BPF-LSM-FILE-001`.

## Boundary

- branch-only M10 experiment;
- audit-only BPF LSM program;
- every LSM decision returns allow / preserves prior denial;
- no file contents, argv, envp, stdin, or payload capture;
- no public backend authority;
- no changes to ExecSurface core or Marketplace behavior.

## Proposition

For two controlled files A and B, the `file_open` LSM hook emits the kernel file object's `(s_dev, i_ino)` for a registered target TGID. The loader compares event counts and object identities against the independently `stat(2)`-verified A/B objects.

The child is stopped before its first controlled open. The loader registers its TGID in the BPF session map, then releases it. Only registered TGIDs produce events.

## Health

The prototype records:

- active LSM list;
- vmlinux BTF availability;
- real BPF LSM load/attach result;
- session ID;
- A/B event counts;
- unmatched/wrong-session events;
- ring-buffer reservation drop count;
- target exit status.

`run.sh` classifies host inability separately from semantic failure.

## Licensing boundary

The research BPF program declares GPL compatibility because BPF LSM attachment and helper availability can impose kernel-side GPL constraints. This prototype is research evidence only and is **not** a licensing decision for the Apache-2.0 public ExecSurface product.
