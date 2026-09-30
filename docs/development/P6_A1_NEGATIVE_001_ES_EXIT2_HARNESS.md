# P6-A1 Negative Evidence 001 — ExecSurface S2 Incompleteness / Harness Stop

Date: 2026-09-30
P6 issue: #111
A1 protocol: `docs/development/P6_A1_SAME_WORKLOAD_PROTOCOL.md`

## Historical run retained

- workflow source: `bcf0da8e42648541b67a383db3c3c76ebc5b5790`
- run: `36770551463`
- ExecSurface job: `110075604095`
- ExecSurface artifact: `11123635852`
- artifact digest: `sha256:3fbb7cffad61a614bc29981d2369c4b5bf9862ba28196ea87f2ce8b75b27e8f1`
- artifact size: `7219` bytes

Other lanes in the same run:
- Harden-Runner job completed successfully through S0–S3;
- cicd-sensor job completed successfully through S0–S3 and emitted attestation/debug artifacts.

## Exact ExecSurface sequence before harness stop

- alpha.4 release asset checksum: PASS;
- S0 baseline learn: PASS;
- learned baseline digest: `sha256:36256b582d59622b3e795fd2470a68cbbc1382e6b7c92ca2a2caeafc98e6d2f1`;
- learned effects: `113`;
- S0 check exit: `0`;
- S1 check exit: `10`;
- S2 check exit: `2`;
- S3 was not executed because the workflow harness rejected exit `2`.

Captured S2 stderr:

```text
ExecSurface: ERROR
execsurface: raw observation is incomplete and cannot form a trusted canonical surface
```

No S2 report JSON was produced because alpha.4 rejected the incomplete raw observation before a policy verdict report could be formed.

## Classification

The **ExecSurface S2 result is real product evidence**, not a fixture failure:

`INCOMPLETE` / native ExecSurface `ERROR`

It demonstrates fail-closed behavior under this frozen network workload. It is not a false PASS.

The **workflow termination after exit 2 is a harness defect**:

`FIXTURE_OR_INFRA_FAILURE` at the orchestration layer only.

A1 is an observation corpus. Native ExecSurface exit `2` is a valid factual observation and must be retained while allowing later scenarios to execute. The harness incorrectly allowed only 0/10/20.

## Permitted correction

The smallest correction is authorized:

- keep the workload blob unchanged: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`;
- keep S0 baseline and all scenarios unchanged;
- keep all product pins unchanged;
- keep all acceptance/classification vocabulary unchanged;
- allow stable native exit `2` to be recorded as error/incomplete evidence and continue to S3;
- require JSON only for exits 0/10/20; retain stderr/stdout and status for exit 2.

No threshold, workload, proposition, or comparison criterion is changed.

## Scientific consequence

This run remains part of the P6 evidence record and must not be replaced or erased by a corrected rerun.