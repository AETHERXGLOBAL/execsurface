# ExecSurface v0.1.0-alpha.5 hardening execution

Branch: `release/v0.1.0-alpha.5-hardening`
Base: `b0a1233661c22f2c17adf0622b68a6796c70b3c9`
Issue: #117

## Team model
Dynamic experts by gate, with fixed roles for Innovation Scientist, Anti-Drift / Scientific Integrity Reviewer, Independent Falsifier, and Critical-Milestone Reviewer.

## Execution law
- Failure-first: record every failing run before correction.
- Frozen assertions: no weakening tests, thresholds, or acceptance criteria.
- Fail closed on incomplete / ambiguous / unsupported / lost evidence.
- Public alpha.4 and stable Action remain unchanged until release decision.
- P8 external validation is independent and cannot be self-certified.

## Evidence ledger

### A1-A4 reproof
- Workflow: `Alpha5 hardening reproof A1-A4`
- Run: `36890949554`
- Source: `911f7fe11f5ae63ed1ca842274625fb24f775b9f`
- Result: **PASS**.
- Identity/public-boundary check: PASS.
- A1 repaired Semantics-v3 admission and v2-adjacent regression: PASS.
- A2 proposition/backend authority, live ptrace authority and Tetragon import falsification: PASS.
- A3 legitimate-variance 12-test corpus and poisoning/false-PASS falsifier: PASS.
- A4 attestation/provenance 12-attack corpora and standards-composition replay: PASS.
- Historical negative evidence remained retained.

### A5 first execution — retained failure
- Workflow: `Alpha5 A5 usability and distribution gate`
- Run: `36891184129`
- Source: `9480e5db0baab9e8cc2355c43c91801be2b4c79e`
- Result: **FAIL — DISTRIBUTION / UX**.
- Candidate `cargo package` and path install completed successfully.
- Public alpha.4 checksum/behavior regression passed.
- Stable `AETHERXGLOBAL/execsurface@v0.1` PASS/REVIEW contract passed.
- Composite Action source contract passed.
- Failure occurred in the zero-assistance candidate CLI smoke because top-level `execsurface --help` omitted valid public commands `doctor`, `init`, and `check`, despite those commands being implemented and documented for users.
- No scientific/runtime semantics failure was observed in this run.
- Failed run is retained permanently as negative usability evidence.

### Bounded A5 correction
- Correction commit: `b447942a43419b0bcfd536eb5540a538787d55e5`.
- Scope: top-level CLI help only; no runtime semantics, policy, baseline, authority, observer, or acceptance-rule change.
- Added the implemented public commands `doctor`, `init`, and `check` to the usage summary.
- One-shot correction workflow was removed after use; removal commit: `9e2b8f7fc4554c86e31669a599ec307bfc0846bb`.
- A5 reproof remains **PENDING** and must use the same acceptance assertions.

## Gate ledger
- A1 Semantics-v3 repaired admission: **PASS — reproof required after UX source change**
- A2 Proposition/backend authority: **PASS — reproof required after UX source change**
- A3 Legitimate-variance anti-poisoning: **PASS — reproof required after UX source change**
- A4 Attestation/provenance binding: **PASS — reproof required after UX source change**
- A5 Usability/install/package/CLI/Action: **FAIL retained; correction applied; reproof PENDING**
- A6 Integrated destructive replay: PENDING
- A7 Reproducible artifact/checksum: PENDING
- Release decision: BLOCKED until all internal gates pass and external-validation boundary is respected.
