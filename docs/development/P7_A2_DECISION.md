# ExecSurface — P7-A2 Self-Hosted CI Consolidated Decision

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Branch: `development/post-alpha4-behavioral-integrity`

Protocols:
- `docs/development/P7_A2_SELF_HOSTED_EVIDENCE_PROTOCOL.md`
- `docs/development/P7_A2_SELF_HOSTED_CI_PROTOCOL.md`
- `docs/development/P7_A2_CONSOLIDATION_PROTOCOL.md`

## Consolidated decision

**P7_A2_SELF_HOSTED_CONTRACTS_PASS_BOUNDED**

Both separately preregistered A2 layers satisfied their original frozen criteria. The consolidation rule was committed at `4c1cc564e5ba49f24cc2c9b47781888bd4f223a1` before A2 result review and explicitly required both layers to pass; success in one layer could not hide failure in the other.

This is a bounded research result. It does not establish live customer/self-hosted runner installation, zero-assistance deployment, or public self-hosted support.

## Frozen predecessor boundary

- public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- P5 closeout: `1f4df8f45a115a1638c997107e069c6acd74c51b`
- P6 closeout: `2fd32bf61275e733ddfb11df4f382cde6deab74b`
- P7-A0 negative decision: `5cc80b7b3188b0879bcc585362579aff644c76f6`
- P7-A1 bounded decision: `1a648cb6a44eb1f87edfc44a67d2b822daab8280`

No A2 result modifies public alpha.4, stable `v0.1`, public v2 semantics, the default observer, or public `crates/`.

## A2-E — evidence-envelope subdecision

Retained subdecision:

**P7_A2_SELF_HOSTED_EVIDENCE_CONTRACT_PASS_BOUNDED**

Frozen files:
- protocol blob: `4e0e44db564a292fb8b688a9e75e4504986b5b0c`
- contract blob: `606fecd50900f3895721e50625c8d73792ee2c42`
- 16-test corpus blob: `ea23d13943c8c4e0a91b06e2ee5afe3909139fd4`

Accepted execution:
- source: `e6dc7dc2b4bc0e092b2da8d4ffca713adb852a51`
- workflow: `.github/workflows/p7-a2-self-hosted-evidence.yml`
- run: `36775802501`
- conclusion: `success`
- job: `110093267129`
- result: exact **16/16 PASS**
- artifact: `11125232540`
- artifact digest: `sha256:b8455e5ad2eecf26cccfe204bb0466c220f4d5416d3513b62531aee06370a8ed`
- artifact size: `2735` bytes

The evidence layer established, within the frozen contract, that runner/provider/machine/label metadata is `context_only`; trusted/root/self-hosted labels cannot upgrade authority; baseline/verdict environment variables cannot steer semantic state; explicit baseline references are digest-bound; verification verdict/evidence/authority/completeness remain sourced from the actual verification result; and non-complete evidence cannot be laundered into a stronger state.

## A2-P — executable package-contract subdecision

**P7_A2_SELF_HOSTED_PACKAGE_CONTRACT_PASS_BOUNDED**

Frozen files:
- protocol blob: `46b12048939f80587bbaaaf631fda0b84b16be99`
- package-model blob: `00933986a1154f1d2ea8bd215b6fc63d1a909299`
- 18-test corpus blob: `91b28a0db60ba1513edf05639045a473b79a32ce`

Accepted execution:
- source: `bbe682ec14ad764b4699e6ee8b38af14e6ad7812`
- workflow: `.github/workflows/p7-a2-self-hosted-contract.yml`
- run: `36775768709`
- conclusion: `success`
- job: `110093155010`
- result: exact **18/18 PASS**
- artifact: `11125257579`
- artifact digest: `sha256:f3736483d24fbfad4b7f2fe110c544d0ef23d438e8b97e49e7e1a1d4d52412b2`
- artifact size: `18577` bytes

### Exact public package identity

The executable probe downloaded the existing public asset:

`execsurface-v0.1.0-alpha.4-x86_64-unknown-linux-gnu.tar.gz`

Frozen package SHA256:

