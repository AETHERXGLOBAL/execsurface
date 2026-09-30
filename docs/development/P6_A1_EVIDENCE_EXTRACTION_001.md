# ExecSurface — P6-A1 Factual Evidence Extraction 001

Date: 2026-09-30
P6 issue: #111
Protocol: `docs/development/P6_A1_SAME_WORKLOAD_PROTOCOL.md`
Frozen workload blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`

## 1. Corrected executable run

- workflow source: `110bf2ce400d50ac2cc2ae950de2de4e3a9cba12`
- run: `36770813526`
- workflow conclusion: `success`

All **authorized** A1 lanes completed S0–S3:
- ExecSurface alpha.4: job `110076474178` — success as an observation harness;
- Harden-Runner: job `110076474096` — success;
- cicd-sensor: job `110076474202` — success.

Tetragon remained `NOT_TESTABLE_YET` by the frozen A0 boundary and was not executed.

Historical first-run negative evidence remains separately retained in `P6_A1_NEGATIVE_001_ES_EXIT2_HARNESS.md`.

## 2. Corrected-run artifacts

| Evidence | Artifact ID | Digest |
|---|---:|---|
| ExecSurface A1 | `11124230271` | `sha256:6eea7bd914814e2690f660c145074826fa9cd9638b671b0133ccebfeb62735b0` |
| Harden-Runner workload-side | `11123990489` | `sha256:431b42624640e50fcfc2e64803c600788e855f67ba355e9035885dea724c6eff` |
| A1 freeze | `11123870581` | `sha256:9072fc9e9c68716b38b4215944366a43bcb86efa8c054062ba3af1435f0095e7` |
| cicd-sensor attestation | `11123269064` | `sha256:6dad33b00832f1569fa1f3dd3af2fb98771b9d8761a2215ecb15d0e8499b8ce9` |
| cicd-sensor debug/runtime events | `11122519443` | `sha256:9852878eb2faf74a3019faa6bb53f7fe80b9d23acc072709d08217b1a166bf58` |
| cicd-sensor workload-side | `11122394384` | `sha256:73b2be95e5657883a8bf69d6df1d8985763a4ffee2ff6298ea6ef6bb59f3fb2a` |

## 3. ExecSurface alpha.4 — native results retained as native semantics

Exact public identity:
- release `v0.1.0-alpha.4`;
- source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- downloaded release checksum verified before execution.

S0 baseline learned once:
- baseline digest: `sha256:36256b582d59622b3e795fd2470a68cbbc1382e6b7c92ca2a2caeafc98e6d2f1`;
- effects: `113`.

No relearn occurred after results.

| Scenario | Exit | Native result | Factual evidence |
|---|---:|---|---|
| S0_CONTROL | `0` | `pass` | no findings |
| S1_CHILD_PROCESS_EXPANSION | `10` | `review` | added `process_exec` for `/bin/bash` relative to S0 baseline |
| S2_NETWORK_DESTINATION_EXPANSION | `2` | `ERROR` / `INCOMPLETE` | `raw observation is incomplete and cannot form a trusted canonical surface`; no policy JSON was formed |
| S3_FILE_WRITE_EXPANSION | `10` | `review` | added process/file effects including write/create/truncate to `$TMP/execsurface-p6/s3.txt` |

S3 evidence specifically included:
- actor `/bin/bash`;
- `file_open` with write/create/truncate intent for `$TMP/execsurface-p6/s3.txt`;
- `file_write` on the same canonical temp path with `kernel_fd_resolved` resolution;
- later read by `/usr/bin/cat`.

### ExecSurface A1 classifications

- CP1 / S1: `OBSERVED_SUPPORTED` for alpha.4's own v2 process-exec drift representation.
- CP2 / S2: `INCOMPLETE`. This is fail-closed evidence, not `NOT_OBSERVED` and not a false PASS.
- CP3 / S3: `OBSERVED_SUPPORTED` for path-based write/open drift evidence in public v2 semantics. This is not promoted to P4/v3 successful-object authority.
- CP4: not exercised as a separate public-alpha attestation lane in A1; P5 research evidence remains separate and is not silently projected onto alpha.4.

## 4. Harden-Runner — factual local evidence

Exact action:
`step-security/harden-runner@e14015d583714f6e62063499dc959a02595150a1`

Frozen configuration:
- `egress-policy: audit`;
- `disable-file-monitoring: false`;
- `disable-telemetry: false`;
- GitHub-hosted public runner.

The action initialized:
- a process monitor;
- a TCP connect network monitor;
- UDP send/sendmsg monitoring;
- file monitoring for `/home/runner/work/execsurface/execsurface` plus selected agent/system paths.

All S0–S3 workload commands themselves completed successfully.

### CP2 / S2 directly evidenced in retained GitHub job log

The Harden-Runner post-step log recorded:
- DNS: `example.com` -> `172.66.147.243` in this run;
- endpoint: `172.66.147.243:443`;
- process: `curl`;
- PID: `2808`.

Classification:
`OBSERVED_SUPPORTED` for destination + process factual observation.

Boundary:
This does not prove socket-success semantics equivalent to ExecSurface P4 `P4.NET.CONNECT_DESTINATION`; A1 records only the fields emitted.

### CP1 / S1

The retained local log states that a process monitor was installed, but the job log did not emit an explicit event line for the S1 child `/bin/bash` / `/usr/bin/printf` chain. The external StepSecurity security-insights URL was emitted, but it is not imported as machine-verifiable local evidence in this gate.

Classification:
`OBSERVED_PARTIAL` — monitor capability and job correlation are evidenced, but the frozen retained local output does not expose the specific S1 process lineage needed for a stronger classification.

### CP3 / S3

The retained local log explicitly says the project file monitor was added for:
`/home/runner/work/execsurface/execsurface`

The frozen S3 write target was:
`/tmp/execsurface-p6/s3.txt`

No exact S3 file-write event was emitted in the retained job log. This is a scope/fixture fact under this lane, not a global product inability claim.

Classification:
`NOT_OBSERVED` for the exact S3 target in the retained A1 local evidence.

The same log did emit repository-workspace file events during checkout, confirming active file monitoring in its declared workspace scope.

### CP4

The pinned action declared no action outputs. It emitted a security-insights URL and job correlation ID, but A1 does not reinterpret those as a run-level attestation predicate.

Classification:
`OBSERVED_PARTIAL` for run/job correlation; no machine-consumable attestation-equivalent artifact was demonstrated in this gate.

Operational/privacy context retained:
- action configuration showed `api_url=https://agent.api.stepsecurity.io/v1`;
- `telemetry_url=https://prod.app-api.stepsecurity.io/v1`;
- `disable_telemetry=false`.

