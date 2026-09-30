# ExecSurface — P6-A1 Same-Workload Observation Protocol

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Predecessor: `P6_A0_OVERLAP_MATRIX_FROZEN`
Status: **PREREGISTERED — NO A1 RESULT YET**

## 1. Question

For one frozen deterministic workload, what factual process/network/file/run-evidence observations are exposed by ExecSurface, Harden-Runner, and cicd-sensor on comparable GitHub-hosted Linux runners?

Tetragon remains a primary comparator but its live lane is not authorized until the A0 execution boundary is satisfied by an exact-source build workflow or immutable image digest. Its absence from the first A1 run is `NOT_TESTABLE_YET`, not a competitive result.

## 2. Frozen workload

Path:
`experiments/p6-competitive-falsification/workload.sh`

Git blob SHA:
`c5986f1acd8bc79a3945e613acc128d5c8a9d168`

Workload version marker:
`p6_workload_version=1`

Any content change requires a new preregistered A1 workload version. Existing A1 evidence remains tied to this blob.

## 3. Frozen scenarios for A1

A1 executes only S0–S3. S4/S5 remain reserved for A2 adversarial work.

### S0_CONTROL
Runs one deterministic `/usr/bin/printf` control operation.

Purpose:
- establish control observation;
- detect false REVIEW/incompleteness only within each tool's own declared semantics;
- no cross-product PASS equivalence is inferred.

### S1_CHILD_PROCESS_EXPANSION
Runs `/bin/bash -c` which invokes fixed `/usr/bin/printf`.

Purpose:
- CP1 process execution identity;
- parent/step/workflow attribution where exposed.

### S2_NETWORK_DESTINATION_EXPANSION
Runs HTTPS to logical destination `example.com:443`.

Rules:
- `example.com` is the frozen logical destination;
- runtime-resolved IPs are evidence, not preregistered identity constants;
- DNS/IP variation does not rewrite the scenario;
- connection success/attempt semantics are recorded only if the tool exposes them.

Purpose:
- CP2 outbound destination;
- actor/step/workflow attribution;
- no claim of socket-success equivalence from destination presence.

### S3_FILE_WRITE_EXPANSION
Writes fixed content to `/tmp/execsurface-p6/s3.txt`.

Purpose:
- CP3 file write/mutation observation;
- actor/step/workflow attribution;
- pathname observation is not promoted to kernel object identity.

## 4. First A1 lanes

### Lane ES — ExecSurface public alpha.4 reference

Identity:
- release `v0.1.0-alpha.4`;
- source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- Linux x86_64 release asset with published checksum.

Procedure:
1. download + checksum-verify exact public release asset;
2. learn S0 baseline once;
3. run checks for S0, S1, S2, S3 without changing the baseline;
4. preserve report JSON, baseline, status code, resolved destination evidence, runner identity.

Expected outcome is **not preregistered as PASS/REVIEW** for S1–S3. Observed verdicts are data. No baseline relearn is allowed after results.

### Lane HR — Harden-Runner

Identity:
`step-security/harden-runner@e14015d583714f6e62063499dc959a02595150a1`

Configuration:
- GitHub-hosted public Linux runner;
- `egress-policy: audit`;
- file monitoring remains enabled;
- no production secret or private endpoint is used.

Procedure:
1. Harden-Runner is first monitoring step before checkout/workload;
2. S0–S3 run as distinct named steps from the same frozen workload blob;
3. preserve GitHub job logs and summary/security-insight link evidence where emitted;
4. do not scrape private/paid service data and do not infer an attestation artifact from a UI link.

Privacy note:
The pinned action may communicate telemetry/insight data to StepSecurity under its documented operating model. A1 contains only the deterministic public workload and normal public GitHub CI metadata. This external-service behavior is recorded as operational/privacy context, not treated as a score.

### Lane CS — cicd-sensor

Identity:
`cicd-sensor/cicd-sensor-action@6511eb44c91d71b2b93d71193b1bf2cb18352f66`

Configuration:
- default bundled sensor `v0.0.45` as frozen in A0;
- attestation artifact enabled;
- HTML report disabled for minimal data duplication;
- debug artifact enabled because A1 needs event-level evidence for CP1/CP3 and the documented runtime-trace predicate intentionally aggregates/omits those details.

Procedure:
1. sensor main step starts before S0–S3;
2. execute S0–S3 as distinct steps;
3. sensor post step finalizes artifacts;
4. preserve `cicd-sensor-attestation` and debug artifact metadata/digests;
5. inspect `predicate.json` separately from debug/event evidence.

Important semantic boundary:
- network addresses/domains and run/job identity in the runtime-trace predicate can support CP2/CP4 factual observations;
- dedicated file-access evidence is not assumed from `predicate.json` because the pinned docs explicitly state it is not emitted there;
- event-level process/file facts, if any, must come from retained debug/log evidence and are reported separately.

## 5. Tetragon lane

Initial A1 run marks Tetragon `NOT_TESTABLE_YET` unless an immutable executable pin is established before execution.

No floating container tag is permitted. No absence is counted as a failure.

## 6. Factual extraction record

For each lane/scenario/proposition, record only fields actually evidenced:

- comparator identity;
- run/job/step identity;
- scenario ID + workload blob SHA;
- process executable/lineage fields actually exposed;
- network domain/IP/destination fields actually exposed;
- file path/operation fields actually exposed;
- run-level artifact/predicate fields actually exposed;
- observer health/loss/incomplete signal if exposed;
- native tool verdict/detection/result, explicitly labelled as native semantics;
- evidence location + digest where available;
- classification from the P6 vocabulary.

## 7. Forbidden normalization

A1 MUST NOT:
- map `detected`, `review`, `pass`, `audit`, or similar native labels into a common verdict;
- equate a network domain list with socket success authority;
- equate pathname writes with object-identity authority;
- equate process-name visibility with complete lineage;
- treat no declared output as no capability;
- use external dashboard availability as machine-verifiable attestation;
- compare performance;
- assign numeric points, tiers, rankings, or overall winner.

## 8. Acceptance / failure classification

A1 may close as `P6_A1_OBSERVATION_CORPUS_COMPLETE_BOUNDED` only if all **authorized** primary lanes execute S0–S3 and their available raw evidence is retained and extracted without semantic laundering.

If a comparator cannot execute due runner/tool restrictions, record `NOT_TESTABLE` or `FIXTURE_OR_INFRA_FAILURE` and close as `P6_A1_INCOMPLETE` if needed.

Any ExecSurface false-PASS or false-REVIEW conclusion requires proposition-specific evidence and is deferred to A2 unless A1 itself exposes an unambiguous counterexample.

## 9. First executable run

The first A1 workflow will execute ES, HR, and CS as independent jobs on `ubuntu-24.04`. Each job records runner image/kernel and the workload blob SHA. Tetragon remains blocked by its A0 live-execution boundary.

No A1 decision is authorized merely because the workflow completes successfully; results require a separate evidence-extraction and decision step.