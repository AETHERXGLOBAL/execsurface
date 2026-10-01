# ExecSurface v0.1.0-alpha.5 clean candidate

Source-of-truth starting point: `integration/post-alpha4-promotion-candidate` at `5fc3bf3ad9c8074045eae45d57faa2e92340d855`.

This file records the clean-candidate execution boundary only. It does not constitute a release decision.

## Fixed rules

- `v0.1.0-alpha.4` and `AETHERXGLOBAL/execsurface@v0.1` remain immutable during candidate assembly.
- No silent baseline-v2 to semantics-v3 reinterpretation.
- No promotion of the retained native-arm64 negative result.
- No backend-name, frequency, similarity, signature, or attestation metadata may raise semantic authority by itself.
- Incomplete, ambiguous, unsupported, or lost evidence remains fail-closed.
- Negative and failed-run evidence is retained.
- No test, threshold, or acceptance rule may be weakened to obtain PASS.
- P8 external validation remains independent and cannot be self-certified by this branch.

## Release gate order

1. Reproduce A1 semantics-v3 repaired admission.
2. Reproduce A2 proposition/backend authority.
3. Reproduce A3 legitimate-variance anti-poisoning.
4. Reproduce A4 attestation/provenance binding.
5. Usability, install, packaging, CLI, and GitHub Action compatibility.
6. Full integrated destructive/adversarial replay.
7. Reproducible artifact and checksum verification.
8. Release decision only after all internal gates pass and the external-validation boundary is respected.
