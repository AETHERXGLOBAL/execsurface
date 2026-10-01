# ExecSurface Alpha.5 — A7 Final Release-Artifact Destructive Closeout

## Decision

`ALPHA5_A7_RELEASE_ARTIFACT_DESTRUCTIVE_EQUIVALENCE_PASS — BOUNDED INTERNAL`

This decision applies only to the frozen Alpha.5 product source and the exact repaired dry-run artifact identified below. It does not authorize public release and does not close P8 external validation.

## Frozen product source

- Frozen SHA: `5067200452c174da6bc8d9d7ecf6957ee379f0a2`
- Public alpha.4 source remains: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Stable `v0.1` remains unchanged during this gate.

## Retained first A7 failure

Run `36901600086` is retained as material negative evidence.

It exposed two independent release-artifact gaps:

1. The first staged binary was built on Ubuntu 24.04 and required `GLIBC_2.39`, so it failed to execute on Ubuntu 22.04.
2. An independent frozen-source rebuild was not byte-for-byte identical to the staged release binary, proving that the earlier same-run reproducibility result did not establish cross-run/path reproducibility.

Neither failure was relabeled, deleted, or bypassed. The public release remained blocked.

## Structural repair

The release builder was repaired without changing frozen product semantics or weakening the A7 acceptance criteria.

Repaired builder commit: `10a27bea65044bcca575454010c18f62540fa15b`

Repaired dry-run:

- run: `36901991871`
- artifact: `11182231176`
- GitHub artifact digest: `sha256:476d8aaa86d47934b042c947b71b3aaca35ce52498783f9ff2d6c41f9dc2c798`
- builder baseline: Ubuntu 22.04
- Rust toolchain: `1.90.0`
- deterministic controls: fixed `SOURCE_DATE_EPOCH`, source-path remapping, disabled incremental compilation, removed linker build-id, fixed locale/timezone, deterministic tar ordering/ownership/mtime, `gzip -n`

The repaired builder passed full staged qualification, two-path byte-identical binary construction, GLIBC ceiling enforcement, deterministic bundle reproduction, and provenance verification.

## Final A7 artifact attack

Workflow source commit: `42005df5686fd20da9f8114747b1e2f69d260942`

Accepted run: `36902486872` — `SUCCESS`

### Artifact binding and provenance

- exact repaired dry-run run and artifact bound by ID and digest;
- alpha.4 and stable `v0.1` anchors rechecked;
- retained failed A7 run explicitly bound as counterevidence;
- bundle SHA-256 sidecar verified;
- in-toto Statement v1 / SLSA provenance subject digest verified;
- frozen source SHA, prospective version/tag, target, Ubuntu 22.04 builder baseline, deterministic path-remap flag, and source-date epoch verified;
- archive rejected unsafe absolute paths, traversal, duplicates, symlinks, hardlinks, and device members.

Binding artifact:
- `11182915253`
- digest `sha256:55b6900a716984cc3f81d7f3651ce239aacd61465378a68f973e37ad44fbb0b6`

### Black-box runtime compatibility

The exact release artifact, not a source-tree build, passed the CLI/verdict/privacy contract on both supported test environments:

- Ubuntu 22.04 artifact `11182042177`, digest `sha256:1f10a370ecfa889b061f7081b71675ee62528ab7c80ca61dfaa9f8f6d5adeeb5`
- Ubuntu 24.04 artifact `11181813286`, digest `sha256:e007c16bae02411fe31ad9e9b4422f2743c7acb63ca1acebffa3cb34dc6c16f3`

The black-box checks exercised version/help/doctor, baseline learning, PASS behavior, REVIEW exit code `10`, missing-baseline ERROR exit code `2`, and privacy-sentinel non-leakage.

### Frozen-source byte equivalence

An independent Ubuntu 22.04 rebuild from the exact frozen SHA, using the repaired deterministic build contract, was required to match the release artifact byte-for-byte.

Both binaries produced:

`sha256:11d1f70d3e6bd6526eff4889b90cba4a0ad95ce09d489643e7f3b703e455f646`

The highest observed required GLIBC symbol version was `GLIBC_2.34`, within the preregistered `<= 2.35` ceiling.

Bidirectional baseline compatibility also passed: a baseline learned by the artifact was accepted by the independent rebuild and vice versa, while both independently produced REVIEW for the same injected drift.

Source-equivalence artifact:
- `11182087236`
- digest `sha256:7c0b126a3c87b57c94fd01504b0689c9294a6043f7b1616c1f80a3540f033ddc`

### Tamper and substitution attacks

The gate rejected or detected:

- archive byte mutation;
- archive truncation;
- post-extraction binary mutation;
- provenance subject substitution;
- provenance digest mismatch;
- archive path traversal;
- symlink member injection.

### Final closeout evidence

Closeout artifact:
- `11182356642`
- digest `sha256:b775001884d838b4141d31cf07f631a82deed8498c5b766c3d1c03529abc12c6`

## Scientific / release boundary

What this establishes, within the tested internal scope:

- the repaired Alpha.5 release artifact executes on Ubuntu 22.04 and Ubuntu 24.04;
- the tested artifact is byte-for-byte reproducible from the exact frozen source under the recorded deterministic build contract;
- its tested baseline/verdict behavior matches the independent frozen-source rebuild;
- its bundle/provenance binding is internally coherent;
- the tested tamper/substitution classes are rejected or detected;
- the first failed final-artifact gate remains retained as negative evidence.

What this does **not** establish:

- universal bug-freedom;
- support for Linux arm64;
- external validation;
- production safety across all environments;
- semantic authority from signatures, provenance, backend identity, frequency, or similarity alone.

## Publication state

`INTERNALLY RELEASE-ARTIFACT-QUALIFIED — PUBLIC RELEASE STILL BLOCKED BY P8 EXTERNAL-EVIDENCE GATE`

- `release_authorized=false`
- `p8_external_validation_closed=false`
- no Alpha.5 public tag, crates publication, release asset publication, or stable Action movement is authorized by this A7 decision alone.
