# M10.4 — Hybrid Exec Authority Gate

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m104`
Status: **PREREGISTERED — NO RESULT YET**

## Objective

Test whether a hybrid BPF architecture can distinguish **exec attempt** from **successful exec** while grounding the executable candidate in a kernel object.

The design under test is deliberately two-stage:

1. audit-only BPF LSM `bprm_check_security` emits a session-scoped candidate executable object identity from `bprm->file`;
2. `sched_process_exec` confirms that an exec actually committed successfully.

An LSM candidate alone MUST NOT be promoted to successful-exec evidence.

## Fixed roles

- **Innovative Systems Architect** — seek a stronger kernel-object + success-confirmation composition than ptrace syscall entry/exit.
- **Anti-Deviation / Skeptical Reviewer** — reject any design that equates pre-commit LSM mediation with successful exec, hides multiple candidate passes, or silently widens authority.

## Dynamic team

Linux exec/`linux_binprm`, LSM/VFS, scheduler tracepoints, BPF/libbpf verifier, credential/namespace semantics, process lifecycle, loss accounting, static guest packaging, reproducibility, internal red team.

## Kernel semantics anchor

The kernel documents `bprm_check_security` as an exec security hook that may be called multiple times during one `execve`. It is therefore an **attempt/candidate** signal, not a success signal by itself.

The kernel's `sched_process_exec` tracepoint is emitted after the task has successfully switched to the new executable image. M10.4 therefore requires both layers for successful-exec authority.

## Controlled fixture

Two independently registered child roles are used under one fixed session:

### Role S — successful exec

1. child starts and `SIGSTOP`s before the tested operation;
2. parent registers its TGID as role `SUCCESS`;
3. child opens `/bin/busybox` with `O_PATH`, obtains `fstat(2)` raw `(st_dev, st_ino)` ground truth for that exact fd, and sends it to parent;
4. child performs `execveat(fd, "", argv={"true",NULL}, envp={NULL}, AT_EMPTY_PATH)`;
5. the new image exits `0`.

### Role F — failed exec

1. child starts and `SIGSTOP`s;
2. parent registers its TGID as role `FAIL`;
3. child attempts `execve()` on a deliberately nonexistent path;
4. the attempt must fail and the child exits `0` only if failure occurred as expected.

## Required BPF evidence

For role `SUCCESS`:

- at least one session-scoped `bprm_check_security` candidate is observed;
- exactly one `sched_process_exec` success confirmation is observed;
- the final candidate associated with the successful direct ELF exec has raw `(dev, ino)` equal to the child's pre-exec `fstat` truth;
- no producer drop, malformed record, or wrong-session record occurs.

For role `FAIL`:

- **zero `sched_process_exec` success confirmations**;
- any LSM candidate, if observed, remains attempt-only and MUST NOT be promoted;
- expected userspace failure is independently confirmed by the child.

## Frozen classifications

- `M10_4_HYBRID_EXEC_MATCH` — successful role has matching object identity + exactly one success confirmation; failed role has zero success confirmations; all health gates clean.
- `M10_4_EXEC_OBJECT_MISMATCH` — successful exec is confirmed but the accepted candidate object differs from target-side fd truth.
- `M10_4_FAILED_EXEC_FALSE_PROMOTION` — failed role receives a success confirmation or is otherwise promoted as successful.
- `M10_4_EVIDENCE_INCOMPLETE` — required uniqueness, correlation, loss, role, or ground-truth conditions are not satisfied.
- `M10_4_ENVIRONMENT_BLOCKED` — required BPF-LSM/tracepoint capability cannot be exposed in the accepted boot-controlled guest.
- `M10_4_INFRA_FAILURE` — build/harness failure before interpretable evidence.

## Stop rule

Record the first interpretable M10.4 result before beginning CONNECT or forced-loss work.

## Non-authorizations

Even `M10_4_HYBRID_EXEC_MATCH` does not authorize public BPF PASS, `learn/check`, baseline interchangeability, backend auto-selection, ptrace replacement, enforcement, or changes to `main`, Marketplace, or `v0.1.0-alpha.3`.