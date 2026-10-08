# ExecSurface Alpha.6 — Productization Gate Result

Date: 2026-10-08  
Protocol: `docs/development/ALPHA6_PRODUCTIZATION_GATE_PROTOCOL.md`  
Qualified product source: `187c747182321e50645b599a5cd9ee6611fa225f`  
Decision: **ALPHA6_PRODUCTIZATION_GATE_PASS_BOUNDED**

## Decision

Alpha.6 passes the bounded productization gate for the documented Linux x86_64 + native `ptrace` path.

The qualified new-user path is:

`landing -> install -> doctor -> learn -> unchanged PASS -> controlled REVIEW -> init -> GitHub Action`

This decision means the current Alpha.6 path is coherent, executable, and guarded against the onboarding failures exercised here.

It does **not** promote ExecSurface to v1.0, production-stable status, universal Linux support, or independent external validation.

The work remains runtime behavioral verification / execution semantics / semantic-evidence correctness. It is not cybersecurity research.

## Source and release identity

Qualified product source:

`187c747182321e50645b599a5cd9ee6611fa225f`

Release identity at closeout:

- immutable Alpha.5 tag still reports `v0.1.0-alpha.5`;
- immutable Alpha.6 tag reports `v0.1.0-alpha.6`;
- moving `v0.1` channel reports `v0.1.0-alpha.6`.

Alpha.5 remains immutable historical/rollback evidence.

## Retained RED evidence

### Harness-only REDs

The first productization test commits contained a Rust literal escaping defect. Those runs failed before semantic execution and are retained as harness-only RED evidence.

The first Productization workflow also used an invalid `actions/checkout` commit pin. Three jobs failed before testing the product. The exact registry Alpha.6 first-run job still passed. The checkout pin was corrected without weakening any acceptance condition.

### Semantic productization RED — initial acceptance

At source:

`cf8d2f4e8f2c8916b3d99081206917ff5e970e22`

the Productization acceptance test reached semantic execution and produced:

**0/3 PASS**

The three failing propositions were:

1. current-facing landing/status wording did not consistently present Alpha.6 as current rather than final;
2. generated GitHub Actions workflow did not run the wrapped target command as an independent correctness gate before ExecSurface;
3. `init --github-actions` did not provide copyable custody-variable setup commands.

These were treated as productization blockers.

### Semantic productization RED — Quick Start custody handoff

After the first corrections, a second fail-first expansion at:

`ebcd01c3927c7b06cb353da0525c3a8d6fc0ba03`

produced:

**2/3 PASS, 1/3 FAIL**

The remaining failure was:

- Five-Minute Start did not name `EXECSURFACE_BASELINE_DIGEST` and `EXECSURFACE_POLICY_SHA256` or tell the user that `execsurface init` prints the copy-paste setup commands.

This RED propagated through CI, Stage-2 Final Internal Gate, Adversarial Regression, and the Productization Gate. The documentation was corrected without weakening the test.

Formatting-only REDs encountered during the correction chain remain classified as formatting-only and are not rewritten as semantic failures.

## Corrections

### 1. Generated target correctness gate

The generated workflow now runs the requested command as its own GitHub Actions step before the ExecSurface step.

This preserves the Alpha.6 contract that ExecSurface verdict codes describe execution-surface drift and do not replace the wrapped target command's native success/failure result.

The same requested command is then observed by ExecSurface using the documented wrapper.

### 2. Actionable custody setup

`execsurface init --command <cmd> --github-actions` now prints copyable GitHub CLI setup commands for:

- `EXECSURFACE_BASELINE_DIGEST`;
- `EXECSURFACE_POLICY_SHA256`.

The CLI shows how to read the learned `baseline_digest` from `execsurface.lock.json` and how to compute the exact-byte policy SHA-256.

### 3. Current-facing Alpha wording

The landing badge and Action guide now say **Current Supported Alpha**, avoiding wording that could make Alpha.6 appear to be a final/stable v1 product.

The authoritative status document points current users to the Alpha.6 release record while preserving Alpha.5 as historical evidence.

### 4. Quick Start handoff

The Five-Minute Start now states that the generated Action workflow:

- runs the target command independently as a correctness gate;
- requires the two custody variables;
- receives copy-paste setup commands from `execsurface init`;
- expects those values to live in trusted GitHub repository/environment variables rather than checkout files.

## Executable Productization Gate

A dedicated workflow was added:

`.github/workflows/alpha6-productization-gate.yml`

Final run:

- **Alpha.6 Productization Gate — run `37743693100` — SUCCESS**

Jobs:

- Public binary / first-run path — SUCCESS
- crates.io exact Alpha.6 / first-run path — SUCCESS
- Source onboarding contract — SUCCESS
- Stable `@v0.1` / custody and verdict contract — SUCCESS

The gate proves:

- stable `v0.1` resolves to immutable Alpha.6;
- public GitHub release archive download;
- checksum verification;
- GitHub attestation verification;
- exact binary version;
- `doctor`;
- learn once;
- unchanged PASS;
- controlled REVIEW/10;
- baseline byte immutability;
- exact crates.io Alpha.6 installation;
- productization acceptance tests;
- existing self-service regression;
- `cargo publish -p execsurface --locked --dry-run`;
- stable Action PASS/REVIEW/BLOCK/ERROR with externally anchored custody pins.

## Adjacent qualification on the same source

All required top-level workflows completed SUCCESS on the exact qualified product source:

- CI — `37743693104` — SUCCESS
- Stage-2 Final Internal Gate — `37743693161` — SUCCESS
- Adversarial Regression — `37743693148` — SUCCESS
- P9.3 Compatibility Contract — `37743693201` — SUCCESS
- Public Consumer Smoke — `37743693258` — SUCCESS
- Technical Evaluation Pack — `37743693286` — SUCCESS
- Alpha.6 Productization Gate — `37743693100` — SUCCESS

No Stage-2 acceptance assertion was weakened.

## Bounded product state

After this gate, Alpha.6 is the recommended public Alpha for a new user within the documented boundary.

It is now reasonable to describe Alpha.6 as:

**developer-usable public Alpha with a qualified onboarding path**

It is still not a stable v1.0 production release.

## Final decision

**ALPHA6_PRODUCTIZATION_GATE_PASS_BOUNDED**

Next maturity work, if pursued, belongs to a later v1/production-readiness program and must not be inferred from this Alpha.6 productization result alone.
