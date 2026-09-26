# M9.1 — Post-hardening zero-contact external workload selection freeze

Date: 2026-09-27
Status: FROZEN BEFORE NEW POST-HARDENING RESULT COLLECTION
Independence class for every record in this batch: `ZERO_CONTACT_EXTERNAL_REPRO`
Protocol authority: `docs/milestones/M9_PROTOCOL.md`

## Why this is a new batch

The earlier M9.1 branch contains the canonical pre-fix `casey/just` failure and remains immutable evidence. Issue #55 was subsequently closed after the lifecycle-specific ptrace fix was proved and merged to `main` at `6234da919d1e46b60564114929db593bbce8ef31`.

This batch starts from that merged product state. It does not relabel the earlier failure, delete it, or count AETHER X-run workloads as independent adoption.

## Fixed selection criteria

A workload is admitted to this batch only if all are true before result collection:

1. the project is public and third-party maintained;
2. the exact upstream revision is pinned before execution;
3. AETHER X does not modify project source code;
4. the command is non-interactive on Linux x86_64;
5. the workload can be primed before measurement so dependency download/build-network effects are not part of the measured command;
6. the measured command does not require secrets, credentials, private data, stdin contents, or network payload inspection;
7. the batch spans more than one implementation/runtime ecosystem or process topology;
8. every success, timeout, observer error, incomplete result, false-positive suspicion, or usability problem is retained.

## Frozen workload batch

### ZC-01 — casey/just

- repository: `https://github.com/casey/just`
- revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- ecosystem: Rust/Cargo
- command: `cargo +1.90.0 test --all`
- purpose: re-run the exact external project that exposed the original ptrace lifecycle blocker, now against merged `main`.

### ZC-02 — BurntSushi/ripgrep

- repository: `https://github.com/BurntSushi/ripgrep`
- revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`
- ecosystem: Rust/Cargo workspace
- command: `cargo +1.90.0 test --workspace --all-targets`
- purpose: independent Rust workspace with a different test/process topology from `just`.

### ZC-03 — junegunn/fzf

- repository: `https://github.com/junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- ecosystem: Go modules (`go 1.23.0` declared upstream)
- command: `go test ./...`
- purpose: non-Rust external project and Go test-process topology.

## Frozen execution procedure

For each project, on the same GitHub-hosted Ubuntu 24.04 job:

1. verify exact pinned revision and clean tracked working tree;
2. build the merged `main` source and record its SHA/version;
3. prime the external workload directly before accepted collection;
4. run the direct command once as a compatibility gate;
5. run `execsurface doctor`;
6. run `execsurface learn` on the unchanged command;
7. run `execsurface check` twice against that baseline using the identical command;
8. require both unchanged checks to be `PASS` with zero findings for a compatibility success;
9. preserve all failures fail-closed rather than extending timeouts or changing the command after seeing a result.

No artificial drift case is required in this first compatibility batch because the purpose is cross-project unchanged-workload stability; M7 already retains a controlled drift proof. Any natural drift observed is preserved rather than normalized away.

## Frozen performance procedure

Performance collection is attempted for every batch workload only after the direct compatibility gate and an observed run can complete on the same job. A performance failure remains evidence and does not remove the compatibility result.

- timeout: 120 seconds per sample;
- direct warmups: exactly 3;
- ptrace warmups: exactly 3;
- measured direct samples: exactly 15;
- measured ptrace samples: exactly 15;
- measured order alternates paired direction: `(direct, ptrace)`, then `(ptrace, direct)`, repeated until 15 samples/mode;
- no measured sample is discarded except a recorded infrastructure failure;
- compute medians in milliseconds;
- M6.5 trigger is crossed only if `D >= 100 ms && P/D > 2.0`, or `P-D > 500 ms`;
- crossing the trigger does not authorize eBPF; it only permits a separate same-workload value experiment.

## Evidence and claims boundary

Every accepted record must conform to `docs/milestones/M9_EVIDENCE_SCHEMA.json` and be validated by the frozen M9 validator. These results are exact-host `ZERO_CONTACT_EXTERNAL_REPRO` evidence only. They do not count as independent adoption and do not support universal Linux performance claims.
