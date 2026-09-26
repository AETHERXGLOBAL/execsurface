# M9.1 — Post-hardening zero-contact external workload selection freeze

Date: 2026-09-27
Status: FROZEN BEFORE RESULT COLLECTION
Execution base: main `6234da919d1e46b60564114929db593bbce8ef31`
Independence class: `ZERO_CONTACT_EXTERNAL_REPRO`
Protocol: `docs/milestones/M9_PROTOCOL.md`

## Governance

The earlier M9.1 branch and its canonical `casey/just` ESRCH failure remain retained evidence. This batch starts from the merged ptrace lifecycle fix on `main`. AETHER X initiates and executes every workload in this batch, so no result may be described as independent adoption.

Fixed roles remain: Innovation Scientist / Adoption Architect; Deviation Prevention / Scientific Integrity; Independent Red Team.

## Predeclared admission criteria

A workload is admitted only when, before result collection, it is public and third-party maintained, its exact revision is pinned, source remains unmodified by AETHER X, the command is non-interactive on Linux x86_64, dependency/build network effects can be primed outside measurement, no secrets/private data/payload inspection are needed, and all failures or incomplete results will be retained.

## Frozen batch

### ZC-01 — casey/just
- revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- ecosystem: Rust/Cargo
- command: `cargo +1.90.0 test --all`
- rationale: exact project that exposed the original ptrace lifecycle blocker.

### ZC-02 — BurntSushi/ripgrep
- revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- ecosystem: Rust/Cargo workspace
- command: `cargo +1.90.0 test --workspace --all-targets`
- rationale: different Rust workspace and test/process topology.

### ZC-03 — junegunn/fzf
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- ecosystem: Go modules; upstream declares `go 1.23.0`
- command: `go test ./...`
- rationale: non-Rust external project and Go test-process topology.

## Frozen compatibility procedure

For every workload on Ubuntu 24.04 x86_64:
1. verify exact revision and clean tracked tree;
2. build ExecSurface from this branch/main-derived source and record exact SHA/version;
3. prime the external workload directly;
4. direct compatibility run;
5. `execsurface doctor`;
6. `execsurface learn` on the unchanged command;
7. two identical `execsurface check` reruns;
8. compatibility success requires complete observations and both checks `PASS` with zero findings;
9. fixed timeout is 120 seconds for an individual external command; a timeout remains evidence.

No artificial drift is required in this batch; M7 retains controlled drift evidence. Natural drift, observer errors, truncation, false-positive suspicion, and usability friction are never silently excluded.

## Frozen performance procedure

Performance is attempted for every workload after the direct compatibility gate and one complete observed run succeed on the same host. A performance failure does not erase compatibility evidence.

- exactly 3 direct warmups and 3 ptrace warmups;
- exactly 15 direct and 15 ptrace measured samples;
- paired order alternates `(direct, ptrace)` then `(ptrace, direct)` until 15 samples/mode;
- 120-second timeout per sample;
- no measured sample discarded except a recorded infrastructure failure;
- report medians, absolute overhead, ratio, and the frozen M6.5 trigger;
- trigger: `(D >= 100 ms AND P/D > 2.0) OR (P-D > 500 ms)`;
- crossing the trigger does not authorize eBPF.

## Evidence boundary

Every accepted record must conform to `docs/milestones/M9_EVIDENCE_SCHEMA.json` and pass `scripts/m9_validate_evidence.py`. Results are exact-host zero-contact evidence only and do not count as independent adoption or universal Linux performance claims.
