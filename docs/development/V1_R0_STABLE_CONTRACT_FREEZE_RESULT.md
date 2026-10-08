# ExecSurface v1-R0 — Stable Contract Freeze Result

Date: 2026-10-08  
Protocol: `docs/development/V1_R0_STABLE_CONTRACT_FREEZE_PROTOCOL.md`  
Qualified source: `8840a75e6eea228b09dd8e6d9f41e774f811e195`  
Decision: **V1_R0_STABLE_CONTRACT_FROZEN_BOUNDED**

## Decision

The bounded stable v1 contract is frozen.

This closes the **contract-definition** step only. It does **not** authorize a v1 release candidate, a stable release, or any maturity claim beyond the evidence recorded here.

The project remains runtime behavioral verification / execution semantics / semantic-evidence correctness. It is not cybersecurity research.

## Fail-first evidence retained

### Initial contract sentinels

The first executable contract-sentinel run reached semantic execution with:

**0/4 PASS**

Failures were:

1. the compatibility document did not declare a frozen bounded v1 contract;
2. the stable Action contract did not explicitly include the Alpha.6 custody inputs;
3. the build/test nondeterminism and performance boundaries were not explicitly frozen;
4. the upgrade contract did not identify Alpha.6/profile-4 as the current supported source.

Acceptance assertions were not weakened.

After the contract was updated, the same four assertions became 4/4 PASS.

Two additional anti-drift sentinels were then added for:

- install-route-specific environment support;
- stable support/deprecation/rollback policy.

Final sentinel result:

**6/6 PASS**

### Environment probe — material negative evidence

The exact public Alpha.6 GitHub Release binary failed on Ubuntu 22.04 x86_64 with:

`GLIBC_2.39 not found`

The failure was retained as a first-class compatibility result and converted into an explicit negative-floor regression.

The same exact public prebuilt artifact passes on Ubuntu 24.04 x86_64.

This prevents the v1 contract from claiming universal Linux x86_64 compatibility for the current prebuilt-binary strategy.

### crates.io / local-build evidence

Exact Alpha.6 crates.io installation, followed by:

`doctor -> learn -> unchanged check`

passed on:

- Ubuntu 22.04 x86_64;
- Ubuntu 24.04 x86_64.

This is intentionally classified separately from the prebuilt-binary path.

### Declared Rust MSRV

The workspace declares:

`rust-version = "1.82"`

The V1-R0 probe successfully ran:

`cargo +1.82.0 check --locked -p execsurface`

Therefore the declared source floor remains evidence-backed at this gate.

### Harness-only formatting RED

After adding the final environment/support sentinels, ordinary CI and Stage-2 failed only because `rustfmt` required formatting changes in the new test file.

No assertion or product semantics failed.

The exact same assertions were retained and only formatting was corrected.

## Frozen stable decisions

### 1. Verdict contract

Stable v1 candidate meanings:

- PASS = 0
- ERROR = 2
- REVIEW = 10
- BLOCK = 20

Incomplete, ambiguous, unsupported or lost required evidence cannot silently become PASS.

### 2. Target outcome

**Decision: preserve current semantics.**

Target exit code/signal remains report metadata and is not verdict-bearing.

`ExecSurface: PASS` does not mean the wrapped target command succeeded.

The generated CI path preserves a separate native target-command correctness gate.

No `fail-on-target-failure` feature is required for v1.

### 3. Baseline compatibility

Stable candidate facts:

- lock schema 2;
- digest format 2;
- current corrected normalization profile 4;
- Alpha.6/profile-4 is the current supported upgrade source;
- Alpha.5/profile-3 remains historical evidence and an explicit semantic incompatibility boundary;
- no automatic baseline migration/relearn;
- ordinary read/check does not mutate baseline evidence.

### 4. Policy compatibility

Frozen v1 candidate policy contract:

- schema 1: legacy-read compatibility / deprecated for new authoring;
- schema 2: default authored/public policy schema;
- schema 3: stable opt-in extension.

Stable schema-3 matchers:

