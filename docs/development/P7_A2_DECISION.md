# ExecSurface — P7-A2 Self-Hosted CI Packaging & Evidence Decision

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Protocol: `docs/development/P7_A2_SELF_HOSTED_EVIDENCE_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**P7_A2_SELF_HOSTED_EVIDENCE_CONTRACT_PASS_BOUNDED**

The bounded research contract can package self-hosted CI context and bind verification evidence without granting behavioral authority to runner ownership, labels, provider identity, machine identity, or private-network placement.

This is a packaging/evidence-contract result only. It does not establish live self-hosted installation or public self-hosted-runner support.

## Frozen source and predecessor boundary

- public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- P5 closeout: `1f4df8f45a115a1638c997107e069c6acd74c51b`
- P6 closeout: `2fd32bf61275e733ddfb11df4f382cde6deab74b`
- P7-A0 negative decision: `5cc80b7b3188b0879bcc585362579aff644c76f6`
- P7-A1 bounded decision: `1a648cb6a44eb1f87edfc44a67d2b822daab8280`

Frozen A2 blobs:
- protocol: `4e0e44db564a292fb8b688a9e75e4504986b5b0c`
- contract implementation: `606fecd50900f3895721e50625c8d73792ee2c42`
- 16-test corpus: `ea23d13943c8c4e0a91b06e2ee5afe3909139fd4`

## Accepted execution evidence

- source SHA: `e6dc7dc2b4bc0e092b2da8d4ffca713adb852a51`
- workflow: `.github/workflows/p7-a2-self-hosted-evidence.yml`
- run: `36775802501`
- conclusion: `success`
- job: `110093267129`
- artifact ID: `11125232540`
- artifact name: `p7-a2-36775802501-1`
- artifact digest: `sha256:b8455e5ad2eecf26cccfe204bb0466c220f4d5416d3513b62531aee06370a8ed`
- artifact size: `2735` bytes

The run passed:
1. frozen predecessor/public-boundary verification;
2. exact frozen protocol/implementation/test blobs;
3. Python static syntax gate;
4. exact **16/16** adversarial corpus;
5. deterministic package/evidence reference emission;
6. candidate decision emission;
7. evidence sealing and upload.

## What A2 establishes

Within the frozen contract:

- Linux x86_64 alpha.4 package eligibility is explicit and deterministic;
- Linux arm64 remains explicitly ineligible, preserving the A0 negative result;
- runner labels are provenance context only;
- `trusted`, `prod`, `root`, `self-hosted` and similar labels cannot upgrade authority;
- runner identity substitution changes the package/evidence binding;
- environment variables cannot steer baseline selection;
- environment variables cannot override verdicts;
- baseline reference is explicit and SHA256-bound when present;
- verification verdict/evidence/observer authority/completeness remain sourced from the verification result;
- non-complete PASS combinations fail closed rather than being laundered or silently rewritten;
- unknown environment metadata cannot affect the canonical contract.

## Preserved negative evidence

P7-A0 remains:

**P7_A0_ARM64_NOT_PORTABLE**

No arm64 support claim is introduced by A2.

P7-A1 historical first execution remains recorded as 13/14 with artifact `11125496504`; it is not deleted or reclassified as a successful run. Its smallest harness correction and accepted 14/14 follow-up remain separately documented in `P7_A1_DECISION.md`.

## Promotion boundary

A2 does NOT establish:
- a successful installation on an actual customer/self-hosted CI machine;
- zero-assistance deployment;
- runner service hardening;
- public self-hosted support;
- Linux arm64 support;
- authority derived from machine ownership, labels or CI provider identity.

Any public self-hosted promotion requires a separate live deployment/reproduction gate.

## Next phase

P7 has now produced bounded evidence for its three highest-priority candidates:
- arm64 parity: negative, retained;
- GitLab context adapter: bounded contract pass, no live/public promotion;
- self-hosted packaging/evidence contract: bounded pass, no live/public promotion.

Opening additional OS families without a concrete measured use case would violate the P7 anti-drift boundary. P7 should therefore proceed to a bounded closeout rather than superficial platform-count expansion.
