# M10.5 — Hybrid CONNECT Authority Gate

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m105`
Status: **PREREGISTERED — NO RESULT YET**

## Objective

Test whether a hybrid kernel observer can distinguish **connect attempt** from **successful connect** while preserving the requested destination from a kernel-owned sockaddr and validating the successful peer independently from userspace.

The architecture under test is deliberately two-stage:

1. audit-only BPF LSM `socket_connect` records a session-scoped candidate IPv4 destination from the kernel sockaddr presented to the security hook;
2. the `sys_exit_connect` tracepoint confirms syscall completion and return value.

A connect candidate alone MUST NOT be promoted to a successful connection.

## Fixed roles

- **Innovative Systems Architect** — seek a stronger destination + success-confirmation composition with explicit health semantics.
- **Anti-Deviation / Skeptical Reviewer** — reject pre-call/success conflation, user-pointer authority, hidden loss, ambiguous session attribution, or silent scope widening.

## Dynamic team

Linux networking/socket semantics, LSM hooks, syscall tracepoints, BPF/libbpf verifier, sockaddr ABI/endian handling, process/session correlation, loss accounting, QEMU reproducibility, internal red team.

## Controlled fixture

All networking is loopback-only inside the isolated guest.

### Role S — successful TCP connect

1. parent creates a TCP listener on `127.0.0.1` with an OS-assigned port;
2. child starts and `SIGSTOP`s before the tested connect;
3. parent registers child TGID as role `SUCCESS`;
4. child connects once to the listener;
5. after successful `connect(2)`, child calls `getpeername(2)` on the connected fd and sends `(family,address,port)` to parent as ground truth;
6. child exits `0`.

### Role F — failed TCP connect

1. parent reserves a second loopback port by binding a TCP socket but does not call `listen(2)`;
2. child starts and `SIGSTOP`s;
3. parent registers child TGID as role `FAIL`;
4. child attempts one `connect(2)` to that bound-but-not-listening endpoint;
5. child reports the returned errno and exits `0` only if the connect failed as expected.

## Required BPF evidence

For role `SUCCESS`:
- exactly one relevant `socket_connect` candidate for the registered role;
- exactly one `sys_exit_connect` completion with return `0`;
- candidate destination exactly equals child `getpeername(2)` ground truth;
- zero malformed records, wrong-session records, or producer drops.

For role `FAIL`:
- a connect attempt may be observed;
- the completion return must be negative;
- **zero successful-connect promotions**;
- userspace independently confirms failure.

## Frozen classifications

- `M10_5_HYBRID_CONNECT_MATCH` — success role destination matches `getpeername`, completion is `0`, failure role has no success promotion, all health gates clean.
- `M10_5_CONNECT_DESTINATION_MISMATCH` — successful connect is confirmed but candidate destination differs from target-side peer truth.
- `M10_5_FAILED_CONNECT_FALSE_PROMOTION` — failed role is promoted as successful.
- `M10_5_EVIDENCE_INCOMPLETE` — uniqueness/correlation/ground-truth/loss conditions prevent interpretation.
- `M10_5_ENVIRONMENT_BLOCKED` — required BPF-LSM or syscall-tracepoint capability cannot be exposed in the accepted guest.
- `M10_5_INFRA_FAILURE` — build/harness failure before interpretable execution.

## Stop rule

Record the first interpretable M10.5 result before forced-loss, stress, packaging, or product-integration gates.

## Non-authorizations

Even `M10_5_HYBRID_CONNECT_MATCH` does not authorize public BPF PASS, `learn/check`, baseline interchangeability, backend auto-selection, ptrace replacement, enforcement, or changes to `main`, Marketplace, or `v0.1.0-alpha.3`.