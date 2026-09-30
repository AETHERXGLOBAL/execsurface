# ExecSurface — P6 Competitive Falsification Protocol

Date: 2026-09-30
Parent program: #100
Predecessor: `P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO COMPETITIVE VERDICT YET**

## 1. Objective

P6 tests factual overlap between ExecSurface and selected runtime/CI observability systems under pinned, reproducible scenarios. It does **not** rank products and does not produce a composite winner score.

The research question is:

> For the same declared runtime proposition and workload, what evidence does each system actually expose, with what identity, authority/completeness, reproducibility, operational friction, data capture, and attestation/provenance properties?

The target is falsification of ExecSurface claims and architecture boundaries, not marketing comparison.

## 2. Fixed roles

1. **Innovation Scientist / Systems Architect** — identify non-obvious, high-leverage comparison propositions and avoid superficial feature parity.
2. **Anti-Drift / Scientific Integrity Reviewer** — block claim inflation, score aggregation, threshold changes after results, and comparisons outside demonstrated overlap.
3. **Independent Falsifier / Red Team** — construct false-PASS, false-REVIEW, loss, replay, identity substitution, and incomplete-observer scenarios.
4. **Independent Critical-Milestone Reviewer** — verify exact pins, workload identity, evidence retention, negative results, and decision wording.

Dynamic specialists for P6:
- CI/CD runtime security architect;
- Linux/eBPF runtime observer engineer;
- runtime evidence semantics / PL specialist;
- benchmarking and reproducibility engineer;
- privacy/data-minimization reviewer;
- in-toto/SLSA/Sigstore interoperability specialist.

## 3. Immutable ExecSurface boundaries

P6 MUST NOT:
- modify public `v0.1.0-alpha.4`, stable `@v0.1`, public v2 semantics, or default public observer;
- turn ExecSurface into an EDR, threat-intelligence, malware-signature, or broad detection product;
- use backend/product name as semantic authority;
- treat unsupported, ambiguous, lost, or incomplete as absence or PASS;
- infer full equivalence from one overlapping event class;
- silently authorize variance or mutate accepted baselines;
- delete negative evidence;
- change scenario definitions or acceptance criteria after observing results;
- report a composite score, tier, or overall winner.

## 4. Frozen external source pins

These pins freeze the comparison source state. Product releases/actions used in executable CI may be pinned separately to immutable release SHAs, but may not float during a gate.

| System | Repository | Frozen source |
|---|---|---|
| Harden-Runner | `step-security/harden-runner` | `e14015d583714f6e62063499dc959a02595150a1` |
| cicd-sensor | `cicd-sensor/cicd-sensor` | `1f031a106e23edda1eb496b0bae51fb12e85d62d` |
| Tetragon | `cilium/tetragon` | `666efe6f91e3605ad58683ad226d759d9cf970ca` |
| Falco | `falcosecurity/falco` | `e12b1d43e47a2903c07e14479e034d74d523ab9d` |
| Tracee | `aquasecurity/tracee` | `2f9dc40c20b17c2ba27f6d92b25e62790bd48a62` |

Falco and Tracee are secondary comparators and are included only where a proposition is directly overlapping. No penalty is assigned for non-overlap.

## 5. Primary overlapping propositions

P6-A0 freezes the first comparison surface to propositions that are evidenced by at least two systems.

### CP1 — Process execution identity
Question: can the system report that a specific process execution occurred and bind it to a usable actor/workload identity?

Required observations:
- executable/process identity;
- parent/lineage or CI-step attribution when available;
- success/attempt semantics distinguished if the product exposes them;
- loss/incompleteness behavior documented.

### CP2 — Outbound network destination
Question: can the system report a process/workload outbound connection destination and bind it to the originating actor/workload?

Required observations:
- destination identity;
- actor/workload binding;
- attempt vs established/success distinction when exposed;
- unsupported/lost/incomplete handling.

### CP3 — File mutation / write observation
Question: can the system report a file mutation/write and bind it to the responsible process/workload?

Required observations:
- path/object identity actually exposed;
- actor/workload binding;
- operation class;
- success/attempt distinction when exposed;
- ambiguity/loss handling.

### CP4 — Runtime evidence/attestation export
Question: can the system emit a machine-consumable run-level evidence artifact or attestation that can be deterministically bound to the run/workflow/artifact context?

Required observations:
- artifact/predicate format;
- subject/run identity;
- provenance/workflow context;
- evidence integrity/digest where exposed;
- whether semantic authority is explicit or only implied.

CP4 is expected to overlap most strongly with cicd-sensor and ExecSurface. Systems without a comparable run-level attestation are marked `NOT_APPLICABLE_TO_CP4`, not failed.

## 6. Frozen scenario family

The initial executable corpus will use deterministic Linux CI workloads on pinned Ubuntu runner images where the compared system supports them.

### S0 — Control
- declared command only;
- no adversarial drift;
- expected result: observation without synthetic anomaly.

### S1 — Child process expansion
- control command spawns one deterministic undeclared child executable;
- tests process identity and lineage/step attribution.

### S2 — Network destination expansion
- control command performs one deterministic outbound TCP connection to a preregistered test destination;
- a second variant substitutes the destination;
- no production secret/token is used.

### S3 — File write expansion
- control command writes one deterministic file under a sandbox directory;
- a second variant changes the destination path;
- no sensitive host paths are touched.

### S4 — Evidence identity substitution
- reuse/copy an evidence artifact under a different expected workflow/run/source identity;
- tests replay/substitution rejection or explicit inability to verify it.

### S5 — Observer degradation/loss
- only when the product exposes a supported, safe method to induce/report observer degradation;
- inability to induce loss is `NOT_TESTABLE`, not a PASS or FAIL.

No scenario may be altered after its first scientifically executable run except to repair a demonstrated fixture/infrastructure defect; such a repair must retain the failed run and document why the scientific hypothesis did not change.

## 7. Measurement dimensions — separate only

Each system/scenario records independent facts. No weighted sum is allowed.

1. setup/privilege requirements;
2. supported runner/environment scope;
3. proposition observation coverage;
4. actor/workflow attribution;
5. success/attempt semantics;
6. explicit completeness/loss state;
7. deterministic reproducibility;
8. false PASS under S1–S5 where applicable;
9. false REVIEW / false incompleteness on S0;
10. runtime overhead under a separately preregistered timing gate;
11. data captured / privacy surface;
12. CI integration friction;
13. attestation/provenance output;
14. failure transparency and retained evidence.

Performance is forbidden in A0/A1 unless timing methodology has been separately frozen. Correctness/coverage gates precede performance claims.

## 8. Classification vocabulary

Per proposition/scenario only:
- `OBSERVED_SUPPORTED`
- `OBSERVED_PARTIAL`
- `NOT_OBSERVED`
- `NOT_APPLICABLE`
- `NOT_TESTABLE`
- `AMBIGUOUS`
- `INCOMPLETE`
- `LOST`
- `FIXTURE_OR_INFRA_FAILURE`

For ExecSurface false-verdict tests:
- `FALSE_PASS_COUNTEREXAMPLE`
- `FALSE_REVIEW_COUNTEREXAMPLE`
- `NO_COUNTEREXAMPLE_IN_FROZEN_SCOPE`

A product is never globally labelled stronger/weaker/better/worse from these classes.

## 9. Gate sequence

### P6-A0 — Source/capability inventory and exact overlap freeze
Deliverables:
- pinned-source manifest;
- proposition-to-system applicability matrix;
- exact executable release/action pins;
- data-access/privacy notes;
- no live benchmark verdict.

Closure candidate:
`P6_A0_OVERLAP_MATRIX_FROZEN`

### P6-A1 — Deterministic same-workload observation corpus
Run S0–S3 against primary comparators where supported. Preserve raw outputs and normalized factual extracts separately.

Closure candidates:
- `P6_A1_OBSERVATION_CORPUS_COMPLETE_BOUNDED`
- `P6_A1_INCOMPLETE`

### P6-A2 — Adversarial false-PASS / identity-binding corpus
Run S1–S5 and CP4 substitution/replay where supported. ExecSurface is explicitly a target of the falsifier.

Any surviving ExecSurface false PASS blocks positive P6 closeout until corrected or bounded out with evidence.

### P6-A3 — Reproducibility / privacy / CI-friction matrix
Repeat accepted scenarios across independent runs; record setup steps, privilege requirements, artifacts, data capture, and unsupported cases.

### P6-A4 — Performance gate (optional, separately preregistered)
Only if A1–A3 produce comparable correctness evidence and the runtime environments are sufficiently aligned. Otherwise record `NO_VALID_PERFORMANCE_COMPARISON`.

### P6 closeout
Allowed bounded decisions:
- `P6_FACTUAL_COMPARISON_COMPLETE_BOUNDED`
- `P6_INCOMPLETE_FOR_COMPARATIVE_CLAIMS`
- `P6_EXECSURFACE_COUNTEREXAMPLE_FOUND`

No closeout can state an overall winner.

## 10. Evidence retention

Every scientifically executable run must retain:
- exact ExecSurface SHA;
- exact external action/image/release pins;
- runner image / kernel identity;
- scenario manifest and workload digest;
- raw tool output where terms/privacy permit;
- normalized factual extraction;
- exit status and observer-health signals;
- workflow run ID;
- artifact ID, digest and size when available;
- decision file after gate completion.

Historical failures remain in the repository/Actions record and may not be rewritten.

## 11. First next action

Execute **P6-A0 only**: verify each external source/release pin, freeze proposition applicability, and create the machine-readable comparison manifest. Do not run live competitive workloads until A0 is closed.