This is recorded as architecture/data-flow context, not a score.

## 5. cicd-sensor — runtime event and attestation evidence

Exact action:
`cicd-sensor/cicd-sensor-action@6511eb44c91d71b2b93d71193b1bf2cb18352f66`

Bundled sensor:
`v0.0.45`

Retained debug corpus contained `5089` runtime events in the inspected artifact, including:
- `4786` `file_open`;
- `184` `process_exec`;
- `31` `file_move`;
- `31` `file_remove`;
- `22` `unix_socket_connect`;
- `17` `network_connect`;
- `14` `domain`;
- `4` `file_link`.

Every sampled scenario event carried the GitHub job identity block including project, run ID, job, workflow reference, workflow SHA and actor.

### CP1 / S1

Direct event sequence included:
- workload `/bin/bash ... S1_CHILD_PROCESS_EXPANSION`;
- child `/bin/bash -c ...` with the workload shell as ancestor;
- `/usr/bin/printf` with the child bash as direct ancestor and the workload bash further up the chain.

Classification:
`OBSERVED_SUPPORTED` for process execution + lineage + run/job binding in the retained event schema.

### CP2 / S2

Direct events included:
- `/usr/bin/getent ahosts example.com`;
- domain events for `example.com`;
- `/usr/bin/curl ... https://example.com/`;
- DNS connection to `127.0.0.53:53`;
- candidate destination observations for Cloudflare addresses;
- an explicit TCP `network_connect` from curl to `104.20.23.154:443` in the retained event corpus.

