# ExecSurface — P3 V5-R2 R4 Fresh Checks + Meaningful-Drift Sentinel Result

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — P3_R4_CHECK_SET_COMPLETE_SENTINEL_PROTECTED**

## Governance

Fixed roles retained:
- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team
- Independent Milestone Reviewer

Dynamic specialists:
- Go/GCC nondeterminism
- canonical diff / multi-run set semantics
- Rust research tooling
- Linux ptrace completeness
- CI reproducibility
- provenance/evidence sealing
- supply-chain false-PASS analysis

No public integration is authorized by this result.

## Frozen predecessor

R3-C decision: `P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED`.

Learning evidence:
- workflow `36573676797`
- artifact `11035133723`
- artifact SHA-256 `sha256:a98a1f022e601f0b0653474df8c4500f33fa5670f7e25f105740b2e79bd2fd26`
- raw learning-set digest `sha256:437a9f8fca9f0ff4c8573c1cfee06b46cb325d1d1a890649ba536acd8e2e648d`
- projected learning-set digest `sha256:16e8519c83ea374359d1e44872d570ccc15be88b7ccf8f9e3b54b5e3d6b7ab30`
- explicit accepted-variable selection: **empty**

R4 protocol was frozen before measurement in `docs/development/P3_V5R2_R4_CHECK_AND_SENTINEL_PROTOCOL.md`.

## Preserved pre-measurement failure

First R4 workflow:
- run `36575710731`
- source `cebf808a5542cce5ae1099772cef79bed65a0e49`
- artifact `11037271196`
- artifact digest `sha256:4f2963f247576f4e5d726f18b97d6cd8d41805ff98b48bdee25096db30738909`

It stopped at Rust 1.90 `rustfmt --check` for the new R4 evaluator. No FZF check and no sentinel observation ran. The failure remains `FORMATTING_ONLY / PRE_MEASUREMENT`; only a rustfmt-only source correction was authorized.

## Accepted R4 execution

Source:
`638875a87ab6074353a2d33ee2d296f6ba3f5077`

Workflow:
`36576614366`

Job:
`109433761711`

Evidence artifact:
`11037682900`

Artifact SHA-256:
`sha256:99f0acef1213a94f08ea6ff5d5908da927098a80505cbc1f4fec1eb74ee4afd1`

Pre-measurement gates:
- frozen R3-C acceptance boundary verified;
- exact R2-C learning artifact digest verified;
- all extracted learning evidence checksums verified;
- sentinel precheck: `R4_SENTINEL_PRECHECK_NO_COLLISION`;
- research-only collector injection PASS;
- rustfmt PASS;
- clippy `-D warnings` PASS;
- `execsurface-observe` regression PASS;
- M11 shared-FD public contract 6/6 PASS;
- M12 adversarial 6/6 PASS;
- ptrace Linux suite 11/11 PASS;
- frozen V2 GCC grammar 7/7 PASS.

## Six unchanged checks

Exactly six fresh unchanged checks executed with no replacement.

Admission:
- check 1: observe=0, baseline=0, evaluator=0, admitted
- check 2: observe=0, baseline=0, evaluator=0, admitted
- check 3: observe=0, baseline=0, evaluator=0, admitted
- check 4: observe=0, baseline=0, evaluator=0, admitted
- check 5: observe=0, baseline=0, evaluator=0, admitted
- check 6: observe=0, baseline=0, evaluator=0, admitted

Aggregate: **6/6 admitted**.

Frozen metrics in check order:

| Metric | C1 | C2 | C3 | C4 | C5 | C6 |
|---|---:|---:|---:|---:|---:|---:|
| raw single-baseline findings | 22314 | 444 | 404 | 409 | 409 | 414 |
| GCC ephemeral eligible findings | 4 | 4 | 4 | 4 | 4 | 4 |
| residual after bounded ephemeral projection | 22310 | 440 | 400 | 405 | 405 | 410 |
| residual after explicit variance | 22953 | 522 | 482 | 487 | 487 | 490 |
| invariant core matches | 4008 | 5588 | 5600 | 5595 | 5595 | 5591 |
| observed variable matches | 46 | 100 | 100 | 100 | 100 | 99 |
| explicitly accepted variable matches | 0 | 0 | 0 | 0 | 0 | 0 |
| unseen effects | 21121 | 216 | 188 | 188 | 188 | 188 |

Medians:
- raw single-baseline findings: **411.5**
- residual after bounded ephemeral projection: **407.5**
- residual after explicit variance: **488.5**

The large first-check outlier is retained in full. It is not removed, replaced, winsorized, or used to relax the gate.

For every check:
- bounded GCC eligible findings were present (`4` each);
- randomized GCC identities were absent from projected findings;
- no non-target projection mismatch was observed;
- explicit accepted-variable matches remained zero.

The multi-run/accepted-variance layer with an empty accepted set did **not** reduce residuals below the ephemeral-only projection on any of the six checks (`checks_explicit_lower_than_ephemeral = 0`). Therefore R4 does not establish extra value for automatic/multi-run authorization beyond the bounded GCC identity projection.

## Meaningful-drift sentinel

The preregistered sentinel executed exactly once after the six unchanged checks.

Sentinel behavior:
`/usr/bin/cat /tmp/execsurface-p3-v5-sentinel-meaningful.txt >/dev/null`

Result:
- observation/evaluation pipeline PASS;
- sentinel collision precheck: none;
- sentinel unseen matches: **2**;
- marker: `R4_SENTINEL_PROTECTED`;
- sentinel was not in the accepted-variable set;
- sentinel was not removed by GCC projection;
- residual-after-explicit-variance remained non-zero.

Therefore the declared meaningful new behavior did not obtain a PASS-equivalent interpretation through the variance machinery.

## Public-contract reproof

After R4 and the sentinel, the unchanged historical M11 public shared-FD contract was rerun and remained **6/6 PASS**.

Public `v0.1.0-alpha.4`, `main`, stable `@v0.1`, and default/raw-v2 semantics remain unchanged.

## Decision

`P3_R4_CHECK_SET_COMPLETE_SENTINEL_PROTECTED`

Bounded interpretation only:
- the exact V2 GCC ephemeral identity mechanism repeatedly removed the targeted random identity component on six fresh unchanged checks;
- non-target behavior remained visible;
- no variable behavior was authorized by frequency;
- the explicit acceptance set remained empty;
- a preregistered meaningful-drift sentinel remained visible as unseen behavior;
- R4 alone does not establish a final P3 product claim.

## Next

R5 independent false-PASS / falsification gate is mandatory before any final P3 decision. It must attack the frozen R3-C/R4 boundary without changing thresholds, accepted set, workload evidence, or projection grammar.
