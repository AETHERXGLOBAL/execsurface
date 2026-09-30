# ExecSurface — P7 Platform / CI Expansion Closeout Decision

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Protocol: `docs/development/P7_CLOSEOUT_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**P7_PLATFORM_CI_RESEARCH_COMPLETE_BOUNDED**

P7 is scientifically complete for its bounded research scope. The closeout preserves the negative arm64 result, the bounded GitLab context result, the dual-layer self-hosted contract evidence, and the immutable public alpha.4 boundary.

This decision does not promote any new public platform or CI support.

## Independent closeout evidence

Successful closeout workflow:
- source SHA: `9b1b7bda9d3170454c808a92e371883ea4ccf44c`
- workflow: `.github/workflows/p7-closeout.yml`
- run: `36776286847`
- conclusion: `success`
- artifact ID: `11126165644`
- artifact name: `p7-closeout-36776286847-1`
- artifact digest: `sha256:436fe9181b6e1d83a1d5684f4ee52a57530f529f160e3d936a616489f810815e`
- artifact size: `1879` bytes

The closeout gate independently re-verified exact historical run conclusions and artifact digests rather than trusting decision prose alone.

## A0 — Linux arm64

Retained decision:

**P7_A0_ARM64_NOT_PORTABLE**

- run `36774237512` remains a failed native parity run;
- native arm64 reached the frozen public alpha.4 source and exposed its explicit Linux-x86_64-only observer boundary;
- no threshold, whitelist or source modification was introduced;
- no arm64 release asset or public-support claim was created.

A0 negative artifacts remain addressable and part of the scientific record.

## A1 — GitLab CI context adapter

Accepted decision:

**P7_A1_GITLAB_CONTEXT_ADAPTER_PASS_BOUNDED**

- historical first executable run retained: `36774938918`, result 13/14 due assertion-harness defect;
- smallest harness-only correction retained separately;
- accepted corrected run: `36775178743`, exact 14/14;
- accepted artifact: `11125646709`;
- artifact digest: `sha256:b86b68056751301db72da23b2fab52939a4212192913e004bafce3254a7fd1fe`.

GitLab metadata remains context-only and cannot select baselines, change verdicts, or upgrade observer authority.

No live/public GitLab support is claimed.

## A2 — self-hosted CI contracts

Consolidated decision:

**P7_A2_SELF_HOSTED_CONTRACTS_PASS_BOUNDED**

### A2-E — evidence envelope
- run `36775802501`: success;
- exact 16/16 adversarial corpus;
- artifact `11125232540`;
- digest `sha256:b8455e5ad2eecf26cccfe204bb0466c220f4d5416d3513b62531aee06370a8ed`.

### A2-P — executable package contract
- run `36775768709`: success;
- exact 18/18 adversarial corpus;
- exact public alpha.4 release checksum verification;
- native S0-S3 vector `0 / 10 / 2 / 10`;
- zero S1-S3 false PASS;
- artifact `11125257579`;
- digest `sha256:f3736483d24fbfad4b7f2fe110c544d0ef23d438e8b97e49e7e1a1d4d52412b2`.

Runner ownership, provider identity, root/admin status and labels remain non-authoritative context. Baseline steering and verdict override from CI environment remain forbidden.

No public self-hosted support or zero-assistance deployment claim is made.

## Immutable boundary reproof

The closeout re-verified:
- `v0.1.0-alpha.4` -> `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable `v0.1` -> the same immutable source;
- P6, A0, A1 and consolidated A2 ancestry;
- no P7 research change after P6 closeout touched public `crates/`;
- negative A0 and historical A1 failure evidence remain retained.

Public v2 semantics and the default public observer remain unchanged.

## Anti-drift closeout

P7 deliberately does not open Windows, macOS, additional architectures or extra CI providers merely to increase platform count. Additional platform work requires a concrete measured use case and a separately preregistered evidence contract.

The correct next parent-program phase is **P8 — External Validation**.

## Non-claims

This decision does NOT establish:
- Linux arm64 support;
- public GitLab support;
- public self-hosted support;
- compatibility with every self-hosted runner configuration;
- cross-platform baseline interchangeability;
- endorsement or adoption by any external organization;
- promotion of P7 research into the public alpha.4 product.

P7 is therefore **scientifically closed, bounded, research-only**.
