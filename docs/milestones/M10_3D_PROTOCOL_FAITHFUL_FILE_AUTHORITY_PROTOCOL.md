# M10.3D — Protocol-Faithful BPF-LSM File Authority Gate

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103d`
Status: **PREREGISTERED — NO RESULT YET**

## Objective

Execute the original M10.3 bounded proposition faithfully inside the boot-controlled BPF-LSM environment proved by M10.3B:

> One session-registered target opens one controlled file after an explicit stop barrier; the BPF LSM `file_open` event's raw `(s_dev, i_ino)` equals target-side `fstat(2)` ground truth for that returned fd, with zero unaccounted producer loss.

## Fixed roles

- **Innovative Systems Architect** — seek stronger kernel-object-grounded evidence without conflating technical authority with product-default selection.
- **Anti-Deviation / Skeptical Reviewer** — reject criterion drift, silent fixture substitution, hidden loss, or authority promotion beyond the bounded result.

## Dynamic team

Linux LSM/VFS, BPF/libbpf verifier, kernel object identity, concurrency/race semantics, loss/health accounting, static guest packaging, QEMU/boot reproducibility, licensing, internal red team.

## Protocol amendment boundary

The prior research BPF source used the wrong `file_open` context shape. Linux `file_open` has one hook argument (`struct file *file`); BPF LSM chaining supplies the prior return value. M10.3D therefore uses:

`BPF_PROG(..., struct file *file, int ret)`

This is an ABI-correctness repair required to execute the already-frozen proposition. No `mask` field is claimed.

The older M10.3C two-file/100-iteration fixture is not reused as authority evidence.

## Fixture

1. parent creates one controlled file before attaching/registering the target;
2. audit-only BPF LSM program loads and attaches to `lsm/file_open`;
3. child starts and immediately `SIGSTOP`s;
4. parent registers child TGID to one fixed session;
5. parent sends `SIGCONT`;
6. child opens exactly the controlled file once;
7. child performs `fstat(2)` on that exact returned fd and reports raw `st_dev`, `st_ino` to parent via an inherited pipe;
8. child closes the fd and exits `0`;
9. collector drains the ring and reads the drop counter.

## Required evidence

A semantic match requires all of:

- BPF LSM active in guest;
- BTF readable;
- `lsm/file_open` attaches successfully;
- session registered before tested open;
- target exits `0`;
- target-side `fstat` ground truth received;
- exactly one accepted event for the registered session;
- event raw `(dev, ino)` exactly equals target `fstat` raw `(dev, ino)`;
- zero wrong-session events;
- zero unmatched session events;
- zero ring-buffer reservation drops;
- no decode/collector failure;
- BPF program preserves prior LSM denial and otherwise returns allow (audit-only).

## Frozen classifications

- `M10_3_OBJECT_IDENTITY_MATCH` — all gates pass and unique event equals target `fstat` identity.
- `M10_3_OBJECT_IDENTITY_MISMATCH` — healthy unique event differs from target `fstat` identity.
- `M10_3_EVIDENCE_INCOMPLETE` — attachment works but health/ground-truth/uniqueness/loss conditions prevent interpretation.
- `M10_3_ENVIRONMENT_BLOCKED` — boot-controlled guest cannot expose/load/attach required BPF-LSM capability.
- `M10_3_INFRA_FAILURE` — harness/build failure before a scientifically interpretable result.

## Stop rule

Record the first interpretable result before starting EXEC, CONNECT, forced-loss, or product integration gates.

## Non-authorizations

Even `M10_3_OBJECT_IDENTITY_MATCH` does not authorize public BPF PASS, `learn/check`, backend auto-selection, cross-backend baselines, ptrace replacement, privileged host service, enforcement, or changes to `main`/Marketplace/`v0.1.0-alpha.3`.