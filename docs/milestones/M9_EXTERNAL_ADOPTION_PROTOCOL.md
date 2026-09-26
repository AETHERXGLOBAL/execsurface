# M9 — External Adoption Progression Protocol

Date: 2026-09-26
Status: **ALIGNED / SUPPLEMENTARY — CANONICAL M9 EVIDENCE RULES RETAINED**
Tracking: #51
Base: `8f1ec63219e084f10678655ee698fd2a482312d6`
Canonical protocol: `docs/milestones/M9_PROTOCOL.md`
Machine schema: `docs/milestones/M9_EVIDENCE_SCHEMA.json`
Validator: `scripts/m9_validate_evidence.py`

## Authority

This document supplements the canonical M9 protocol with external-adoption progression levels and workflow guidance.

If any historical wording in this file conflicts with `M9_PROTOCOL.md`, `M9_EVIDENCE_SCHEMA.json`, or the semantic validator, the stricter canonical M9 rule controls. In particular, the accepted performance plan is **exactly 3 warmups and exactly 15 measured samples per mode**, not the earlier 2/11 draft.

No threshold, sample count, independence rule, privacy rule, or eBPF reopen condition may be changed after observing target results without an explicit pre-result protocol amendment.

## Objective

M9 moves ExecSurface from AETHER X-controlled compatibility evidence toward independently attributable product evidence while preserving strict claim boundaries.

The program asks two separate questions:

1. Does the current ptrace public product produce useful, reproducible runtime-drift evidence on real external developer/CI workloads?
2. Does any real workload provide an evidence-backed reason to reopen the deferred eBPF path?

A negative result is valid evidence. M9 must not manufacture adoption or an eBPF need.

## Governance

### Innovation Scientist / Product Architect

- choose high-information targets rather than high-volume outreach;
- convert real external friction into minimal evidence-backed product improvements;
- search for workflows where execution-surface drift is genuinely useful;
- separate product value from backend novelty.

### Deviation Prevention / Scientific Integrity

- freeze claim levels and benchmark rules before target results;
- prevent AETHER-controlled compatibility from being called adoption;
- preserve negative results, nondeterminism and false positives;
- reject post-hoc sample removal or target switching to improve metrics;
- keep the M6.5 eBPF trigger unchanged.

### Independent Red Team

Challenge:

- independence/provenance of the evidence source;
- reproducibility from pinned target revisions;
- hidden setup/manual intervention;
- noisy baselines and nondeterministic findings;
- false-positive / false-negative risk;
- privacy leakage;
- performance population and host claims;
- user-facing friction;
- whether an eBPF reopening trigger is genuinely met.

## External evidence levels

These levels are descriptive overlays. Every machine-readable record must also carry exactly one canonical `independence_class` from the M9 schema.

### L0 — AETHER-controlled external compatibility

AETHER X runs ExecSurface against a pinned, unmodified external project/workload without maintainer coordination.

Canonical class: `ZERO_CONTACT_EXTERNAL_REPRO`.

Permitted claim: bounded external compatibility/workload evidence.

Forbidden claims: independent validation, external adoption, maintainer endorsement.

### L1 — Independent external reproduction

A third party independently executes the prescribed workflow and returns auditable evidence.

Canonical class is normally `INDEPENDENT_USER` if AETHER X did not materially control execution; otherwise use `COLLABORATIVE_EXTERNAL`.

Permitted claim: independent external reproduction only when the schema provenance requirements are met.

### L2 — Adoption trial

An independent external project/user intentionally installs or integrates ExecSurface in a branch, PR, CI experiment, or sustained evaluation.

This remains scoped to the named project/workflow and evidence date.

### L3 — Retained adoption

The external project/user merges, retains, or repeatedly uses ExecSurface after the initial trial.

Permitted claim: retained adoption only for the evidenced project/workflow and period.

### Promotion rule

A target advances only when the required external action actually occurs. Interest, messages, GitHub stars, downloads, issue comments, AETHER-authored tests, or AETHER-authored PRs do not by themselves promote a target to L1/L2/L3.

## Target-selection rules

A candidate should satisfy as many as possible:

1. public repository;
2. reproducible Linux workflow from a pinned commit;
3. no secret-dependent setup for the selected command;
4. deterministic or explainably variable execution behavior;
5. meaningful process/file/runtime activity;
6. command executable non-interactively in CI;
7. direct median preferably >=100 ms if used for the relative M6.5 trigger;
8. practical runtime for the canonical 3-warmup + 15-measured-per-mode protocol;
9. no target-source change required for the clean baseline experiment;
10. licensing and repository policy do not block local compatibility testing.

