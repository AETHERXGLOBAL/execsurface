# ExecSurface — P7-A2 Consolidated Self-Hosted Contracts Decision

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Consolidation protocol: `docs/development/P7_A2_CONSOLIDATION_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Consolidated decision

**P7_A2_SELF_HOSTED_CONTRACTS_PASS_BOUNDED**

Both separately preregistered A2 layers satisfied their original criteria without weakening:

- **A2-E — evidence envelope:** exact **16/16 PASS** plus frozen boundary/reproof gates;
- **A2-P — executable package contract:** exact **18/18 PASS** plus the exact public alpha.4 release-package probe and frozen S0-S3 execution vector.

Neither layer substitutes for the other. This consolidated result remains research-only and does not authorize public self-hosted support.

## A2-E — evidence-envelope evidence

Decision layer:
`P7_A2_SELF_HOSTED_EVIDENCE_CONTRACT_PASS_BOUNDED`

Accepted execution:
- source: `e6dc7dc2b4bc0e092b2da8d4ffca713adb852a51`
- workflow: `.github/workflows/p7-a2-self-hosted-evidence.yml`
- run: `36775802501`
- conclusion: `success`
- job: `110093267129`
- artifact ID: `11125232540`
- artifact name: `p7-a2-36775802501-1`
- artifact digest: `sha256:b8455e5ad2eecf26cccfe204bb0466c220f4d5416d3513b62531aee06370a8ed`
- result: exact **16/16 PASS**

This layer proved the abstract separation between package identity, runner context and verification evidence. Runner labels/ownership/provider identity remain context-only, environment values cannot steer baseline or verdict, and non-complete PASS combinations fail closed.

## A2-P — executable-package evidence

Decision layer:
`P7_A2_SELF_HOSTED_PACKAGE_CONTRACT_PASS_BOUNDED`

Frozen identities:
- protocol blob: `46b12048939f80587bbaaaf631fda0b84b16be99`
- package-model blob: `00933986a1154f1d2ea8bd215b6fc63d1a909299`
- test blob: `91b28a0db60ba1513edf05639045a473b79a32ce`
- P6 workload blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`

Accepted execution:
- source: `bbe682ec14ad764b4699e6ee8b38af14e6ad7812`
- workflow: `.github/workflows/p7-a2-self-hosted-contract.yml`
- run: `36775768709`
- conclusion: `success`
- job: `110093155010`
- artifact ID: `11125257579`
- artifact name: `p7-a2-36775768709-1`
- artifact digest: `sha256:f3736483d24fbfad4b7f2fe110c544d0ef23d438e8b97e49e7e1a1d4d52412b2`
- result: exact **18/18 PASS**

The workflow downloaded the exact public release asset:
`execsurface-v0.1.0-alpha.4-x86_64-unknown-linux-gnu.tar.gz`

Frozen release SHA256:
`55776cd130784d1c03451a14ab05ed113ffae4fa714c986666cbcadf242d8d1d`

Observed binary version:
`execsurface 0.1.0-alpha.4`

Observed binary SHA256 in the accepted runner:
`4a850ba66b0484bdf566d5407e12183d85adaf20e8e5e4a2a43c6d8ba84cf853`

## Executable S0-S3 probe

The frozen package wrapper preserved native ExecSurface behavior under the pinned P6 workload:

- `S0_CONTROL` -> exit `0` / PASS
- `S1_CHILD_PROCESS_EXPANSION` -> exit `10` / REVIEW
- `S2_NETWORK_DESTINATION_EXPANSION` -> exit `2` / ERROR because raw observation was incomplete
- `S3_FILE_WRITE_EXPANSION` -> exit `10` / REVIEW

Required vector: **`0 / 10 / 2 / 10`** — satisfied exactly.

Zero S1-S3 false PASS was also enforced.

The harness intentionally injected hostile CI values including a fake baseline path, fake baseline digest and PASS verdict. They did not steer the approved baseline or native verdict/exit code.

## Baseline boundary

The executable probe created a harness-only S0 fixture, moved it into an approved path, froze its SHA256, and then validated the explicit configuration before checks.

This fixture is evidence setup only. It is not a product recommendation to learn and approve a baseline inside the same CI verification job.

Same-job automatic authorization remains forbidden.

## Retained P7 negative evidence

A0 remains:

**P7_A0_ARM64_NOT_PORTABLE**

The exact public alpha.4 observer remains Linux x86_64 only. A2 does not reinterpret arm64 as supported.

A1 historical 13/14 harness-defect run remains retained and separate from its accepted corrected 14/14 run.

## Immutable boundaries preserved

No A2 result authorizes:
- changes to public alpha.4, stable `v0.1`, `main`, public v2 semantics or the default observer;
- authority derived from runner ownership, labels, root/admin status or CI provider;
- baseline auto-learning/approval;
- verdict override by CI metadata;
- incomplete/error evidence laundering;
- arm64 promotion;
- public self-hosted support or zero-assistance deployment claims.

## Scientific conclusion

The tested self-hosted packaging/evidence architecture is internally coherent under the bounded Linux x86_64 research scope: an exact public artifact can be checksum-pinned, a preapproved baseline can be digest-bound, CI context can remain non-authoritative, and native ExecSurface results can pass through without verdict/exit-code laundering.

This is a bounded contract result, not a universal operational compatibility claim.
