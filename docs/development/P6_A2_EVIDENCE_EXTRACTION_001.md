# ExecSurface — P6-A2 Adversarial Evidence Extraction 001

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Protocol: `docs/development/P6_A2_ADVERSARIAL_PROTOCOL.md`

## Executable evidence

- workflow source: `411116ee87bbe45270b1fd8ce84b17abcecd8642`
- workflow: `.github/workflows/p6-a2-adversarial.yml`
- run: `36772303119`
- job: `110081460858`
- conclusion: `success`
- evidence artifact: `11124710931`
- artifact name: `p6-a2-36772303119-1`
- artifact digest: `sha256:aa2205eb719baf4a9168ee9694bc6da0cfc5d94fab2230db181ab3fe0a3ba8b8`
- artifact size: `24253` bytes
- expiry at capture: `2026-10-30T20:23:59Z`
- runner: Ubuntu 24.04 x64; image `20260927.320.1`

Frozen workload blob:
`c5986f1acd8bc79a3945e613acc128d5c8a9d168`

Exact public target:
- `v0.1.0-alpha.4`
- source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

The public release asset checksum was independently verified before execution.

## Frozen S0 baseline

The A2 run learned S0 exactly once and never relearned after results.

Observed baseline:
- stored digest: `sha256:36256b582d59622b3e795fd2470a68cbbc1382e6b7c92ca2a2caeafc98e6d2f1`
- baseline file SHA-256: `50fd140538af871b0874ce610bea700413b851c837bd043d07ddc9a1ce557a0b`
- executable: `/bin/bash`
- argument count: `2`
- canonical effects: `113`

## Track E — ExecSurface false-PASS attacks

| Attack | Input | Native exit | Result | A2 classification |
|---|---|---:|---|---|
| E1 | S1 child process expansion | `10` | `REVIEW` | no false PASS |
| E2 | S3 file write expansion | `10` | `REVIEW` | no false PASS |
| E3 | S2 network expansion | `2` | `ERROR/INCOMPLETE` | fail-closed, no false PASS |
| E4 | semantic payload tamper, old digest retained | `2` | `ERROR` | digest integrity rejection |
| E5 | stored digest substitution, payload retained | `2` | `ERROR` | digest integrity rejection |
| E6 | command argument-count mismatch | `2` | `ERROR` | factual public-contract rejection |

### E1 — process expansion

Native output:
- `ExecSurface: REVIEW`
- one added `ProcessExec` finding relative to the frozen S0 baseline.

The attack did not PASS.

### E2 — file write expansion

Native output:
- `ExecSurface: REVIEW`
- added process/file open/write effects, with 73 native review findings in this run.

The attack did not PASS.

### E3 — network expansion / incompleteness

Native stderr:

```text
ExecSurface: ERROR
execsurface: raw observation is incomplete and cannot form a trusted canonical surface
```

This is explicit fail-closed incompleteness, not absence and not PASS.

### E4 — payload tamper with stored digest retained

Mutation:
- original `payload.command.argument_count = 2`
- tampered value `99`
- stored baseline digest intentionally left unchanged.

Tampered file SHA-256:
`70eefc8e6c733143cb55798e04e7141ffc7850c24c3dd0b138366247ebb41bbe`

Native rejection:

```text
execsurface: baseline digest mismatch: expected sha256:36256b582d59622b3e795fd2470a68cbbc1382e6b7c92ca2a2caeafc98e6d2f1, calculated sha256:e6d812b5647536f48f31dba8840de3e2d18a681c89853f263eb43a19c6bcd868
```

No PASS.

### E5 — digest substitution with payload retained

Mutation:
- payload unchanged;
- stored digest changed to `sha256:` followed by 64 zeroes.

Tampered file SHA-256:
`9134b73ec93cbb3b618a17a00fe6232d48cb23e91c17fd731a27bd2ee14500f6`

Native rejection:

```text
execsurface: baseline digest mismatch: expected sha256:0000000000000000000000000000000000000000000000000000000000000000, calculated sha256:36256b582d59622b3e795fd2470a68cbbc1382e6b7c92ca2a2caeafc98e6d2f1
```

No PASS.

### E6 — public v2 command compatibility probe

The S0 baseline was checked against the same `/bin/bash` executable with one additional top-level argument.

Native rejection:

```text
execsurface: baseline and candidate are not comparable; command.argument_count baseline=2 candidate=3
```

This is recorded as a factual public v2 compatibility property. No broader workflow/source binding claim is inferred from it.

## Track P — exact P5-A5 reproof

The exact accepted test blob was verified before execution:

`6a64ad84d93a23c35d53e737c90508dc528d9cc1`

Target:
`experiments/p5-cross-attestation-redteam/tests/a5_graph_redteam.rs`

Result:

**12 passed / 0 failed**

Marker:
`P5_A5_EXACT_12_TEST_REPROOF_PASS`

This independently re-proved the existing cross-attestation attacks including baseline/current-surface/evidence/source/workflow substitution, loss masking, backend-name inflation, semantic duplication, and digest-domain confusion without editing the corpus.

## Track C — cicd-sensor bounded identity/integrity facts

Frozen A1 artifact identity was verified against GitHub artifact metadata before use:
- A1 run `36770813526`
- artifact `11123269064`
- artifact digest `sha256:6dad33b00832f1569fa1f3dd3af2fb98771b9d8761a2215ecb15d0e8499b8ce9`
- downloaded ZIP SHA-256 matched that same digest.

Extracted predicate SHA-256:
`079d43102504a902d38e3eee70794ece94b3b7b110b31ad98949c8432e591d1e`

### C1 — replay-context mismatch

Original embedded run ID:
`36770813526`

Synthetic expected context:
`36770813527`

Classification:
`IDENTITY_MISMATCH_EXPLICITLY_DETECTABLE_BY_CONSUMER`

This states only that an equality-checking consumer can detect the mismatch. It does not claim cicd-sensor itself performs that consumer check.

### C2 — copied JSON identity tamper

The copied predicate run ID was changed to the synthetic run ID and remained structurally valid JSON. No embedded top-level signature field existed in the predicate payload.

Tampered predicate SHA-256:
`1fe6b139b9494ed9c789f603102921b4a57bc49eb1dd9707d836d6c1614a6517`

Classification:
`EXTERNAL_INTEGRITY_REQUIRED_FOR_TAMPER_DETECTION`

This is an integration fact consistent with the frozen documentation that the predicate artifact is not signed by the action. It is not labelled a vulnerability or product defect.

### C3 — native result semantic boundary

Native predicate result:
`passed`

P6 common verdict:
`null`

The native label was deliberately not mapped to ExecSurface `PASS`.

## Gate summary

- `false_pass_count=0`
- `infra_marker_count=0`
- Harden-Runner offline attestation replay: `NOT_APPLICABLE_TO_OFFLINE_ATTESTATION_REPLAY`
- Tetragon live A2: `NOT_TESTABLE_YET`

Candidate emitted by the frozen workflow:

`P6_A2_NO_FALSE_PASS_IN_FROZEN_SCOPE_CANDIDATE`

## Scientific boundary

This evidence establishes only that no false PASS survived the frozen E1–E5 attack set and that the accepted P5-A5 cross-attestation corpus remained green in this run.

It does not establish universal absence of false PASS, global product superiority, competitor vulnerability, full backend equivalence, or a release decision.