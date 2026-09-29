# ExecSurface — P4-A1.3 Ptrace Adapter Adversarial Result

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Protocol: `docs/development/P4_A1_3_ADVERSARIAL_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

`P4_A1_3_FALSIFICATION_PASS_BOUNDED`

This is a research-only, proposition-scoped result. It does not authorize public integration, a new backend, backend equivalence, or release promotion.

## Accepted source and CI evidence

Accepted source:
`3566f09c0963cf8065ac65fc34c447084c2c7b2c`

Workflow:
`36585353341`

Job:
`109464068693`

Evidence artifact:
`11040679788`

Artifact SHA-256:
`sha256:a3ead10a7adf43a074268b190d23d912a029e7a5b63b9e9f5052e5dc6f55e259`

Environment:
- GitHub-hosted Ubuntu 24.04;
- Rust 1.90.0;
- canonical research authority model: `experiments/p4-backend-authority`;
- public/raw-v2 collector and public verdict path unchanged.

## Retained pre-test failure

The first A1.3 execution attempt is retained:
- workflow `36585042885`;
- job `109462989610`;
- source `4c76832f4545cfb467fd2f1d3202b74a2ea6b003`;
- artifact `11040484586`;
- artifact SHA-256 `sha256:d0349425a22121af415f7bf904b5918ad725726cd862076affd0881b983c24b2`;
- classification: `FORMATTING_ONLY / PRE_TEST`.

No adversarial test executed in that failed attempt. The only correction was Rust 1.90 formatting of the already-frozen adversarial test source. No assertion, authority rule, capability gap, acceptance condition, or expected outcome was changed.

## Accepted gates

The accepted workflow passed:
- rustfmt PASS;
- clippy `-D warnings` PASS;
- canonical authority-model tests **8/8 PASS**;
- A1.3 integration binary **18/18 PASS**, comprising the eight frozen A1.3 adversarial cases plus the ten existing A1.2 mapping cases;
- separate A1.2 mapping binary **10/10 PASS**;
- Semantics v3 reproof **7/7 PASS**;
- public M11 shared-FD contract **6/6 PASS**.

## A1.3 frozen adversarial cases

All eight newly frozen attacks passed without weakening the mapping:

1. PATH-TOCTOU/object laundering did not convert pathname-attempt evidence into `P4.FILE.OPEN_OBJECT` authority.
2. Missing successful-open object evidence remained an explicit capability gap and did not become negative proof.
3. Absence of a successful raw-v2 exec event did not create `P4.EXEC.SUCCESS`.
4. Raw-v2 clone evidence without retained flags kept exact fd-table relation ambiguous/incomplete and blocked dependent FD authority.
5. Incomplete/warning-bearing observation stopped effect mapping and emitted observer loss evidence instead.
6. Causal-chain substitution changed the causal proposition identity even when the final executable/target matched.
7. Wrong-actor substitution changed the FD proposition identity for the same target.
8. Unsupported/no-event state did not become a supported `direct + complete` negative proposition.

## Scientific interpretation

Within the frozen raw-v2 mapping scope, the falsifier did not find a declared path that launders weak, missing, ambiguous, or lost evidence into false `direct + complete` authority.

This result is bounded. It does not establish universal ptrace completeness, successful-object authority for raw-v2 pathname evidence, exact clone fd-table relation from raw-v2 bytes, successful rename/connect semantics, or global backend equivalence.

## Promotion boundary

A1 remains open until A1.4 public anti-drift verification passes. Only then may A1 close as:

`P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY`

A2 authority-gap matrix work is not authorized before that closure.
