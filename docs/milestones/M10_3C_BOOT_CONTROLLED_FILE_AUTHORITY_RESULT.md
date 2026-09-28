# M10.3C — Boot-Controlled BPF-LSM File Authority Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103c`
Protocol: `docs/milestones/M10_3C_BOOT_CONTROLLED_FILE_AUTHORITY_PROTOCOL.md`
Protocol amendment: `docs/milestones/M10_3C_PROTOCOL_AMENDMENT_001_FILE_OPEN_ABI.md`
Status: **CLOSED — BPF_LSM_FILE_AUTHORITY_PASS**

## Accepted run

Source SHA:
`c89351c9cef1ed19a2192ed9d7d82f814f07c94a`

Workflow run:
`36436038157`

Job:
`108974101370`

Artifact:
`10975547912`

Artifact ZIP digest:
`sha256:41262adfa3fea71cd82d011fb03c58239b5ac2a4ad632d7db3c881ea54aac6e4`

## Environment

- GitHub outer host: Ubuntu 24.04
- guest kernel: `6.8.0-142-generic`
- guest BPF configuration verified: `CONFIG_BPF=y`, `CONFIG_BPF_SYSCALL=y`, `CONFIG_BPF_LSM=y`
- guest boot argument: `lsm=bpf`
- active guest LSM list: `lockdown,capability,bpf`
- guest BTF readable: `true`
- deterministic guest probe complete: `true`

## Frozen proposition result

The corrected audit-only `lsm/file_open` BPF-LSM program attached successfully inside the boot-controlled guest.

Independent fixture truth and accepted event counts:

| Object | Independent truth | Accepted events | Expected |
|---|---|---:|---:|
| A | `dev=2, ino=21` | 100 | 100 |
| B | `dev=2, ino=22` | 100 | 100 |

Additional health:

- target exit: `0`
- wrong-session events: `0`
- other/unmatched object events: `0`
- ring-buffer reservation drops: `0`
- loader exit: `0`
- loader candidate: `BPF_LSM_FILE_AUTHORITY_PASS`

Frozen evaluator classification:

`BPF_LSM_FILE_AUTHORITY_PASS`

## Build identities

- amended BPF source SHA-256: `0ef40bb6ce2f081e5df8e3f2a4981d3af2c2c98b926d8d6ca2fe1a85b9807a76`
- loader source SHA-256: `c98b478b7befde3c95937134a16aecf1769a250a5e34e8ff5d42681153529fee`
- BPF object SHA-256: `4bbb898e81b0a4fc649ac3074104361e83199dc709ce95439871fef9da5db9eb`
- static guest loader SHA-256: `4c9de96ab1f48a5be88f608d121f1a50b3e47097120795e3b2d021c1ef1cb44b`

Guest kernel inputs:

- kernel package SHA-256: `51772494d46f23d0fad486c7bcd59e21727ec59b9b1894f9c6b4d61ca1899d3c`
- buildinfo package SHA-256: `93d1f90f340714c2259757b2ab5d7b5e9c62a154b820dcb2af8ba6f81e1bd910`
- `vmlinuz` SHA-256: `cd5fcfd260b91782637b7b4e221a48e656358f6eef549602540e62380b6f2f2c`

## Required preserved negative evidence

The first real BPF-LSM semantic run (`36435456534`) is permanently preserved as:

`M10_3C_FILE_OPEN_ABI_INVALID`

It exposed a prototype ABI error before fixture execution. The repair was preregistered in Amendment 001 before source modification. The file-object proposition and success/kill thresholds did not change.

## What this proves

Within this bounded controlled proposition:

> An audit-only BPF-LSM `file_open` hook can emit kernel-object-grounded `(s_dev, i_ino)` evidence for a registered execution session that exactly matches independent file-object truth across 200 controlled successful opens, with zero observed session misrouting and zero recorded ring-buffer reservation drops.

This is stronger object-identity evidence than treating the current ptrace syscall-entry pathname copy as universally equivalent to the kernel-consumed object.

## What this does NOT prove

This result does not prove:

- complete filesystem semantics;
- read/write fd lifecycle parity with current ExecSurface;
- path-name reconstruction semantics;
- rename/mount/namespace equivalence;
- exec semantics;
- network semantics;
- zero event loss under stress;
- public GitHub-hosted direct BPF-LSM availability;
- general cross-kernel portability;
- public BPF-LSM `learn`, `check`, or PASS authority;
- ptrace/BPF-LSM baseline interchangeability;
- product integration readiness.

## Authority decision

The proposition-level technical result is accepted.

Authority remains bounded to research evidence for the specific file-object proposition.

No public product authority is granted.

## Next gate

M10.4 — `BPF-LSM-EXEC-001`

The next gate must preregister successful-exec object/credential evidence correlated with a kernel lifecycle commit signal, while explicitly distinguishing failed exec attempts from committed execs.

## Product freeze

Unchanged:

- `main` remains untouched by M10 implementation;
- public `v0.1.0-alpha.3` remains unchanged;
- Marketplace behavior remains unchanged;
- ptrace remains the public correctness-reference backend;
- no BPF-LSM PASS/learn/check authority;
- no backend auto-selection;
- no cross-backend baseline interchangeability;
- no privileged public daemon/service;
- no enforcement.
