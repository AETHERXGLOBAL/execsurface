# M9 — Preserved Negative Evidence Ledger

Date: 2026-09-26
Status: **ACTIVE — APPEND ONLY IN MEANING; DO NOT ERASE FAILURES**
Tracking: #51

## NE-001 — Pre-freeze `casey/just` ptrace observe operational failure

Classification: **PRE-FREEZE / NON-ACCEPTED M9 PERFORMANCE EVIDENCE**
Claim status: `PARTIAL`

### Provenance

- preserved branch: `milestone/m9-independent-adoption`
- workflow commit: `d52fd331dd7664651295a94c996cb1c09cc83646`
- workflow run: `36266650230`
- job: `108472498841`
- upstream: `casey/just`
- upstream revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- independence class: `ZERO_CONTACT_EXTERNAL_REPRO`
- maintainer coordination: none
- upstream modification by AETHER X: none

### What succeeded before the failure

- pinned upstream checkout: PASS;
- Rust 1.90.0 setup: PASS;
- fresh public install of ExecSurface `0.1.0-alpha.2` from crates.io: PASS;
- `execsurface doctor`: PASS on Linux x86_64 with ptrace observer available;
- direct upstream workload `cargo +1.90.0 test --all`: PASS;
- unit suite: 575 passed, 0 failed;
- integration suite: 1852 passed, 0 failed, 18 ignored by upstream.

These facts are preserved as setup/compatibility context only. They do not convert the run into accepted M9 adoption or performance evidence.

### Failure

The performance/observation stage failed during `execsurface observe` with:

```text
ExecSurface: ERROR
execsurface: observer OS error: No such process (os error 3)
observe workload failed: 2
```

Consequences in that run:

- performance series: not completed;
- baseline learn: skipped;
- repeated no-drift check: skipped;
- controlled drift check: skipped;
- privacy sentinel step: skipped because the earlier gate failed;
- raw failure artifact: preserved.

### Raw artifact

- artifact ID: `10914520707`
- artifact name: `m9-just-evidence-36266650230-1`
- artifact SHA-256: `8b6126f2bbc5a0a620f4be49559d6b2456afd781c989e1ca379eca515be3faf2`

### Protocol invalidity for accepted performance claims

The pre-freeze workflow used:

- 2 warmups per mode;
- 11 measured samples per mode.

The merged canonical M9.0 protocol requires:

- exactly 3 warmups per mode;
- exactly 15 measured samples per mode;
- alternating measured pair order.

Therefore any timing data from this pre-freeze attempt is **KILLED as accepted M9 performance evidence**, independently of the operational failure.

### Scientific interpretation

`PARTIAL`: this run is a real operational counterexample showing that the public ptrace observation path can encounter an `ESRCH` / `No such process` failure on this fast, process-dense external workload. One pre-freeze occurrence does not establish root cause or generality.

Required next action:

1. rerun the same pinned workload post-freeze under the canonical protocol with no ptrace code change;
2. preserve the result whether success or failure;
3. only if the failure reproduces, open a separate product-hardening gate and investigate root cause before modifying observer semantics.

No fix is authorized merely to obtain a green M9.1 result.
