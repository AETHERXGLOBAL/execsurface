# P4 — Backend Adapter / Proposition-Authority Architecture

Parent: #100
Predecessor: #103 `P3_EPHEMERAL_ONLY_VALUE_ESTABLISHED`
Branch: `development/post-alpha4-behavioral-integrity`
Public `v0.1.0-alpha.4` remains unchanged.

## Status

A0 CLOSED — `P4_A0_PROPOSITION_CONTRACT_ACCEPTED_BOUNDED`

A1 OPEN — ptrace reference adapter, research only.

## Objective

Separate ExecSurface semantic authority from collector implementation using proposition-scoped proof records. No backend is globally equivalent to another backend; evidence is compared only under explicit proposition/proof requirements.

## Canonical artifacts

- `docs/development/P4_BACKEND_AUTHORITY_PROTOCOL.md`
- `docs/development/P4_A0_PROPOSITION_INVENTORY.md`
- `docs/development/P4_A0_REVIEW_MATRIX.md`
- `docs/development/P4_A0_DECISION_PROTOCOL.md`
- `docs/development/P4_A0_REVIEW_RESULT.md`
- `docs/development/P4_A1_PTRACE_ADAPTER_PROTOCOL.md`
- canonical experiment: `experiments/p4-backend-authority`
- canonical CI: `.github/workflows/p4-a0-a1-authority.yml`

## Fixed roles

1. Innovation Scientist / Systems Architect
2. Anti-Drift / Scientific Integrity Reviewer
3. Independent Falsifier / Red Team
4. Independent Milestone Reviewer

## Dynamic specialist pool

PL/evidence semantics, Linux ptrace, kernel/BPF-LSM, process/causal identity, file/FD lifecycle, networking identity, Rust schema/API design, reproducibility/CI, provenance/evidence sealing.

## Boundaries

- no public raw-v2 or verdict change;
- no global backend equivalence;
- unsupported/ambiguous/lost is never absence-of-behavior;
- no full-host daemon for parity;
- no new eBPF authority without an A2 proposition gap;
- no external import before adapter contract is stable;
- all failures retained.

## Next

Repair/re-run the canonical A0/A1 authority-model falsification workflow after its retained formatting-only failure, then close A1 only if the canonical experiment, Semantics-v3 reproof and public M11 fail-closed regression all pass. A2 may start only after A1 PASS_RESEARCH_ONLY.
