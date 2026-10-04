# P8-A3.4 — Experimental Typed-Evidence Consumer Contract Hardening

Date: 2026-10-04  
Tracking: #155  
Implementation PR: #156  
Parent implementation: #153 / PR #154  
External evidence authority: #114

## Decision

`EXPERIMENTAL_TYPED_EVIDENCE_CONSUMER_TRIAL_READY_BOUNDED`

This decision means the current experimental typed-evidence artifact is sufficiently hardened for a **bounded external consumer trial** against an exact source revision.

It does **not** authorize:
- a public release or tag;
- stable-v1 compatibility;
- GitHub Action exposure;
- promotion of the experimental schema to a stable contract;
- P8 A1/A4/A5 closeout;
- production-adoption claims.

## C0 — Frozen source boundary

Gate opened from main:

`2fa91e73d35e74267b915de4f1b582efdc0fb612`

Accepted code/test source before this documentation-only closeout:

`c97afb8d388ab34691a01f2166e36981642676d9`

Experimental surface remains:

`execsurface observe --evidence-output PATH -- COMMAND [ARGS...]`

Legacy `observe -- COMMAND` remains raw schema v2.

## C1 — Machine-readable experimental contract

Added:

`schemas/execsurface-typed-evidence-experimental-v1.schema.json`

The contract explicitly requires:
- `schema_version = 1`;
- `report_kind = typed_observation_evidence`;
- `stability = experimental`;
- recognized `linux-ptrace-metadata-v2` / Linux x86_64 / metadata-only profile;
- typed collection health;
- bounded fd-write effects;
- explicit unsupported capabilities;
- explicit limitations and non-claims;
- embedded raw observation schema v2.

The schema is experimental and non-stable. A future incompatible semantic change must not silently reuse this contract as though it were stable-v1.

## C2 — Independent reference consumer

Added stdlib-only consumer:

`scripts/p8_a3_typed_evidence_reference_consumer.py`

It imports no ExecSurface Rust implementation.

The consumer independently verifies:
- envelope identity/version/stability;
- backend/profile boundary;
- raw-v2 top-level shape;
- warning-code binding;
- typed health against raw complete/warning state;
- unsupported-capability profile;
- typed limitations against raw backend limitations;
- exact non-claim set;
- event sequence uniqueness/order;
- process/TID binding for every retained fd-write;
- one-for-one typed effect correspondence with raw fd-write evidence;
- exact bounded guarantee vocabulary;
- absence of verdict, byte-count, state-root, custody and other authority-inflating fields.

Accepted result:

`C2_REFERENCE_CONSUMER_FAIL_CLOSED_PASS`

## C3 — Mutation/substitution red team

Starting from a real native ptrace evidence artifact, **27/27 mutations were rejected**.

Attacks included:
1. schema-version substitution;
2. report-kind substitution;
3. stability promotion;
4. backend-profile inflation;
5. health-state substitution;
6. warning-code substitution;
7. raw-complete mutation;
8. raw backend identity substitution;
9. typed target substitution;
10. typed actor substitution;
11. proposition substitution;
12. guarantee substitution;
13. dropping a supporting raw write;
14. injecting a typed effect;
15. removing a typed effect;
16. repeating a typed effect;
17. injecting a verdict;
18. injecting byte count;
19. injecting state root;
20. injecting custody;
21. removing an explicit non-claim;
22. unsupported-capability drift;
23. limitation drift;
24. raw warning injection;
25. raw write-path substitution;
26. raw exec/actor substitution;
27. duplicate event sequence.

Accepted workflow run:

`37199410250` — **SUCCESS**

Evidence artifact:

`11302497151`

Artifact digest:

`sha256:db39f70ca90f6adee6aeaaac5f95b9e060b36291aa84a74d5cd379011cae7507`

Accepted result:

`C3_27_OF_27_MUTATIONS_REJECTED`

## C4 — Transport and output-path integrity

### Target stdout/stderr isolation

A target emitted unique stdout and stderr payload sentinels while also performing the observed write.

The dedicated typed-evidence file remained valid and independently consumable, and neither payload sentinel appeared in the evidence artifact.

Accepted result:

`C4_DEDICATED_EVIDENCE_TRANSPORT_ISOLATED_PASS`

### Failure-first output collision finding

Initial run:

`37198927682` — **FAILURE retained**

Initial source:

`e5a507c2146145ecc8348f6c9a5bbfa22336e940`

Artifact:

`11302211032`

Artifact digest:

`sha256:ea34296a0f514dae3f1c38ada224e38b67d0f89b9bfd1b5588e6a36f391e8991`

