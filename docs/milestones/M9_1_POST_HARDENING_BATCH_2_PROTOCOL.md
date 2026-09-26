# M9.1 Post-Hardening Zero-Contact Batch 2 Protocol

Status: **PREDECLARED — execute exactly as frozen below**

## Purpose

Batch 2 tests stable, user-like runtime commands after build/setup has completed outside the observed command. It is prospective evidence only. Batch 1 remains unchanged.

This batch is `ZERO_CONTACT_EXTERNAL_REPRO`; it is compatibility/performance evidence and is **not** independent adoption.

## Authority boundary

- Public authority remains Linux x86_64 `ptrace`.
- eBPF remains research-only under M8.8 and cannot authorize PASS, learn/check, backend auto-selection, or baseline interchange.
- Evidence completeness remains fail-closed.
- No normalization rule, threshold, baseline schema, policy behavior, or product semantic is changed for this batch.

## Source under test

The batch must build ExecSurface from the branch state descended from verified main merge:

`6234da919d1e46b60564114929db593bbce8ef31`

The runner must record the exact tested HEAD.

## Frozen external workloads

### B2-ZC-01 — casey/just stable runtime

- upstream: `casey/just`
- revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- setup outside observation: release build with Rust 1.90.0
- controlled fixture: static Justfile created by the harness
- observed command: built `just` binary listing recipes from the static Justfile; child output redirected by a frozen shell wrapper

### B2-ZC-02 — junegunn/fzf stable runtime

- upstream: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- setup outside observation: build the pinned Go binary
- controlled fixture: static input file containing `alpha`, `beta`, `gamma`
- observed command: built `fzf` binary with `--filter alpha` reading the static file; output redirected by a frozen shell wrapper

### B2-ZC-03 — sharkdp/fd stable runtime

- upstream: `sharkdp/fd`
- revision: `ce97e473ebaec49697c07daa50a7bc2b32f713d2`
- setup outside observation: release build with Rust 1.90.0
- controlled fixture: static two-file directory tree
- observed command: built `fd` binary enumerating files in the static fixture; output redirected by a frozen shell wrapper

## Per-workload gate

For each workload, in this order:

1. verify exact upstream revision and clean tracked checkout;
2. direct observed command must exit 0;
3. `execsurface doctor` must pass;
4. `learn` must complete and produce a baseline;
5. unchanged `check` #1 must be recorded exactly as observed;
6. unchanged `check` #2 must be recorded exactly as observed;
7. controlled runtime expansion appending `/usr/bin/sha256sum` must remain visible as added `process_exec` and produce REVIEW/exit 10;
8. performance protocol: exactly 3 direct warmups and 3 ptrace warmups, then 15 measured direct and 15 measured ptrace samples in alternating order;
9. compute medians, absolute overhead, slowdown, and the frozen M6.5 trigger:
   - `(D >= 100 ms AND P/D > 2.0)` OR `(P-D > 500 ms)`;
10. preserve raw JSON/stderr/timing files and SHA-256 manifest.

No result is relabeled after execution. If the direct command fails, later workload stages do not run and the record remains PARTIAL.

## Acceptance interpretation

- `PASS` baseline stability means both unchanged checks are PASS with zero findings.
- Any unchanged REVIEW/BLOCK/ERROR is preserved, investigated separately, and cannot be normalized post hoc.
- Controlled drift visibility is mandatory even if unchanged checks are noisy.
- A performance-trigger crossing permits only a same-workload eBPF value experiment; it does not change public backend authority.

## Fixed review roles

- Innovation Scientist / Systems Architect: seek useful generalizable interpretation without changing the frozen protocol.
- Deviation Prevention / Scientific Integrity: reject post-hoc exclusions, normalization, threshold changes, and adoption inflation.
- Independent Red Team: verify controlled process expansion remains visible and that stable-runtime framing does not hide meaningful drift.
