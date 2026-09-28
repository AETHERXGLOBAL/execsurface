# M12.6 Failure 002 — Release-note sentinel phrase mismatch

Date: 2026-09-28
Workflow run: `36465008842`
Job: `109072771257`
Classification: **HARNESS/WORDING SENTINEL FAILURE — NOT PRODUCT OR ARTIFACT FAILURE**
Status: **PRESERVED NEGATIVE EVIDENCE**

## What passed before the failure

The repaired M12.6 run passed all substantive release-engineering gates before the release-note sentinel:

- staged prospective `0.1.0-alpha.4` version coherence;
- exact internal dependency topology validation;
- full fmt/clippy/test workspace gates;
- publishable package file-set audit;
- actual `execsurface-model` package creation;
- `cargo publish --dry-run` for the registry-front package;
- explicit expected failure proving downstream `execsurface-observe` packaging is blocked only by unpublished prospective `execsurface-model = =0.1.0-alpha.4` registry visibility;
- optimized Linux x86_64 release binary build;
- exact-format release bundle and SHA-256 verification;
- release-workflow preparation preview;
- in-toto/SLSA-shaped local dry-run provenance generation and subject digest verification;
- clean extraction consumer smoke: `doctor`, learn/check PASS, controlled drift REVIEW with exit code 10;
- before/after proof that public `main`, `v0.1`, and `v0.1.0-alpha.3` were not mutated and `v0.1.0-alpha.4` was not created.

The evidence artifact for this run was uploaded as artifact ID `10989152317` with uploaded-artifact SHA-256:

`af671f40dc01db683016b934a87827c5ef0b79b3c3949a186c69eb0e4abaaec8`

## Failure

The release-note sentinel searched for the exact substring:

`not kernel-object identity`

The draft already stated the stronger and semantically correct sentence:

`ptrace pathname observations remain pathname access-attempt metadata under the recorded observer; they are not represented as kernel-object identity;`

Because the word `represented as` interrupts the exact substring, the literal grep failed.

## Resolution rule

Do not weaken the claim. Make the release-note boundary additionally explicit with a standalone sentence that preserves the same meaning:

`Pathname access-attempt metadata is not kernel-object identity.`

Then rerun the complete M12.6 gate rather than reclassifying the failed run as PASS.
