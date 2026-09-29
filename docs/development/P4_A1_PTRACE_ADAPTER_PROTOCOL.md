# ExecSurface — P4-A1 Ptrace Reference Adapter Protocol

Date: 2026-09-29
Parent program: #100
Predecessor: `P4_A0_PROPOSITION_CONTRACT_ACCEPTED_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — IMPLEMENTATION MAY START**

## Objective

Build a research-only proposition adapter over the existing ptrace evidence path while preserving public/raw-v2 semantics and current verdict behavior.

The adapter must make current authority limits explicit rather than repairing them by assertion.

## Fixed team

- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team
- Independent Milestone Reviewer

## Dynamic specialists

- Rust trait/schema engineer
- Linux ptrace specialist
- process/exec causal-lineage specialist
- file/FD lifecycle specialist
- network endpoint identity specialist
- deterministic serialization / reproducibility specialist

## Research-only boundary

Implementation must live under development/research code and must not:
- alter public `Observation` v2 bytes;
- change public normalization/baseline/diff/verdict behavior;
- enable the research certificate in default `observe_command`;
- publish a new crate/version;
- move stable tags/actions;
- claim ptrace completeness beyond tested propositions.

## Adapter record

Each emitted research record must contain:
- proposition ID;
- backend ID = ptrace reference adapter;
- subject identity;
- optional object/effect identity;
- authority state;
- completeness state;
- evidence reference / derivation reference;
- optional causal binding;
- ambiguity/loss reason;
- deterministic record identity.

## A1 implementation stages

### A1.1 — types and deterministic serialization
Implement research-only proposition record types and deterministic ordering/serialization. No collector binding yet.

Required tests:
- enum/state roundtrip;
- deterministic record ordering;
- unsupported/ambiguous/lost preserved;
- no invalid `unsupported + complete` combination;
- no `direct + complete` without evidence reference for propositions requiring evidence.

### A1.2 — ptrace evidence mapping
Map existing evidence to the frozen propositions without changing the collector.

Required minimum:
- process creation;
- successful exec;
- pathname attempt;
- successful-open file object where current evidence supports it;
- covered FD IO;
- rename/delete;
- outbound connect destination;
- causal exec lineage;
- observer health/loss;
- fd-table relation research evidence.

### A1.3 — adversarial adapter gate
Mandatory cases:
- pathname TOCTOU does not become object authority;
- failed open does not become successful-open proposition;
- failed exec does not become successful exec;
- unknown shared-FD relation makes dependent FD proposition incomplete;
- resource loss downgrades affected records;
- causal-chain substitution fails exact causal record;
- wrong actor does not inherit effect authority;
- unsupported proposition/no event cannot become negative proof;
- serialization deterministic across input order where semantics are identical.

### A1.4 — public anti-drift gate
Run current public regression suites, including M11 shared-FD fail-closed tests. No public behavior change is acceptable.

## Allowed A1 outcomes

- `P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY`
- `P4_A1_FALSE_AUTHORITY_PATH_FOUND`
- `P4_A1_PUBLIC_CONTRACT_REGRESSION`
- `P4_A1_INCOMPLETE_EVIDENCE`

Only PASS_RESEARCH_ONLY authorizes A2 authority-gap matrix work. It does not authorize a new backend.
