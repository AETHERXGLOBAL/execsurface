# M12.6 Failure 001 — Downstream package blocked by unpublished prospective dependency

Date: 2026-09-28
Gate: M12.6 — Release Artifact / Provenance Dry Run
Workflow run: `36464283655`
Job: `109070309128`
Classification: **HARNESS/PUBLICATION-SEQUENCING FAILURE — NOT PRODUCT CODE FAILURE**
Status: **PRESERVED NEGATIVE EVIDENCE**

## What passed before the failure

The first M12.6 attempt successfully completed:

- frozen source checkout at `16ff98b205f951cf463b6ae5f811f164f4ba4572`;
- public-main immutability check at `db11761e2d75ebca7d4458dc39094f4aed6a26bb`;
- prospective `0.1.0-alpha.4` staging without committing a version bump;
- workspace/internal exact-version coherence checks;
- `cargo fmt --all -- --check`;
- `cargo clippy --locked --workspace --all-targets -- -D warnings`;
- `cargo test --locked --workspace --all-targets`;
- actual `cargo package --locked --allow-dirty --no-verify -p execsurface-model`.

## Failure

The next command attempted:

```text
cargo package --locked --allow-dirty --no-verify -p execsurface-observe
```

Cargo rejected the prospective downstream package because the normalized package dependency requires:

```text
execsurface-model = "=0.1.0-alpha.4"
```

and crates.io currently exposes only earlier public versions. The observed error was:

```text
failed to select a version for the requirement `execsurface-model = "=0.1.0-alpha.4"`
candidate versions found which didn't match: 0.1.0-alpha.3, 0.1.0-alpha.2
```

## Interpretation

This does **not** invalidate the source candidate. It demonstrates a real property of the current crates.io publication chain: downstream prospective packages cannot complete Cargo's registry-backed packaging preparation until their exact prospective internal dependencies have become visible in the registry.

This behavior is consistent with the existing sequential publication workflow and must not be bypassed by pretending every downstream crate can perform a full crates.io dry run before the preceding prospective crate exists publicly.

## Corrected M12.6 assertion

The repaired dry-run harness must therefore:

1. require a real successful package archive and `cargo publish --dry-run` for the dependency-root crate `execsurface-model`;
2. require the *next* dependent crate to fail registry-backed package preparation for the **specific expected reason** that `execsurface-model = =0.1.0-alpha.4` is not yet published;
3. treat any different failure as a gate failure;
4. validate every publishable crate's package file-set with `cargo package --list`;
5. validate the complete exact internal prospective dependency topology from `cargo metadata`;
6. continue binary bundle, provenance, clean-consumer, claim-boundary, and public-line immutability gates only after these publication-sequencing assertions pass.

No public release, crates.io publication, tag, Marketplace pointer, or stable Action pointer was changed by the failed run.
