# ExecSurface — P6-A0 Initial Overlap Matrix

Date: 2026-09-30
Protocol: `docs/development/P6_COMPETITIVE_FALSIFICATION_PROTOCOL.md`
Status: **FROZEN FOR A0 SOURCE/CAPABILITY VERIFICATION**

## Source pins

| System | Source pin | Role in P6 |
|---|---|---|
| ExecSurface | P5 closeout predecessor `1f4df8f45a115a1638c997107e069c6acd74c51b` | system under test / falsification target |
| Harden-Runner | `e14015d583714f6e62063499dc959a02595150a1` | primary CI comparator |
| cicd-sensor | `1f031a106e23edda1eb496b0bae51fb12e85d62d` | primary CI/runtime-trace comparator |
| Tetragon | `666efe6f91e3605ad58683ad226d759d9cf970ca` | primary runtime evidence comparator |
| Falco | `e12b1d43e47a2903c07e14479e034d74d523ab9d` | secondary overlap-only comparator |
| Tracee | `2f9dc40c20b17c2ba27f6d92b25e62790bd48a62` | secondary overlap-only comparator |

## Applicability freeze

Legend:
- `A0-YES`: source/docs show enough overlap to justify an executable applicability check in A0/A1.
- `A0-PARTIAL`: overlap is narrower or semantics differ; compare only the supported factual slice.
- `A0-CP4`: run-level evidence/attestation overlap specifically justified.
- `A0-SECONDARY`: do not execute until a narrower overlap fixture is defined.
- `NO-ASSUMPTION`: no capability is inferred merely from product category/name.

| System | CP1 process exec | CP2 outbound destination | CP3 file mutation/write | CP4 run evidence/attestation |
|---|---|---|---|---|
| ExecSurface | A0-YES | A0-YES | A0-YES | A0-CP4 |
| Harden-Runner | A0-YES | A0-YES | A0-YES | A0-PARTIAL |
| cicd-sensor | A0-YES | A0-YES | A0-YES | A0-CP4 |
| Tetragon | A0-YES | A0-YES | A0-YES | NO-ASSUMPTION |
| Falco | A0-SECONDARY | A0-SECONDARY | A0-SECONDARY | NO-ASSUMPTION |
| Tracee | A0-SECONDARY | A0-SECONDARY | A0-SECONDARY | NO-ASSUMPTION |

This matrix is an **execution eligibility map**, not a product capability score.

## Source-grounded rationale frozen for A0

### Harden-Runner
Current project documentation describes CI/CD-aware correlation of outbound network connections, file operations, and process execution to step/job/workflow context, plus network baseline/anomaly behavior. A0 must verify exact open-source/action behavior at an immutable executable action pin before any benchmark result is recorded.

### cicd-sensor
Current documentation describes eBPF observation of CI/CD process/network/file activity and production of a runtime-trace attestation predicate on GitHub-hosted runners. It is therefore the strongest initial CP4 interoperability comparator. A0 must separately pin the action commit used for live CI.

### Tetragon
Current documentation describes process lifecycle events plus network and file tracing/observability, including TCP connect examples and file access policies. P6 compares only the overlapping event propositions, not Tetragon's broader runtime-security/enforcement feature set.

### Falco
Falco consumes runtime event streams (commonly syscalls) and evaluates rules over them. P6 includes it only when a direct proposition-specific event fixture can be compared without converting rule-detection semantics into ExecSurface verification semantics.

### Tracee
Tracee is retained as a secondary eBPF event comparator. P6 will not execute it until the exact CP1/CP2/CP3 event schema/filters used by the frozen source are recorded in a follow-up A0 evidence file.

## A0 unresolved items — must be closed before A1

1. Immutable executable action pin for Harden-Runner live CI.
2. Immutable executable action pin for cicd-sensor live CI.
3. Exact Tetragon installation artifact/image pin and policies/events used for CP1–CP3.
4. Whether Harden-Runner exposes a machine-consumable local evidence artifact adequate for CP4; if not, classify CP4 as `NOT_APPLICABLE` or `OBSERVED_PARTIAL` based on evidence, never assumption.
5. Exact cicd-sensor runtime-trace predicate fields relevant to subject/run/workflow binding.
6. Exact runner/kernel compatibility for each primary comparator.
7. Privacy/data retention constraints for captured raw evidence.
8. Narrow Falco/Tracee event-level fixtures, if they remain useful after primary-comparator A1.

## A0 acceptance condition

A0 closes only when:
- all primary executable versions are immutable-pinned;
- every CP1–CP4 cell has an evidence-backed applicability classification;
- unsupported/non-applicable cells remain explicit;
- no benchmark result has been generated before the freeze;
- the machine-readable manifest matches this document.

Candidate decision only after those conditions:

`P6_A0_OVERLAP_MATRIX_FROZEN`