The run-level predicate also listed:
- domain `example.com`;
- network addresses including `104.20.23.154` and `172.66.147.243`.

Classification:
`OBSERVED_SUPPORTED` for process-bound destination evidence in this event schema.

Boundary:
Presence of `network_connect` is not silently equated to ExecSurface P4 success-authority semantics; A1 records the native event facts only.

### CP3 / S3

Direct event evidence included:
- process `/bin/bash ... S3_FILE_WRITE_EXPANSION`;
- `file_open.path=/tmp/execsurface-p6/s3.txt`;
- `is_write=true` for the bash write;
- later `file_open` by `/usr/bin/cat` with `is_read=true`.

Classification:
`OBSERVED_SUPPORTED` for path-based file write/read facts plus process lineage and job binding.

### CP4 run evidence / attestation

`predicate.json` contained run-level fields:
- network address list;
- domain list including `example.com`;
- native result extension: `passed`;
- GitHub job identity extension with:
  - provider `github`;
  - project `AETHERXGLOBAL/execsurface`;
  - run `36770813526`;
  - job `cicd-sensor`;
  - run attempt `1`;
  - workflow reference;
  - workflow SHA `110bf2ce400d50ac2cc2ae950de2de4e3a9cba12`;
  - actor identity;
- build start/finish timestamps.

Classification:
`OBSERVED_SUPPORTED` for machine-consumable run/job-bound Runtime Trace predicate evidence.

Boundary:
The native predicate result string `passed` is **not** mapped to ExecSurface `PASS` and does not grant proposition authority by name.

## 6. Bounded factual matrix

| System | CP1 process / S1 | CP2 network / S2 | CP3 file / S3 | CP4 run evidence |
|---|---|---|---|---|
| ExecSurface alpha.4 | `OBSERVED_SUPPORTED` | `INCOMPLETE` fail-closed | `OBSERVED_SUPPORTED` | not separately exercised in A1 |
| Harden-Runner | `OBSERVED_PARTIAL` in retained local output | `OBSERVED_SUPPORTED` | `NOT_OBSERVED` for `/tmp` fixture target | `OBSERVED_PARTIAL` job/link correlation, no attestation-equivalent artifact demonstrated |
| cicd-sensor | `OBSERVED_SUPPORTED` | `OBSERVED_SUPPORTED` | `OBSERVED_SUPPORTED` | `OBSERVED_SUPPORTED` Runtime Trace predicate |
| Tetragon | `NOT_TESTABLE_YET` | `NOT_TESTABLE_YET` | `NOT_TESTABLE_YET` | `NOT_APPLICABLE/NO_ASSUMPTION` |

This table is not a score and does not imply an overall ordering.

## 7. Scientific reading

A1 demonstrates that the frozen same-workload corpus is capable of exposing meaningful architectural differences without collapsing native semantics:

- ExecSurface public alpha.4 demonstrates deterministic baseline-relative drift and fails closed on the observed network incompleteness;
- Harden-Runner demonstrates strong job-integrated network attribution in retained logs, while the exact S3 `/tmp` path falls outside the workspace file-monitor scope shown in the retained local evidence;
- cicd-sensor exposes rich event-level process/network/file facts and a machine-consumable Runtime Trace predicate tied to GitHub job identity;
- none of these facts proves global superiority or semantic equivalence.

The next falsification question is not “which product saw more events?” It is whether ExecSurface's claimed verification/authority distinctions survive adversarial identity substitution, replay, incompleteness and false-PASS tests under P6-A2.