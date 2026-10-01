# M10.5 — Hybrid CONNECT Authority Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m105`
Protocol: `docs/milestones/M10_5_CONNECT_AUTHORITY_PROTOCOL.md`

## Classification

`M10_5_HYBRID_CONNECT_MATCH`

## Accepted execution

Workflow run: `36440309861`  
Job: `108988768092`  
Source SHA: `d9641581c6ed97a16e23aa463f7da9a632cb66fd`  
Artifact: `10977711272`  
Artifact ZIP digest: `sha256:f7f0183d431fa9468e7f48c45fda4863825c65b16984ea5802530fe152a30115`

Workflow conclusion: `success`.

## Environment

- guest kernel: `6.8.0-142-generic`;
- active LSM list: `lockdown,capability,bpf`;
- BPF LSM active;
- kernel BTF readable;
- `syscalls/sys_exit_connect` tracepoint available after mounting tracefs;
- probe completed.

## Frozen evidence

### Successful role

Independent target / external truth:

- family: IPv4 (`AF_INET` / `2`);
- destination address: `127.0.0.1` (`2130706433` host-order print);
- destination port: `36025`;
- userspace `connect()` return: `0`;
- `getpeername()` matched the same destination;
- server-side `accept()` succeeded;
- child exit: `0`.

Hybrid kernel evidence:

- LSM `socket_connect` candidates: `1`;
- syscall-exit events: `1`;
- success confirmations: `1`;
- candidate family: `2`;
- candidate address: `127.0.0.1`;
- candidate port: `36025`;
- `sys_exit_connect` return: `0`.

### Failed role

Independent target truth:

- non-listening loopback destination port: `41771`;
- userspace `connect()` return: `-1`;
- errno: `111` (`ECONNREFUSED`);
- child exit: `0`.

Hybrid kernel evidence:

- LSM destination candidates: `1`;
- syscall-exit events: `1`;
- success confirmations: `0`;
- `sys_exit_connect` return: `-111`.

The failed role was not promoted to successful-connect evidence.

### Health

- malformed records: `0`;
- wrong-session records: `0`;
- unknown-role records: `0`;
- producer ring-buffer reservation drops: `0`.

The frozen loader and workflow evaluator both emitted:

`M10_5_HYBRID_CONNECT_MATCH`

## Attempt 1 preservation

Run `36440004482` is preserved separately in `M10_5_ATTEMPT1_HARNESS_REVIEW.md` as non-interpretable. BPF LSM and BTF were present, but tracefs was not mounted, so libbpf could not attach `sys_exit_connect` and the scientific fixture did not execute. The accepted repair mounted tracefs only; it did not alter the BPF proposition or loader semantics.

## Interpretation

Within the declared bounded IPv4/TCP loopback fixture, the hybrid design successfully separated:

1. kernel-mediated destination candidate evidence from BPF LSM `socket_connect`; and
2. successful/failed syscall completion from `sys_exit_connect`.

This composition avoided the semantic error of treating a connect attempt as proof of successful connection. The successful destination matched independent target and server-side ground truth, while the controlled failed connect remained attempt/failure evidence only.

## What this proves

For this bounded fixture:

- BPF LSM can provide kernel-side destination candidate metadata for the tested connect;
- syscall-exit evidence can provide a distinct success/failure confirmation layer;
- candidate + completion can be correlated by registered session/role;
- a failed connect can be prevented from false promotion;
- no recorded transport loss or session contamination occurred.

## What this does not prove

This result does **not** establish:

- universal IPv4/IPv6/Unix-domain coverage;
- UDP semantics;
- asynchronous/nonblocking-connect completion equivalence;
- namespace/container equivalence;
- zero loss under load;
- acceptable production overhead;
- public BPF PASS or `learn/check` authority;
- baseline interchangeability;
- backend auto-selection;
- ptrace replacement;
- product integration.

## Decision

M10.5 is **CLOSED / MATCH**.

Authorized next gate only: **M10.6 — LOSS-AUTHORITY-001**, requiring real producer/transport loss to be forced and detected so incomplete evidence cannot be mistaken for clean/PASS-capable evidence.

No change to `main`, `v0.1.0-alpha.3`, Marketplace behavior, public ptrace authority, or baseline semantics is authorized.
