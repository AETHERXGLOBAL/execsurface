# Alpha.5 team and attack model

## Dynamic expert cells
- Formal semantics / PL: proposition identity, proof requirements, migration invariants.
- Runtime authority: backend/proposition boundaries and fail-closed evidence semantics.
- Rust/API invariants: misuse-resistant types, serialization round-trips, compatibility.
- Provenance/attestation: subject/source/verifier/baseline/current binding.
- Release engineering: clean builds, packaging, install paths, checksums, reproducibility.
- Developer experience: CLI, examples, GitHub Action, error diagnostics, upgrade/rollback.
- Adversarial/property testing: mutation, replay, substitution, ambiguity, downgrade and poisoning.

## Fixed roles
### Innovation Scientist
Must prefer stronger general invariants over ad-hoc test patches. Any proposed correction must answer: can the invalid state be made unrepresentable or rejected at a single trusted boundary?

### Anti-Drift / Scientific Integrity Reviewer
Blocks scope drift, claim inflation, hidden semantic reinterpretation, threshold changes, deleted negative evidence, and any move from internal evidence to external-validation claims.

### Independent Falsifier
Attempts wrong-proposition substitution, subject mutation, authority laundering, ambiguity injection, replay, serialization laundering, migration downgrade, frequency poisoning, attestation substitution, install/package mismatches and tag/action drift.

### Critical-Milestone Reviewer
No gate closes from implementation presence alone. Closure requires reproducible execution evidence and retained failures.

## Non-negotiable destructive invariants
1. Wrong proposition or wrong subject never satisfies a requirement.
2. Adding invalidating ambiguity can never improve admissibility.
3. Empty/default/underspecified requirement never becomes a valid proof contract.
4. Serialization/deserialization cannot lose proposition or authority binding.
5. v2→v3 migration cannot invent proof, completeness, or authority.
6. Backend/profile/version/signature/attestation metadata cannot raise semantic authority alone.
7. Frequency/similarity/recurrence cannot authorize behavior.
8. Loss/incompleteness/unsupported evidence remains non-PASS-eligible.
9. Public alpha.4 semantics and stable Action pointer remain untouched until release decision.
10. Native arm64 negative evidence remains negative unless independently reproved.
