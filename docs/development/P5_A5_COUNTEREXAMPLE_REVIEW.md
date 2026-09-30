# ExecSurface — P5-A5 Counterexample Review

Date: 2026-09-30
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Classification: **SCIENTIFIC COUNTEREXAMPLES RETAINED — IMPLEMENTATION BINDING GAP, NOT A STANDARDS REPRESENTATION GAP**

## Frozen gate that produced the finding

Source SHA: `a7753bee6cc9f145adc7823dd82837fa40cfca01`
Workflow run: `36755962689`
Job: `110026152357`
Evidence artifact: `11116268894`
Artifact SHA-256: `97aa4db53e27355d8b2d62e86d6f1ba5c2ee6d7d6e10607b965e4730fa878314`

The boundary gate and static gates passed. The frozen twelve-test corpus executed. Result: **8 passed / 4 failed**. This is a scientific/adversarial result, not a formatting or CI failure.

## Surviving attacks

Four preregistered attacks survived the self-verifying bundle acceptance path:

1. **A5-05 baseline substitution** — a changed baseline digest remained acceptable after attacker-visible outer digest recomputation.
2. **A5-06 current-surface substitution** — a changed current-surface digest remained acceptable after attacker-visible outer digest recomputation.
3. **A5-10 source substitution subcheck** — source identity substitution remained acceptable when the verifier checked workflow/command/host but not source identity.
4. **A5-11 duplicate semantic item subcheck** — a duplicate SVR semantic property remained acceptable.

The other eight frozen attacks failed closed as required.

## Falsifier interpretation

The result demonstrates that an internally self-consistent attestation bundle is not sufficient to prove that the bundle is the one expected by an external verification context. An attacker who is allowed to recompute public outer digests can create a new internally consistent graph after changing baseline/current-surface/source fields unless the verifier supplies an independently frozen expected context.

This does **not** demonstrate a missing in-toto, SCAI, SVR, SLSA, or Sigstore predicate. It demonstrates an implementation binding gap in the A5 acceptance function.

The duplicate-property counterexample is separate: presence checks are insufficient when a semantic field has set/cardinality semantics. The research verifier must reject duplicates before treating order-insensitive reconstruction as canonical.

## Smallest evidence-supported correction

The correction is restricted to the research-only A5 graph-verification layer already introduced before this run:

- verify the bundle against an explicit `ExpectedVerificationContext` containing subject, source, artifact, workflow, command, host, baseline, current surface, evidence digest, observer profile, policy, authority, completeness, verdict, and provenance;
- verify an explicit role->digest graph manifest;
- reject duplicate SVR semantic properties;
- retain order-insensitive canonical reconstruction only where semantics are explicitly order-insensitive;
- rerun the same twelve frozen attacks without changing their acceptance criteria or attack model.

No public crate, public alpha.4 semantics, stable `@v0.1`, predecessor decision, threshold, or attack is changed by this correction.

## Anti-drift ruling

Do not respond to this result by weakening A5-05, A5-06, A5-10, or A5-11. Do not convert frequency, backend name, signature presence, or provenance presence into authority. Do not invent a new attestation predicate merely to encode verifier-side expected context.

The historical failing run and artifact are permanent negative evidence and remain part of the A5 closeout record.
