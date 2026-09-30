# ExecSurface — P8-A3 External Standards / Interoperability Review Protocol

Date: 2026-09-30
Parent program: #100
P8 issue: #114
Status: **PREREGISTERED — EXTERNAL INTEROPERABILITY FEEDBACK REQUIRED**

## Question
Does an external reviewer with relevant supply-chain/attestation context identify a concrete interoperability strength, mismatch, missing semantic binding, or better standards path in the bounded P5 composition?

## Target under review
P5 decision:
`P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`

Composition:
- in-toto Statement v1;
- Runtime Trace v0.1;
- SCAI v0.3;
- SVR v0.2;
- SLSA provenance reference where available;
- GitHub/Sigstore artifact-attestation identity/verification.

This is research architecture, not a claim that the public alpha.4 product exposes the full composition.

## Fixed roles
1. Innovation Scientist / Systems Architect — prefer existing interoperable standards over proprietary reinvention.
2. Anti-Drift / Scientific Integrity Reviewer — blocks standards-name authority and endorsement inflation.
3. Independent Falsifier / Red Team — attacks producer/verifier binding, substitution, replay, duplicate semantics and completeness representation.
4. Independent Critical-Milestone Reviewer — preserves original external guidance and exact standards/version target.

Dynamic specialists:
- in-toto / SCAI / SVR;
- SLSA;
- Sigstore/GitHub attestations;
- evidence semantics / schema design.

## Qualifying feedback
External feedback must make at least one concrete technical claim about:
- representational mismatch;
- producer/verifier identity binding;
- evidence/completeness expressivity;
- replay/substitution boundary;
- duplicate/conflicting semantic fields;
- tooling/ecosystem interoperability;
- an existing standard/predicate that is a better fit;
- a concrete reason a narrow new predicate/extension is necessary.

Generic praise, participation in a standards group, or naming a standard does not qualify.

## Processing
1. Preserve raw guidance before response.
2. Qualify relationship/assistance under P8-A0.
3. Map the guidance to an exact P5 claim or graph edge.
4. Preregister an executable schema/graph/adversarial test where possible.
5. Retain no-fit/incompatibility results.
6. Separate cryptographic validity, provenance identity, semantic authority and observer completeness.

## Prohibited
- treating a signature or standards label as behavioral authority;
- claiming standards-body endorsement;
- inventing a new ExecSurface schema merely to appear novel;
- rejecting external incompatibility because internal tests passed;
- universal interoperability claims from one implementation/reviewer.

## A3 outcomes
- `P8_A3_INTEROPERABILITY_GUIDANCE_ACCEPTED_BOUNDED`
- `P8_A3_INTEROPERABILITY_GAP_REPRODUCED_BOUNDED`
- `P8_A3_EXISTING_COMPOSITION_RETAINED_TESTED_SCOPE`
- `P8_A3_INTEROPERABILITY_FINDING_INCONCLUSIVE`

Current state: **`P8_A3_EXTERNAL_INTEROPERABILITY_FEEDBACK_PENDING`**.
