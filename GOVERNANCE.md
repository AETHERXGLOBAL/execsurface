# ExecSurface Governance

AETHER X GLOBAL maintains ExecSurface as an open-source engineering project with evidence-gated product and research decisions.

## Product identity

ExecSurface is a **runtime behavioral-integrity and verification layer for observed execution-surface drift**.

The maintained public boundary is intentionally narrower than antivirus, EDR, SIEM, malware detection, sandboxing, general observability, or proof that software is safe.

A change that materially alters that identity requires an explicit governance decision before implementation is treated as the new product direction.

## Decision model

Architecture- and claim-affecting work follows:

`QUESTION -> HYPOTHESIS -> PREREGISTERED TEST -> ADVERSARIAL TEST -> EVIDENCE -> DECISION`

Decisions are recorded through committed architecture documents, milestone/gate records, issues, pull requests, release records, or other durable repository evidence.

## Non-negotiable evidence rules

- evidence before claims;
- baseline remains separate from policy;
- privacy-safe defaults;
- observer authority and completeness are explicit;
- incomplete, ambiguous, unsupported, or lost evidence cannot silently become PASS;
- backend name, frequency, similarity, or attestation metadata does not create universal authority;
- preserve material failures and negative results;
- no threshold or acceptance-rule weakening to manufacture a PASS;
- deterministic/reproducible behavior is claimed only where demonstrated;
- internal validation is not relabeled as independent external validation.

## Release governance

Public releases must identify:

- the immutable release/tag source;
- supported platform scope;
- installation/distribution surfaces;
- release evidence and material limitations;
- any open validation or evidence boundary.

Stable moving channels such as `v0.1` may advance only after the release-specific public checks required by the release protocol have passed.

Historical release failures remain part of the evidence trail even after repair.

## External review

External criticism, reproduction failure, no-fit findings, interoperability guidance, and counterexamples are first-class evidence.

Participation, routing, acknowledgement, praise, or membership in an external community does not constitute endorsement or validation.

External findings are processed as:

`CLAIM -> EXTERNAL RESULT/CRITICISM -> QUALIFICATION -> TEST/REPRODUCTION -> EVIDENCE -> DECISION`

## Repository hygiene

Current user-facing documentation should remain easy to find from `README.md` and `docs/README.md`.

Superseded operational material containing scientific, release, failure, or governance evidence is archived rather than deleted. Archiving is organizational hygiene; it must not alter the historical outcome of a gate or experiment.

## Security-sensitive information

Potential vulnerabilities or sensitive implementation findings follow [SECURITY.md](SECURITY.md) rather than normal public issue/evidence channels.

## IP review

If a component may contain strategically sensitive intellectual property, stop public publication of that component and mark it:

**IP REVIEW REQUIRED**

No claim of patentability, novelty, or first-in-world status is made without dedicated prior-art and legal review.
