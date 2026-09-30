# ExecSurface — P8-A0 External Validation Pack Decision

Date: 2026-09-30
Parent program: #100
P8 issue: #114
External engagement issue: #94
Protocol: `docs/development/P8_A0_EXTERNAL_EVIDENCE_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**P8_A0_EXTERNAL_VALIDATION_PACK_READY_BOUNDED**

The evaluator packet, evidence qualification rules, assistance taxonomy, intake template, public artifact identity, and negative-evidence handling are frozen and internally reverified.

This decision is **not external validation**. It establishes only that the external-validation input path is explicit, reproducible, and protected against self-validation / endorsement inflation.

## Accepted A0 evidence

- workflow source: `7ad1e6c139c4787f84e54261744a189e1835722a`
- workflow: `.github/workflows/p8-a0-external-pack.yml`
- run: `36777065464`
- conclusion: `success`
- artifact ID: `11125794344`
- artifact name: `p8-a0-36777065464-1`
- artifact digest: `sha256:7d4021b4d93760f594a4faf26ad8ace2b1cdc2874f288fcd6e753d781483bce5`
- artifact size: `1727` bytes

Frozen A0 document blobs:
- protocol: `c00dc7c16feab48e1791d170be0ad9369d21f161`
- evaluator packet: `db460b76fda75e67d18b0a287563dbe6fe50119a`
- intake template: `fcecc44d53e1d0dc67a95dc0a8ed88cc138e8806`

## Public target reproof

The gate reverified:
- release/tag `v0.1.0-alpha.4` source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable `v0.1` at the same source;
- public main freeze `b47721ce675444ce029b048edce6fc94631d6408`;
- public Linux x86_64 release artifact download;
- expected release SHA256 `55776cd130784d1c03451a14ab05ed113ffae4fa714c986666cbcadf242d8d1d`;
- binary reports `execsurface 0.1.0-alpha.4`;
- P7 closeout remains ancestral;
- no P8-A0 change touches public `crates/`.

## Qualification boundary

A0 freezes a distinction between:

Qualifying evidence:
- zero-assistance reproduction;
- reproduction failure;
- architecture criticism;
- counterexample;
- no-fit/redundancy;
- interoperability guidance;
- external real-workload report;
- external contribution.

Non-qualifying by itself:
- routing guidance;
- community participation;
- acknowledgement/praise;
- internal AETHER X rehearsal;
- organizational membership;
- unactionable anonymous opinion.

An evaluator's title, employer or prestige does not raise evidence quality.

## Historical evidence classification

Retained historical external criticism:
- `EXT-0001` — Greg Kroah-Hartman ptrace/LSM architecture criticism — accepted and already acted upon; retained as prior external technical evidence, but not used as a substitute for current-state P8 review.

Current routing/participation evidence:
- Linux Foundation/OpenSSF response — `ROUTING_GUIDANCE`;
- Rust Foundation response directing project review to the Rust users Code Review category — `ROUTING_GUIDANCE`;
- ORBIT Slack entry — `COMMUNITY_PARTICIPATION`;
- AETHER X clean-environment rehearsal — pack-integrity evidence only, `DISQUALIFIED_SELF_EVIDENCE` for external independence.

## External dependency after A0

P8-A1 and later gates require evidence produced by an external participant. AETHER X cannot self-certify that dependency.

A failed, partial, unsupported or no-fit external result is acceptable and must be retained. Assisted follow-up must be recorded separately and cannot rewrite the initial result.

## Claims boundary

A0 authorizes none of the following claims:
- independent reproduction achieved;
- external validation achieved;
- OpenSSF/Rust Foundation endorsement;
- adoption;
- standards-body approval;
- architecture correctness beyond the bounded internal evidence.

Current exact state:

`P8_A0_PACK_READY / QUALIFYING_EXTERNAL_RESPONSE_PENDING`
