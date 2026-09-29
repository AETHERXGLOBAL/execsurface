# ExecSurface — P3 Variance / Nondeterminism Threat Model

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **V0 CLOSED — THREAT MODEL FROZEN FOR RESEARCH IMPLEMENTATION**

## Objective

Define what P3 is allowed to treat as legitimate variance before building any stabilization mechanism.

The governing rule is:

> Variability evidence can explain recurrence; it cannot silently authorize behavior.

P3 therefore separates identity normalization, effect-set variability, authorization and observer completeness.

## 1. Variance classes

### V-A — Ephemeral identity variance

Different serialized identity for the same narrowly defined semantic producer role.

Example already proven by #58:
- `$TMP/go-build<digits>/...`

A valid normalization requires a bounded grammar, semantic-root restriction and collision tests. It is a canonicalization problem, not a multi-run acceptance problem.

### V-B — Genuine effect presence/absence variance

Real semantic effects differ across equivalent trusted runs.

Examples from #62:
- different Go build-cache reads;
- different module/source reads;
- different standard-library source reads.

These paths are not disposable merely because they vary. They remain real effects and require an explicit variance contract if accepted.

### V-C — Ordering/interleaving-only variance

The same semantic effect set appears under different scheduling/interleaving order.

If canonical set semantics already remove order, no new variance permission is needed. If ordering is semantically relevant for a future proposition, it must remain explicit rather than normalized away.

### V-D — Environment/platform variance

Differences caused by platform, architecture, observer capability, normalization profile, workspace roots or declared environment identity.

These are comparability questions, not legitimate within-contract variance unless an explicit cross-environment contract is separately proved.

### V-E — Observer incompleteness

Loss, truncation, lifecycle gaps, unsupported capabilities, unresolved identity and similar evidence-health failures are **not variance**.

No multi-run model may convert incomplete evidence into accepted variability.

### V-F — Behavior expansion/contraction

An effect that appears/disappears relative to an accepted contract and is not already covered by explicitly accepted variance remains drift.

### V-G — Learning-time poisoning

A malicious or accidental effect appears during one or more learning runs with the goal/effect of entering the accepted contract.

Frequency alone cannot distinguish legitimate recurrence from repeated malicious behavior. Therefore frequency is never authorization.

## 2. Separation of concerns

P3 freezes four distinct layers:

1. **Canonicalization** — decides whether two identities are the same semantic identity under a versioned rule.
2. **Observed recurrence analysis** — measures intersection, union and support/provenance across trusted runs.
3. **Acceptance** — explicit human/tooling decision that selects which variable effects are part of an accepted contract.
4. **Policy/verdict** — remains separate and decides what to do with drift or proof changes.

No layer may silently absorb another.

## 3. Safe multi-run semantics hypothesis

Given an explicitly supplied set of comparable, verified trusted learning runs `R`:

- `core(R)` = effects present in every distinct trusted run;
- `union(R)` = effects present in at least one distinct trusted run;
- `variable_candidates(R)` = `union(R) - core(R)`;
- `support(e)` = number of distinct trusted runs containing effect `e`;
- `provenance(e)` = exact distinct source baseline/run digests containing effect `e`.

These are descriptive facts only.

They do not create an accepted variance contract.

## 4. Authorization rule

An effect becomes accepted variable behavior only through a separately explicit acceptance operation/record.

Forbidden automatic rules include:
- `seen once => accepted`;
- `seen k/N => accepted`;
- `majority => accepted`;
- `union => accepted`;
- `frequent => safe`.

Frequency/support may be displayed as decision support but has no authorization semantics by itself.

## 5. Learning-run admissibility

A run may enter recurrence analysis only if:
- baseline/evidence digest verifies;
- canonical schema/profile is supported;
- command identity is comparable;
- platform identity is comparable;
- observer identity/capability contract is comparable for the research question;
- observation was complete for all effect families being analyzed;
- source run/artifact identity is unique in the learning set.

Duplicate artifacts do not increase support counts.

An incomplete run is rejected rather than treated as a run in which missing effects were absent.

## 6. Provenance requirement

Every candidate variable effect must retain exact source provenance to the distinct learning artifacts in which it appeared.

The analyzer must make it possible to answer:
- which runs contained this effect?
- how many distinct runs contained it?
- was it part of the invariant core?
- did a duplicate artifact attempt to inflate support?

A future accepted-variance record must retain a deterministic learning-set digest plus per-effect provenance or cryptographic references sufficient to audit acceptance.

## 7. Actor/causal scope

Variance acceptance cannot be path-only when the baseline semantics distinguish actor or causal chain.

An effect observed from actor A does not automatically authorize the same path/effect from actor B.

Any future variance key must use the same canonical semantic effect identity (including actor/chain fields where present) as the baseline/diff contract unless an explicit projection is separately proved.

## 8. Ephemeral grammar threat model

A producer-specific normalization rule must survive:
- attacker-controlled filename matching the grammar outside the permitted semantic root;
- same prefix with semantically meaningful suffix difference;
- non-digit/non-conforming variants;
- neighboring producer names;
- path traversal/root confusion;
- collision between two genuinely different semantic roles.

If a grammar cannot avoid unsafe collision, it is rejected even if it removes many findings.

## 9. Check-time rule

A future variance-aware check may classify an observed effect as:
- invariant accepted effect;
- explicitly accepted variable effect;
- unseen/unaccepted effect;
- missing invariant effect;
- proof/completeness incompatibility;
- schema/comparability error.

An effect that merely appeared during research learning but was never explicitly accepted remains unseen/unaccepted for authorization purposes.

## 10. Red-team invariants

P3 implementation is invalid if any of the following becomes possible:
- one poisoned run silently expands accepted behavior;
- repeated poisoned runs automatically gain permission;
- duplicate artifacts inflate recurrence support;
- incomplete runs look like legitimate absence;
- broad temp/cache normalization hides a meaningful new path;
- path-only matching ignores actor/causal differences;
- unseen behavior passes due to similarity/frequency;
- check mutates the accepted variance contract;
- changing thresholds after seeing results converts REVIEW to PASS.

## V0 decision

**`P3_V0_THREAT_MODEL_ACCEPTED`**

P3 may proceed to V1 with a research-only analyzer that computes recurrence/provenance facts and has **no authorization or verdict output**.