A target may be KILLED before execution for irreproducible setup, secret dependency, excessive runtime, unsafe side effects, or meaningless runtime behavior. The rejection remains preserved.

## Target pinning

Every accepted experiment records:

- repository owner/name;
- exact commit SHA;
- selected command;
- host image/OS/kernel/architecture/CPU facts;
- toolchain/runtime versions materially affecting the command;
- ExecSurface version/commit;
- setup commands required before the measured/observed command.

Never use a moving branch as the evidence identity.

## Reproduction gate

Before accepted ExecSurface observation:

1. checkout the pinned target commit;
2. install only declared prerequisites;
3. execute the selected command directly;
4. require successful, explainable output/exit status or preserve the failure;
5. record direct runtime and expected nondeterminism;
6. record any workaround attempt separately before applying it.

A target that needs material source modification before direct reproduction is not accepted under its original candidate definition.

## Fresh-install gate

Where practical, install the public product without using an ExecSurface repository checkout:

```bash
cargo install execsurface --locked
```

If the registry path cannot be used for a specific host/toolchain reason, record the exact alternative and preserve the lower/qualified evidence scope.

## L0 runtime workflow

### W1 — Learn

Run the selected pinned workload through public ptrace-backed `execsurface learn` and record observation health/completeness, baseline digest, canonical effect count, elapsed time and warnings.

### W2 — Repeat no-drift

Run the identical command with `execsurface check` against the learned baseline. PASS is accepted only if normal product semantics justify it. Repeated nondeterministic drift is negative evidence to preserve, not a reason to weaken normalization.

### W3 — Controlled runtime expansion

Where practical, add a non-destructive wrapper-level runtime expansion without modifying target source. The expansion may test detection but must never be represented as target-project behavior.

### W4 — Repeatability

Repeat clean and expansion checks enough to distinguish deterministic product behavior from one-run scheduling noise. Functional repeat counts must be declared in the workload execution plan before accepted results.

## Canonical performance protocol

Performance evidence is separate from functional compatibility.

For every accepted performance case:

- exactly 3 warmups per mode;
- exactly 15 measured direct samples;
- exactly 15 measured ptrace-observed samples;
- same pinned source, setup, command and host class;
- paired alternating execution order as defined in `M9_PROTOCOL.md`;
- raw samples retained in execution order;
- no post-hoc sample deletion based on magnitude;
- infrastructure exclusions remain machine-readable with reasons.

Report at minimum median direct runtime, median ptrace runtime, median absolute overhead and slowdown ratio. Additional robust statistics may be reported but cannot replace the frozen trigger calculation.

### M6.5 eBPF evaluation trigger

A targeted eBPF evaluation may be reopened only if an accepted external workload reproduces either:

1. `direct median >= 100 ms` and `ptrace median / direct median > 2.0`; or
2. `ptrace median - direct median > 500 ms`.

Crossing the trigger means **REOPEN EVALUATION**, not eBPF approval or public authority.

## Privacy boundary

Evidence must not persist file contents, environment values, stdin contents, network payloads, secret values, or unrestricted argv values. Machine-readable records must satisfy the canonical metadata-only privacy fields.

## False-positive / false-negative ledger

Every target must preserve:

- unexplained drift;
- expected effects missing from evidence;
- machine-specific path noise;
- scheduling/interleaving noise;
- transient toolchain/cache effects;
- unsupported semantics;
- usability/setup friction;
- whether the issue appears target-specific or product-general;
- evidence status before and after any fix.

A fix cannot erase the pre-fix counterexample.

## External-contact boundary

M9.0 and M9.1 are zero-contact evidence phases. No maintainer contact is required for local public-repository reproduction.

Before any external issue/PR/outreach:

- the L0 pack must be reproducible enough to share narrowly;
- no private AETHER X material is disclosed;
- no unsupported performance/security claim is made;
- the external action is recorded in the M9 ledger;
- resulting evidence is classified by actual provenance, not desired outcome.

## Phase mapping

The canonical roadmap controls milestone names:

1. **M9.0 — Protocol & intake freeze:** CLOSED / PROVED.
2. **M9.1a — Target qualification:** pin cohort before execution results.
3. **M9.1b — Zero-contact external execution:** collect L0 / `ZERO_CONTACT_EXTERNAL_REPRO` compatibility and performance evidence.
4. **M9.2 — Independent adoption evidence:** accept L1/L2/L3 only if third-party provenance actually satisfies the schema.
5. **M9.3 — Evidence synthesis / product decision:** bounded product conclusions and any justified reopen decision.

Evidence-backed product hardening may occur only under a separately recorded fix gate that preserves the pre-fix failure and does not rewrite M9 history.