- `path_resolution`
- `open_intent`
- `rename_from_class`
- `rename_from_prefix`
- `rename_from_resolution`

Schema 3 does not reinterpret schema 1/2 behavior.

### 5. Stable CLI surface

Candidate stable commands:

- `doctor`
- `init`
- `learn`
- `check`
- `--version` / `version`
- `--help` / `help`

Not stable-by-default:

- `observe`
- experimental libbpf/eBPF behavior;
- `render-error`;
- internal Rust crate APIs.

### 6. Stable GitHub Action surface

Candidate v1 stable inputs include:

- `command`
- `baseline`
- `policy`
- `expected-baseline-digest`
- `expected-policy-sha256`
- `require-custody`
- `fail-on-review`
- `upload-artifact`
- `artifact-name`

Candidate stable outputs include:

- `verdict`
- `exit-code`
- `report-json`
- `summary-markdown`
- `sarif-status`
- `artifact-url`
- `artifact-digest`

### 7. Workload-support boundary

The stable contract does not promise zero drift for every syntactically unchanged command.

Highly nondeterministic build/test graphs may legitimately produce REVIEW under exact baseline semantics.

No general variance suppression, broad temp/cache whitelist or automatic multi-run authorization is introduced by this gate.

### 8. Platform/install-route boundary

Architecture boundary remains:

- Linux x86_64;
- native ptrace reference observer.

Current prebuilt-binary evidence:

- Ubuntu 24.04-class / glibc-2.39-class x86_64: qualified current path;
- Ubuntu 22.04: current Alpha.6 prebuilt artifact incompatible.

Current local-build/crates evidence:

- Ubuntu 22.04: PASS;
- Ubuntu 24.04: PASS.

This does not claim universal Linux distribution/kernel/container compatibility.

### 9. Performance contract

v1 has no universal low-overhead promise.

Native ptrace overhead is workload-dependent and may be material.

The v1 RC must characterize representative declared workload classes under a frozen protocol. Negative/no-fit outcomes remain valid.

### 10. Support/deprecation/rollback

`docs/SUPPORT_POLICY.md` freezes:

- immutable exact `v1.x.y` identities;
- deliberately movable `@v1` channel;
- no invented fixed calendar SLA;
- compatibility/deprecation rules within v1;
- bad-release rollback without rewriting user baseline/policy evidence.

## Final exact-source qualification

All required top-level workflows completed SUCCESS on the exact qualified source `8840a75e6eea228b09dd8e6d9f41e774f811e195`:

- V1-R0 Stable Contract Freeze — run `37749702011` — SUCCESS
- CI — run `37749701962` — SUCCESS
- Alpha.6 Productization Gate — run `37749701987` — SUCCESS
- P9.3 Compatibility Contract — run `37749701935` — SUCCESS
- Stage-2 Final Internal Gate — run `37749701945` — SUCCESS
- Adversarial Regression — run `37749702040` — SUCCESS

V1-R0 jobs additionally prove:

- frozen contract sentinels — SUCCESS;
- retained Alpha.6 prebuilt incompatibility on Ubuntu 22.04 — SUCCESS as negative-evidence assertion;
- Alpha.6 public binary on Ubuntu 24.04 — SUCCESS;
- Alpha.6 crates.io local build on Ubuntu 22.04 — SUCCESS;
- Alpha.6 crates.io local build on Ubuntu 24.04 — SUCCESS;
- declared Rust 1.82 source floor — SUCCESS.

## P9 consequence

P9.3 moves from “contract undefined/open” to:

**CONTRACT FROZEN / EXACT v1 RC EVIDENCE PENDING**

P9.3 itself remains open because final closeout still requires the exact frozen v1 RC upgrade/rollback and consumer proof.

P9.1 and P9.5 external evidence remain independently open.

V1-R1 release-control-plane hardening remains the next internal engineering gate.

## Final decision

**V1_R0_STABLE_CONTRACT_FROZEN_BOUNDED**

No product feature was added to manufacture this result.