The target wrote `WORKLOAD-STATE` to the same path selected for `--evidence-output`.

Observed initial behavior:
- ExecSurface returned exit code 0;
- after target completion, the typed evidence JSON replaced the workload-owned file.

This is retained as material negative evidence.

### Bounded fix

The typed-evidence writer now refuses an output path whose resolved identity overlaps a retained successful `file_descriptor_access/write` path from the same observation.

Tests cover:
- direct path collision;
- symlink alias collision where the path can be resolved;
- workload bytes remain unchanged after rejection.

Accepted behavior:
- collision returns explicit ExecSurface error / exit 2;
- workload-written bytes survive;
- evidence is not written over the workload file.

Accepted result:

`C4_OBSERVED_FD_WRITE_OUTPUT_COLLISION_FAIL_CLOSED_PASS`

### Important boundary

This guard is intentionally bounded by what ExecSurface actually observed.

It does **not** claim collision detection for file mutations outside the current observer coverage, including un-attributed memory-mapped I/O or io_uring data access.

The existing backend limitation remains authoritative.

### Evidence-write failure is not rollback

A separate test forces evidence output failure after a workload side effect.

Accepted result:
- ExecSurface returns explicit error / exit 2;
- the target side effect remains present;
- no rollback claim is made.

Decision:

`EVIDENCE_OUTPUT_FAILURE != TARGET_ROLLBACK`

## C5 — Compatibility, privacy and regression

All required gates passed on the same accepted code source:

- P8-A3.4 consumer-contract run `37199410250`: **SUCCESS**
- P9.3 Compatibility Contract run `37199410306`: **SUCCESS**
- full CI run `37199410248`: **SUCCESS**
- P8-A3 typed-evidence implementation regression run `37199410343`: **SUCCESS**
- Adversarial Regression run `37199410253`: **SUCCESS**
  - Ubuntu 22.04: PASS
  - Ubuntu 24.04: PASS

Adversarial artifacts:
- Ubuntu 22.04: `11301519612`
  - `sha256:43a4694164b1a6520bc6b09236bdb89721c19cab7a73fcb6cce6b3a546660247`
- Ubuntu 24.04: `11301778573`
  - `sha256:642a24fe616291e38a661c4932d80cdd9a8b6db9d005b1f22e1a32c82a5521bf`

P8-A3 typed-evidence artifact:
- `11301642980`
- `sha256:4ed58eab54f41e682057da39c570f6801be8c4122282ca81697d217eb68200f1`

The argv-only privacy sentinel remained absent from the evidence file.

The no-flag `observe` path retained the raw schema-v2 top-level object and did not gain typed-evidence fields.

The GitHub Action contract remained unchanged and green under P9.3.

## Durable CI improvement

`.github/workflows/adversarial-regression.yml` now runs when the typed-evidence producer or its CLI integration changes.

This closes a governance gap where evidence-layer changes could previously avoid the Ubuntu 22.04 / 24.04 adversarial matrix.

## Fixed-role review

### Innovation Lead

High-value finding:
output-path interference was a concrete integrity risk not covered by the original producer-centric implementation gate.

Accepted innovation:
solve it at the evidence-output boundary with a narrow observed-write collision guard rather than expanding collection, introducing state snapshots, or adding custody semantics.

### Anti-Drift / Goal Alignment

Blocked:
- Action expansion;
- release work;
- whole Semantics v3 promotion;
- file-content capture;
- exact byte-count claims;
- state-root/custody/signing scope;
- treating internal consumer tests as external validation.

### Independent Falsifier

Retained the initial collision failure and required 27 mutation/substitution attacks plus transport, privacy and compatibility attacks.

No failed assertion was weakened to obtain the accepted result.

### Critical Milestone Review

The accepted claim is limited to:

> The experimental typed-evidence v1 artifact can be consumed fail-closed by a separate reference consumer under the tested Linux x86_64 native-ptrace profile, with preserved raw-v2 semantics and explicit non-claims.

It is not a universal interoperability, adoption, production-readiness or stable-schema claim.

## C6 — Final bounded decision

`P8_A3_4_EXPERIMENTAL_TYPED_EVIDENCE_CONSUMER_TRIAL_READY_BOUNDED`

The next legitimate step for this feature is an **external consumer trial against an exact source commit**, preserving the evaluator's initial result before any AETHER X assistance.

That future trial must remain separate from P8 A1/A4 qualification unless the applicable external-evidence protocol explicitly accepts the tested target/version and independence conditions.

P8 #114 and P9.5 #140 remain open.
