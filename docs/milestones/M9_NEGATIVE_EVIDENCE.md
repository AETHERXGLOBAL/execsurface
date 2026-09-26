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

Required next action was canonical post-freeze reproduction before any observer fix. That reproduction is recorded below as NE-002.

No fix is authorized merely to obtain a green M9.1 result.

---

## NE-002 — Canonical post-freeze `casey/just` ESRCH reproduction

Classification: **POST-FREEZE / ACCEPTED NEGATIVE EXTERNAL EVIDENCE**
Claim status: `PARTIAL`
Root cause: `OPEN`

### Provenance

- branch: `milestone/m9-1-external-workloads`
- workflow head: `ea6d8011f807143e582cb1e5fb115c90607af7cd`
- workflow run: `36267932394`
- job: `108476087232`
- evidence record ID: `M9-ZC-JUST-POSTFREEZE-001`
- evidence record SHA-256: `288974ca9526f1fd506c915900588fbe956a13014fd28bf2d9c612cd0ca6d91c`
- upstream: `casey/just`
- upstream revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`
- independence class: `ZERO_CONTACT_EXTERNAL_REPRO`
- maintainer coordination: none
- upstream modification by AETHER X: none
- public ExecSurface version: `0.1.0-alpha.2`
- public release source SHA: `c6cabf1b4d1399898b9643a7fce011664aac0918`

### Host

- OS: Ubuntu 24.04
- kernel: `6.17.0-1022-azure`
- architecture: x86_64
- CPU: AMD EPYC 7763 64-Core Processor
- runner image: `ubuntu24`

### Canonical setup results

- public crates.io install: PASS;
- `execsurface doctor`: PASS;
- pinned direct upstream workload reproduction: PASS;
- privacy: metadata-only confirmed;
- prohibited sensitive payload collected: false;
- canonical evidence semantic validator: PASS.

### Frozen direct warmups

The three required direct warmups completed:

- `5131.698421 ms`
- `5136.315395 ms`
- `5088.200901 ms`

### Reproduced failure

The **first ptrace warmup** failed before any measured 3x15 series began:

```text
ExecSurface: ERROR
execsurface: observer OS error: No such process (os error 3)
```

The canonical runner recorded:

```text
M9_JUST_CASE_PARTIAL: ptrace observe warmup-1 failed rc=2
```

Consequences:

- ptrace warmups completed: 0;
- measured direct samples: 0;
- measured ptrace samples: 0;
- canonical `performance`: `null`;
- baseline learn: NOT_RUN;
- repeated no-drift checks: NOT_RUN;
- controlled drift: NOT_RUN;
- comparability: false;
- completeness: false;
- workflow conclusion: FAILURE by explicit fail-closed gate.

No partial timing result is promoted to accepted performance evidence.

### Raw artifact

- artifact ID: `10914805829`
- artifact name: `m9-just-post-freeze-36267932394-1`
- uploaded ZIP SHA-256: `875a12b63b55cd099d80944a2c0b73f340f8066e97f744c418f9d0f43df35fb1`

### Separate harness defect

The same workflow exposed a secondary evidence-harness defect after the semantic validator had already returned `M9_EVIDENCE_VALID`: `sha256sum -c` was executed from the repository root while the checksum file contains paths relative to its evidence directory.

This is **not** the cause of the ptrace failure. It was corrected separately by changing checksum verification to execute inside `m9-evidence/just-post-freeze`.

### Scientific interpretation

`PARTIAL / OPEN ROOT CAUSE`: the earlier `ESRCH` counterexample reproduced after the protocol freeze with the same pinned external workload and without any ptrace implementation change. This upgrades the concern from a one-off pre-freeze observation to a reproducible product-hardening problem.

It does **not** yet prove:

- which ptrace request returned `ESRCH`;
- that the race is benign;
- that any `ESRCH` may be ignored;
- that events were or were not lost;
- that the failure generalizes beyond the tested host/workload class.

Tracking hardening gate: **Issue #55 — M9 hardening — ptrace ESRCH on process-dense external workload**.

Required next action is diagnostic instrumentation that preserves fail-closed behavior while identifying the exact failing ptrace operation and lifecycle state. No semantic repair is authorized before that evidence exists.
