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

## Gate ledger
- A1 Semantics-v3 repaired admission: PENDING
- A2 Proposition/backend authority: PENDING
- A3 Legitimate-variance anti-poisoning: PENDING
- A4 Attestation/provenance binding: PENDING
- A5 Usability/install/package/CLI/Action: PENDING
- A6 Integrated destructive replay: PENDING
- A7 Reproducible artifact/checksum: PENDING
- Release decision: BLOCKED until all internal gates pass and external-validation boundary is respected.
