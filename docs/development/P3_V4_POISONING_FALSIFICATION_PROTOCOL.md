# ExecSurface — P3 V4 Poisoning / Consolidated Falsification Protocol

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **V4 PREREGISTERED — NO V5 AUTHORIZED YET**

## Objective

Attempt to break the combined V1 multi-run analyzer, V2 bounded GCC ephemeral classifier and V3 explicit accepted-variance contract before any real-workload value requalification.

V4 is a falsification gate, not a feature gate. A discovered authorization leak is retained as a negative result and blocks V5 until classified and separately resolved.

## Team

Fixed roles:
- Innovation Scientist / Systems Architect — search for structural fixes rather than whitelist growth;
- Anti-Drift / Scientific Integrity Reviewer — prohibit test weakening, after-the-fact exceptions and frequency-based permission;
- Independent Falsifier / Red Team — owns attack construction and final classification.

Dynamic specialists:
- CI/supply-chain attack modeling;
- build-system/compiler behavior;
- provenance/attestation;
- deterministic serialization/set analysis;
- canonicalization/path identity;
- process actor/causal-lineage semantics;
- observer completeness and platform/profile compatibility.

## Immutable safety properties

The consolidated stack must preserve all of:

1. **Observation is not authorization.** Union membership and recurrence never imply acceptance.
2. **Frequency is not authorization.** Repetition across learning runs never creates acceptance by itself.
3. **Incomplete evidence is not variance evidence.** An incomplete run must not enter the trusted learning set.
4. **Duplicate evidence cannot vote twice.** Identical evidence digest cannot inflate support.
5. **Profile mismatch is non-comparable.** Platform/observer/schema/normalization mismatch must not enter one learning set.
6. **Ephemeral classification is bounded.** Filename similarity alone must not collapse identity.
7. **Explicit acceptance is exact.** A different actor, causal chain or canonical effect remains a different effect.
8. **Unseen similar behavior remains unseen.** Similarity to an accepted effect does not grant acceptance.
9. **Public alpha.4 remains unchanged.** No research result modifies v2 baseline/check semantics.

## Preregistered attack suite

### A1 — single-run malicious injection
Inject a malicious canonical effect into exactly one of N otherwise trusted learning runs. Required result: variable candidate only; never automatically accepted.

### A2 — repeated malicious injection
Inject the same malicious effect into multiple but not all learning runs. Required result: support count may rise, acceptance must remain unchanged without explicit selection.

### A3 — duplicate-vote inflation
Repeat the same evidence artifact/digest. Required result: analyzer rejects the learning set.

### A4 — incomplete-run poisoning
Mark an injected run incomplete. Required result: analyzer rejects it rather than interpreting missing/present effects as variance.

### A5 — run-order manipulation
Permute the same trusted learning artifacts. Required result: byte-identical V1 report and byte-identical V3 contract for equivalent explicit selection.

### A6 — profile/schema/observer mismatch
Change platform, observer or normalization/schema identity in one run. Required result: learning set rejected as incomparable.

### A7 — GCC grammar mimicry outside bounded role
Use `cc[A-Za-z0-9]{6}.s` with wrong root, nesting, actor, missing create/delete role, incompatible operation or conflicting actor. Required result: not classified eligible.

### A8 — accepted-effect actor substitution
Start from an explicitly accepted variable effect and alter actor identity and/or causal execution chain while preserving a similar target path. Required result: exact accepted-effect lookup must fail.

### A9 — unseen similar target
Create an unseen effect with a path lexically similar to an accepted candidate but not exact canonical effect identity. Required result: not accepted.

### A10 — invariant laundering
Attempt to put an invariant effect into accepted-variable selection. Required result: V3 rejects it.

## Classification

Each attack must end in exactly one of:
- `BLOCKED_AS_DESIGNED`
- `UNCLASSIFIED_AUTHORIZATION_PATH`
- `INCOMPLETE_TEST_EVIDENCE`

## V4 gate outcomes

- `P3_V4_FALSIFICATION_PASS_BOUNDED`
- `P3_V4_AUTHORIZATION_LEAK_FOUND`
- `P3_V4_INCOMPLETE_EVIDENCE`

V5 becomes authorized only under `P3_V4_FALSIFICATION_PASS_BOUNDED` and only for the declared scope. No universal safety/completeness claim follows.