`55776cd130784d1c03451a14ab05ed113ffae4fa714c986666cbcadf242d8d1d`

The downloaded archive matched that digest before extraction.

Extracted binary:
- version: `execsurface 0.1.0-alpha.4`
- binary SHA256: `4a850ba66b0484bdf566d5407e12183d85adaf20e8e5e4a2a43c6d8ba84cf853`

### Baseline-integrity boundary

For the executable harness only, the exact alpha.4 binary generated an S0 baseline before wrapper invocation. Its file bytes were then frozen and supplied through an explicit `external_preapproved` test configuration.

Harness-only baseline byte digest:

`78f30a921f07e358106bd61554c96448dffa13d59af020b6c347bdbc930294ae`

Configuration digest:

`sha256:d033c98092e7de761199ad31448404919be14544b73b9a80a4e9bafb3fe0c8c0`

This fixture is test setup only. It is **not** a production recommendation to learn and approve a baseline inside the same verification job.

During the live wrapper probe, hostile CI variables were deliberately injected:
- `EXECSURFACE_BASELINE=/tmp/evil-baseline.json`
- `BASELINE_SHA256=deadbeef`
- `EXECSURFACE_VERDICT=PASS`
- `VERDICT=PASS`

They did not replace the frozen baseline/configuration or alter the native verdict/exit code. The emitted environment authority remained `execution_environment_only`, with baseline selection, verdict mutation and observer-authority upgrade all false.

### Native result preservation

The wrapper preserved the exact alpha.4 native result vector on the frozen P6 workload:

- S0_CONTROL -> `PASS`, exit `0`
- S1_CHILD_PROCESS_EXPANSION -> `REVIEW`, exit `10`
- S2_NETWORK_DESTINATION_EXPANSION -> fail-closed `ERROR/INCOMPLETE`, exit `2`
- S3_FILE_WRITE_EXPANSION -> `REVIEW`, exit `10`

Frozen vector: **`0 / 10 / 2 / 10`**.

False PASS under S1-S3: **0**.

S2 retained the explicit product error:

`raw observation is incomplete and cannot form a trusted canonical surface`

No wrapper-specific translation converted that state into PASS.

## Preserved negative evidence

P7-A0 remains:

**P7_A0_ARM64_NOT_PORTABLE**

The exact alpha.4 source can build a native arm64 binary, but its runtime/default observer explicitly refuses arm64 execution. A2 does not bypass or reinterpret that negative result.

P7-A1's historical first run remains recorded as **13/14**, artifact `11125496504`. Its assertion defect and smallest correction remain separately documented; the failed run is not rewritten as successful.

## What consolidated A2 establishes

Within Linux x86_64 and the tested alpha.4 package boundary:

- package identity can be pinned to the exact public artifact digest;
- runner metadata can be bound without becoming behavioral authority;
- trusted/self-hosted/root labels do not upgrade authority;
- approved baseline bytes can be explicitly SHA256-bound;
- untrusted CI variables cannot steer baseline/verdict under the tested wrapper;
- native ExecSurface exit codes and fail-closed behavior can be preserved unchanged;
- abstract evidence-envelope semantics and executable package behavior agree under the two independent frozen A2 gates.

## Promotion boundary

The consolidated A2 result does **not** establish:
- successful deployment on an actual customer/self-hosted runner;
- zero-assistance installation;
- every self-hosted Linux environment;
- runner-service hardening;
- Linux arm64 support;
- public GitLab support;
- public self-hosted support;
- authority from machine ownership, provider identity, labels or root privilege.

Those require separate live promotion/external-reproduction evidence.

## Next phase

P7 has now exercised its three highest-priority candidates:
- Linux arm64 parity — negative and retained;
- GitLab context adapter — bounded contract pass, live/public validation deferred;
- self-hosted CI — dual-layer evidence/package contract pass, live/public validation deferred.

Opening additional OS families merely to increase platform count would violate the P7 anti-drift boundary. The next justified step is a bounded P7 closeout, followed by P8 external validation/promotion evidence where actual external execution environments are required.
