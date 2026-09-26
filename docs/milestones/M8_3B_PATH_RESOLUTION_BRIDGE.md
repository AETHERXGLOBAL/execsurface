# M8.3b — Userspace Path-Resolution Bridge

Date: 2026-09-26
Status: **CLOSED — CONDITIONAL FAIL-CLOSED RESOLVER PROVED ON REFERENCE CI**
Tracking: #37
Parent: `docs/milestones/M8_3_BACKEND_CONTRACT.md`

## Decision

The userspace `/proc` bridge is accepted **only as a conditional fail-closed resolver**.

It is not a persistent path-authority mechanism and it does not make numeric eBPF occurrence evidence path-equivalent to ptrace evidence.

Accepted bridge:

- exec occurrence `(pid)` -> promptly attempt `/proc/<pid>/exe`;
- successful open `(pid, fd)` -> promptly attempt `/proc/<pid>/fd/<fd>`;
- successful live resolution may supply the observed path identity for that event;
- any resolution failure, lifecycle race, decode failure, or stale-object condition must make the required semantic incomplete rather than emit a guessed path.

The kernel-side event remains metadata-only. No argv, environment value, file content, stdin, or network payload is persisted.

## Why this gate exists

M8.2 proved exec occurrence and successful-open fd identity. It did **not** prove exec path identity or open path identity. The current raw model requires real path metadata for `ProcessExec` and path-bearing file semantics. PID/FD numbers cannot be promoted to path claims by assumption.

## Dynamic team

Fixed:

- Innovation Scientist / Architect
- Deviation Prevention / Scientific Integrity

Dynamic:

- Linux eBPF / tracepoint engineer
- Rust/libbpf-rs engineer
- `/proc` lifecycle/race specialist
- observation-semantics reviewer
- privacy red team
- CI/reproducibility reviewer

## Experiment matrix and result

### E1 — held exec — PROVED

A controlled `/bin/sleep` process remained live after its eBPF exec occurrence. Userspace resolution established:

`M8_3_EXEC_HELD_RESOLUTION_PASS pid=4260 path=/usr/bin/sleep`

### E2 — reaped short-lived exec — PROVED NEGATIVE LIFECYCLE CASE

A controlled `/bin/true` process was reaped before resolution. The numeric exec occurrence remained in collected evidence, but `/proc/<pid>/exe` was unavailable:

`M8_3_EXEC_REAPED_RESOLUTION_UNAVAILABLE_PASS pid=4261`

This proves that exec occurrence does not imply durable later path resolvability.

### E3 — held successful open — PROVED

The corrected deterministic Rust fixture directly invoked `openat(AT_FDCWD, "/dev/null", O_RDONLY)` and held the returned fd open. The live fd resolved correctly:

`M8_3_OPEN_HELD_RESOLUTION_PASS pid=4262 fd=3 path=/dev/null`

### E4 — closed/reaped short-lived open — PROVED NEGATIVE LIFECYCLE CASE

The deterministic short-lived child opened `/dev/null`, closed/exited, and was reaped before resolution. The numeric successful-open evidence remained, while `/proc/<pid>/fd/<fd>` was unavailable:

`M8_3_OPEN_REAPED_RESOLUTION_UNAVAILABLE_PASS pid=4263 fd=3`

This proves that successful-open fd identity does not imply durable later path resolvability.

### E5 — privacy — PROVED ON REFERENCE RUN

The controlled children carried an environment sentinel. The persisted proof log did not contain it:

`M8_3_PATH_BRIDGE_PRIVACY_PASS`

## Reference evidence

Corrected source head:

`72ce6d144cfcc737e6af42a25629f04834481cdd`

Dedicated workflow:

- `M8.3 Path Resolution Bridge`
- run `36244748914`
- result: **SUCCESS**

Normal repository CI on the same head:

- run `36244748984`
- result: **SUCCESS**

Reference environment:

- Ubuntu 24.04.5;
- Linux `6.17.0-1022-azure` x86_64;
- readable kernel BTF;
- `kernel.unprivileged_bpf_disabled=2`;
- `sys_exit_openat` return field audited at offset 16, size 8.

The isolated Rust-1.82 release build completed successfully. Measured binary SHA-256:

`0944b4408bbbf41145e9c61ccd1ac0e0640d66081b8103e2b9ec906ac4391055`

Unprivileged loading failed explicitly rather than escalating privilege:

`M8_3_UNPRIVILEGED_DENIAL_PASS exit=1`

## Preserved negative evidence

The first held-open harness used:

`/bin/sh -c 'exec 9</dev/null; sleep 2'`

That run timed out while trying to identify a `/dev/null` fd under the shell PID. The shell harness was classified as unsuitable evidence rather than as a failure of the bridge hypothesis.

The failure is preserved in:

`docs/milestones/M8_3_NEGATIVE_001_SHELL_OPEN_FIXTURE.md`

The corrected experiment replaced shell behavior with a deterministic direct `openat` child. The failed approach is not rewritten as success.

## Anti-drift conclusion

The evidence supports exactly this claim:

> `/proc` path resolution can enrich live numeric libbpf events with correct path identity when the referenced process/object is still resolvable, and lifecycle loss must remain an explicit incomplete condition.

The evidence does **not** support:

- unconditional exec/open path identity;
- delayed path reconstruction after process/fd lifetime ends;
- ptrace/eBPF parity;
- cross-backend evidence equivalence;
- multi-kernel portability;
- performance superiority;
- public stable eBPF readiness;
- eBPF-derived PASS authority.

## M8.3c entry condition

The selected libbpf collector may now be connected behind the backend contract only if:

1. path-bearing raw events are emitted only after successful live resolution;
2. unresolved required path identity produces explicit incomplete collection state;
3. no eBPF identity silently falls back to ptrace;
4. ptrace remains the default/reference backend;
5. the libbpf implementation remains isolated from the normal ptrace installation path;
6. no `auto` backend selection is introduced;
7. eBPF PASS authority remains **NOT AUTHORIZED**.

## Labels

- live exec path resolution: **PROVED ON REFERENCE CI**
- reaped exec path availability: **NEGATIVE / UNAVAILABLE AS EXPECTED**
- live successful-open fd path resolution: **PROVED ON REFERENCE CI**
- reaped/closed fd path availability: **NEGATIVE / UNAVAILABLE AS EXPECTED**
- userspace path bridge: **ACCEPTED ONLY AS CONDITIONAL FAIL-CLOSED RESOLVER**
- ptrace/eBPF parity: **OPEN / M8.5**
- eBPF PASS authority: **NOT AUTHORIZED**
