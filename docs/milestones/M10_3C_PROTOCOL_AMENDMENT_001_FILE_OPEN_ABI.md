# M10.3C Protocol Amendment 001 — `file_open` BPF-LSM ABI Repair

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103c`
Status: **RECORDED BEFORE REPAIR / FIRST SEMANTIC RUN INVALID FOR PROPOSITION**

## Trigger

The first M10.3C run that reached a real BPF-LSM-enabled guest was:

- source SHA: `ac482f11dd8d95460db0a8260dc3d6abfa937b1e`
- workflow run: `36435456534`
- job: `108972088818`
- artifact: `10975880099`
- artifact ZIP digest: `sha256:8cb5ef3e8b70bbd3f8c4184d158d6f94deceb1910f569c9a11f01f1ee8295ece`

The environment itself was healthy:

- guest booted Ubuntu `6.8.0-142-generic`;
- active LSM list: `lockdown,capability,bpf`;
- guest BTF readable;
- BPF-LSM was active;
- probe reached deterministic completion.

The BPF program failed verifier load before the controlled A/B fixture executed:

```text
func 'bpf_lsm_file_open' doesn't have 3-th argument
invalid bpf_context access off=16 size=8
```

The frozen evaluator therefore emitted:

`BPF_LSM_FILE_INFRA_FAILURE`

This is accepted as an implementation/ABI failure record, **not** as evidence for or against the file-object authority proposition.

## Root cause

The frozen prototype declared:

```c
SEC("lsm/file_open")
int BPF_PROG(m10_file_open, struct file *file, int mask, int ret)
```

But the Linux LSM hook definition for `file_open` is an integer-returning hook whose hook-specific argument list is:

```c
file_open(struct file *file)
```

For BPF LSM integer-returning hooks, the BPF program receives the hook-specific arguments followed by the previous BPF-program return value used for chaining. Therefore the correct BPF program signature for this hook is:

```c
SEC("lsm/file_open")
int BPF_PROG(m10_file_open, struct file *file, int ret)
```

Authoritative references used for this ABI correction:

- Linux `include/linux/lsm_hook_defs.h` — `LSM_HOOK(int, 0, file_open, struct file *file)`
- Linux BPF-LSM documentation — integer-returning hook examples append `int ret` to the hook-specific arguments for BPF chaining.

## Allowed repair

The next implementation revision may make exactly these semantic-neutral ABI corrections:

1. change the BPF program signature from `(file, mask, ret)` to `(file, ret)`;
2. preserve the existing event schema for comparability, but set the diagnostic `mask` field to `0` because `file_open` does not supply a hook-specific mask argument;
3. retain all existing object identity, TGID/session membership, drop accounting, privacy, and allow-only behavior unchanged.

No other BPF proposition logic may be changed under this amendment.

## Frozen scientific proposition remains unchanged

The rerun must still require:

- a real `lsm/file_open` attachment;
- target exit `0`;
- exactly 100 A-object events matching independent `(st_dev, st_ino)` truth;
- exactly 100 B-object events matching independent `(st_dev, st_ino)` truth;
- zero other object events;
- zero wrong-session events;
- zero ring-buffer reservation drops;
- audit-only behavior;
- no public ptrace/core changes.

## Interpretation rule

Run `36435456534` remains permanently preserved as:

`M10_3C_FILE_OPEN_ABI_INVALID`

It must never be reclassified as a semantic PASS, PARTIAL, or COUNTEREXAMPLE.

Only a post-amendment run that successfully loads and attaches the corrected hook may be interpreted against the frozen file-object proposition.

## Product authority

Still unchanged:

- `main`: untouched;
- public `v0.1.0-alpha.3`: untouched;
- Marketplace behavior: untouched;
- ptrace public authority: unchanged;
- BPF-LSM public PASS / learn / check authority: **not granted**;
- product integration: **not authorized**.
