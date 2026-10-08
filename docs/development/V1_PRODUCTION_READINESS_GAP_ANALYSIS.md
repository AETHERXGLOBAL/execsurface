# ExecSurface v1 — Production Readiness Gap Analysis

Date: 2026-10-08  
Source of truth audited: `main@8fc424e251d4fbdcdb234d19c88903f4e57c036d`  
Current public release: `v0.1.0-alpha.6`  
Current stable Alpha Action channel: `v0.1 -> v0.1.0-alpha.6`

Decision:

**V1_PRODUCTION_READINESS_GAP_ANALYSIS_COMPLETE — REWORK_REQUIRED_BEFORE_RC**

## Classification

This analysis concerns runtime behavioral verification, execution semantics, semantic-evidence correctness, product reliability, compatibility, release engineering and operational supportability.

It is **not cybersecurity research**.

No item below is authorized merely to increase feature count. Stable v1 requires a smaller, explicit, evidence-backed contract rather than a broader product.

## Executive conclusion

Alpha.6 is already a developer-usable public Alpha with a qualified onboarding path. Internal/public-artifact maturity is strong.

The repository is **not yet ready to freeze a v1.0 release candidate**.

The remaining blockers are concentrated in five areas:

1. independent execution/use evidence on the actual supported public product;
2. final stable-contract decisions and RC-specific compatibility proof;
3. v1-capable release/control-plane mechanics and immutable-tag governance;
4. a product decision for noisy build/test workloads that do not satisfy exact single-run baseline stability;
5. an explicit production support/performance/environment contract.

No evidence supports opening a general feature-development phase before these are resolved.

---

## Current evidence baseline

### Already strong / retained

- Alpha.6 Stage-2 remediation qualified under the retained R0-R8 gate chain.
- Post-R8 merge-authorization destruction reached 17/17 PASS before merge.
- Alpha.6 Productization Gate closed as:
  `ALPHA6_PRODUCTIZATION_GATE_PASS_BOUNDED`.
- Public binary, exact crates.io Alpha.6 install, `doctor`, baseline, PASS, REVIEW, Action custody and PASS/REVIEW/BLOCK/ERROR paths are executable gates.
- `v0.1` resolves to immutable `v0.1.0-alpha.6`.
- Alpha.5 remains immutable historical/rollback evidence.
- Main has an active repository ruleset requiring pull requests, blocking deletion/non-fast-forward, requiring current status checks and allowing no bypass by the current user.
- P9.2 public-consumer reliability has prior closeout evidence and is further strengthened by the Alpha.6 Productization Gate.
- Pre-v1 P9.3 compatibility harness already proves core baseline/schema/verdict/Action/rollback properties, but not against a frozen v1 RC.

### Important historical negative evidence retained

- build/test workloads can exhibit substantial legitimate execution-surface variance under exact single-run baseline semantics (#59/#60/#62);
- the research variance program did not establish a safe general public variance mechanism;
- ptrace all-syscall stop/resume cost is a measured causal overhead source; no accepted transparent >=10% native-ptrace optimization was found;
- target-command exit/signal is report metadata rather than an ExecSurface verdict input (#143);
- external validation cannot be manufactured from internal CI, downloads, stars or outbound invitations.

---

# Gate-by-gate current state

## P9.0 — Production criteria freeze

**State: PASS / REFRESHED BY THIS ANALYSIS**

The production-readiness contract exists in `docs/PRODUCTION_READINESS.md` and issue #129.

The criteria remain valid. Their factual baseline was Alpha.5-era and must now be interpreted against Alpha.6 plus the Productization Gate.

No criterion is weakened by this refresh.

## P9.1 — Independent external evidence

**State: BLOCKED — ONE SIDE OF THE MINIMUM IS STILL MISSING**

P8 current closeout requires two distinct current external evidence records:

1. execution/use evidence from A1 zero-assistance reproduction or A4 external real workload; and
2. challenge/interoperability evidence from A2/A3.

The challenge/interoperability side is materially satisfied by the processed Probity record `P8-EXT-0004`.

The execution/use side remains unclosed for the supported public product.

Issue #164 is genuine unaffiliated zero-assistance evidence on an evaluator-owned CounterProof workload, but its own frozen record explicitly identifies:

- product surface: experimental `observe --evidence-output`;
- product stability: experimental/unreleased;
- qualification state: unqualified;
- P8-A1 / P8-A4 / P9.5: not claimed.

Therefore #164 is valuable external evidence and friction input, but this analysis does **not** relabel it into public-product P9.1 closeout evidence.

### Closeout requirement

Obtain and qualify at least one current external execution/use record against the supported public ExecSurface path, preserving the unassisted initial result.

A single properly qualified external real-workload record may satisfy both the missing execution/use side of P9.1 and P9.5.

---

## P9.2 — Public-consumer reliability

**State: PASS FOR CURRENT ALPHA6 / MUST REPLAY ON V1 RC**

Issue #137 is closed.

Current Alpha.6 Productization evidence additionally proves:

- public release binary;
- checksum;
- attestation;
- exact version;
- `doctor`;
- learn once;
- unchanged PASS;
- controlled REVIEW/10;
- baseline immutability;
- exact crates.io install;
- stable `@v0.1`;
- custody;
- Action PASS/REVIEW/BLOCK/ERROR;
- packaging dry-run.

No new product feature is needed to satisfy the current public-consumer gate.

For v1, the same contract must be replayed against the exact RC/public artifact and `@v1` channel.

---

## P9.3 — Compatibility & stability contract

**State: PARTIAL / BLOCKING BEFORE RC**

Issue #134 remains open by design.

The pre-v1 executable harness is strong, but its own closeout explicitly requires rerun against a frozen v1 RC.

Four contract questions must be closed before RC freeze.

### P9.3-A — Current upgrade source must be Alpha.6

The existing contract text is partly Alpha.5-centered.

The supported upgrade source is now Alpha.6.

Alpha.5 remains required historical evidence for the explicit profile-3/profile-4 semantic boundary and rollback reproduction, but v1 upgrade qualification must start from the current supported public Alpha.6 path.

### P9.3-B — Policy schema v3 must receive an explicit stable classification

Current code defines:

- legacy policy schema v1;
- default/public policy schema v2;
- opt-in policy schema v3.

Alpha.6 publicly documents v3 as opt-in while retaining v1/v2 meanings.

The current compatibility document still describes the current policy schema primarily as v2.

Before v1, choose and freeze one explicit contract:

- v1 stabilizes both retained v2 semantics and the documented opt-in v3 matchers; or
- v3 remains explicitly experimental/non-stable and v1 promises only the retained v2 surface.

Silence is not acceptable.

### P9.3-C — Target outcome decision (#143)

Alpha.6 intentionally preserves:

`ExecSurface PASS != wrapped target command succeeded`.

Productization reduced the footgun by generating a separate target-command correctness gate.

Before stable v1, #143 requires an explicit versioned decision:

- **Preserve** target exit/signal as report metadata and freeze that as stable v1 semantics; or
- introduce a separately versioned target-outcome output/gate/policy behavior and qualify compatibility.

This analysis does **not** require changing the current semantics. It requires deciding and freezing them.

### P9.3-D — Final RC upgrade/rollback rehearsal

After the above contract is frozen:

- upgrade from Alpha.6 to the exact v1 RC;
- explicit profile-3 Alpha.5 incompatibility behavior remains proven;
- unsupported schema states remain explicit ERROR;
- no baseline/policy mutation;
- rollback to the previously qualified public release works without rewriting evidence;
- `@v1` Action inputs/outputs and verdict enforcement are qualified.

---

## P9.4 — Repository and release control plane

**State: PARTIAL / HARD BLOCKER BEFORE V1 RELEASE**

### What is already closed

Active ruleset `Protect main` (`24429386`) currently proves:

- default branch targeted;
- deletion blocked;
- non-fast-forward/force-push blocked;
- pull request required;
- stale reviews dismissed on push;
- review threads required resolved;
- current checks required;
- no bypass actors;
- current user cannot bypass.

This supersedes the original Alpha.5-era “no branch protection” finding.

### Remaining blocker 1 — required-check set is narrow

The current ruleset requires only:

`Rust / Linux x86_64`.

That is useful, but a stable-release control plane should explicitly decide which release-critical gates are required before v1-affecting source can merge.

At minimum review whether these must become mandatory for relevant v1 changes:

- Stage-2 Final Internal Gate;
- Adversarial Regression;
- P9.3 Compatibility Contract;
- Productization / future v1 consumer gate;
- release-control-plane tests.

Do not add checks mechanically; select the smallest set that actually protects the stable contract.

### Remaining blocker 2 — immutable-tag protection is Alpha.5-specific

Current repository rulesets show an immutable tag rule only for:

`refs/tags/v0.1.0-alpha.5`

under ruleset `24430022`.

No current repository ruleset was found protecting `v0.1.0-alpha.6`, and no predeclared generic `v1.*` immutable release-tag rule is present.

Before creating a stable v1 release identity, an administrator must configure and this program must re-verify a rule that prevents unauthorized update/deletion of immutable v1 release tags.

The moving stable channel `v1` must remain deliberately movable under the release process and must not be confused with the immutable `v1.0.0` identity.

### Remaining blocker 3 — current promotion workflow is v0.1-line specific

`.github/workflows/promote-release.yml` currently requires:

`stable_channel == v0.1`.

It therefore rejects a correct v1 release request using `stable_channel: v1`.

### Remaining blocker 4 — current release workflow is Alpha-line specific

`.github/workflows/release.yml` currently:

- validates promotion only for tags matching the `v0.1.*` line;
- moves the `v0.1` channel;
- creates/edits the GitHub Release with `--prerelease` unconditionally.

Therefore the existing release pipeline **cannot correctly publish a stable v1.0.0 release**.

This must be corrected and tested before v1 RC/release authorization.

The correction must preserve the working Alpha line; no Alpha.6 tag or historical release is rewritten.

---

## P9.5 — External real-workload production evidence

**State: BLOCKED**

Issue #140 remains open.

The required evidence is an external evaluator using a workload they control/select, preserving the initial result before AETHER X troubleshooting.

Current external contributions and the experimental typed-evidence trial are useful but must not be relabeled to satisfy this gate without meeting the frozen A4/P9.5 requirements.

### Efficient closeout path

One external evaluator using **public Alpha.6** on their own real workload can potentially close:

- the missing execution/use side of P9.1; and
- P9.5;

provided the evidence is independently qualified under #114.

---

## P9.6 — v1 RC qualification

**State: BLOCKED BY P9.1 / P9.3 / P9.4 / P9.5 AND THE SUPPORT-SCOPE DECISIONS BELOW**

Do not create a v1 RC merely to create momentum.

Once prior blockers close:

- freeze exact RC source;
- run source + Stage-2 + destruction/adversarial + compatibility + productization gates;
- build exact stable artifact;
- consume it from clean supported environments;
- qualify `@v1`;
- perform Alpha.6 -> v1 upgrade;
- perform rollback;
- prove stable GitHub Release classification, not prerelease;
- verify v1 immutable tag protection;
- retain every failed attempt.

---

## P9.7 — stable release decision

**State: BLOCKED**

Allowed outcomes remain:

- `RELEASE_V1_0`
- `REWORK_REQUIRED`
- `REMAIN_ALPHA_BETA`

No date or confidence score can replace missing gate evidence.

---

# Additional stable-v1 gaps outside the simple P9 row labels

## V1-G1 — Build/test workload stability must be scoped explicitly

**State: BLOCKING PRODUCT-SCOPE DECISION**

Issue #59 remains open and labeled `known-limitation`.

Preserved evidence shows that full build/test graphs can produce substantial legitimate variance under exact single-run baseline semantics.

The later user-like external CLI program materially narrowed the issue:

- compiled/user-like CLI surfaces showed repeatable zero-finding unchanged PASS;
- full build/test graphs remain a distinct noisy boundary.

The research variance program did not establish a safe general public variance mechanism. That negative result is retained.

Current onboarding still uses commands such as:

`cargo test --locked`

as a primary example.

Before stable v1 choose one path:

### Option A — qualify build/test workloads

Provide a bounded, falsified solution/evidence that makes the declared build/test class operationally usable without hiding meaningful drift.

### Option B — narrow the stable v1 claim

Keep exact semantics unchanged and explicitly state that highly nondeterministic build/test graphs may produce legitimate REVIEW and are outside the “unchanged should normally PASS” stable experience unless the user supplies an appropriate explicit policy/workflow.

Do not invent a variance feature solely to obtain v1.

The strongest current default is **Option B unless new evidence justifies A**.

---

## V1-G2 — Production performance envelope is not frozen

**State: QUALIFICATION GAP, NOT AN AUTOMATIC OPTIMIZATION REQUIREMENT**

Historical exact-host evidence established material ptrace overhead:

- pinned ripgrep: about 4.52x / +401 ms in the accepted attribution run;
- pinned fzf: about 3.20x / +251 ms;
- earlier full build/test measurements included multi-second absolute overhead.

The cost source was causally attributed to all-syscall ptrace stop/resume work.

Two bounded native optimization candidates failed the preregistered value threshold and were correctly left unmerged.

Stable v1 does **not** require a faster backend merely to change the version number.

It does require:

- current-RC performance characterization on representative declared workload classes;
- a documented operational envelope;
- no misleading “low overhead” claim without evidence;
- P9.5 external evidence allowed to conclude `NO_FIT / TOO_COSTLY`.

Performance failure may lead to a narrower v1 support claim or remaining pre-v1; it must not trigger threshold weakening.

---

## V1-G3 — Supported environment floor is underspecified

**State: CONTRACT GAP**

Current public support wording is:

`Linux x86_64 + native ptrace`.

The durable release binary is currently built on Ubuntu 24.04 and current release consumers are heavily Ubuntu-24.04 based.

Historical Alpha.5 qualification also exercised additional environments, but that does not automatically define a permanent v1 compatibility floor.

Before v1 freeze, document and test the exact supported environment contract, including what is promised about:

- minimum/qualified Linux kernel range or kernel assumptions;
- libc/userland compatibility expectations for the published binary;
- hosted CI vs container/VM restrictions;
- ptrace/Yama/capability requirements;
- which clean environments must be release-gating.

Do not claim “all Linux x86_64”.

---

## V1-G4 — Stable support/deprecation lifecycle is not documented

**State: CONTRACT GAP**

No dedicated stable support policy was found defining:

- which v1.x versions receive fixes;
- how long old minor/patch releases remain supported;
- deprecation notice expectations;
- how stable Action `@v1` moves;
- emergency bad-release rollback/revocation procedure from the user perspective.

Compatibility and release docs contain parts of this story, but stable v1 requires one coherent operational policy.

This is documentation/governance work unless implementation evidence exposes a product defect.

---

## V1-G5 — stale remediation PR hygiene

**State: NON-BLOCKING CLEANUP**

PR #168 remains open/draft against an old pre-Stage-2 base and is superseded by the merged R0-R8 remediation / Alpha.6 line.

It is not a product blocker, but leaving it open can mislead reviewers into believing R2 remains unresolved.

Close it as superseded after preserving a pointer to the merged evidence.

---

# Explicit non-blockers

The following are **not required for v1** under the current bounded product strategy:

- Windows support;
- ARM64 support;
- public eBPF/BPF-LSM learn/check/PASS authority;
- backend auto-selection;
- ptrace/eBPF baseline interchangeability;
- a new general variance feature;
- expanding raw research schemas merely for feature breadth;
- closing every research issue in the repository.

These remain research/expansion tracks unless a later scope decision explicitly promotes them.

---

# Minimal path to v1

The strongest execution order is:

## V1-R0 — Stable contract freeze

Do this **before product changes**.

Decide and version:

1. target-outcome semantics (#143);
2. policy v3 stable/non-stable status;
3. supported workload classes, especially build/test variance (#59);
4. supported Linux x86_64 environment floor;
5. performance evidence/claim boundary;
6. support/deprecation/rollback policy;
7. exact stable CLI/Action/schema contract.

No code feature is authorized merely by R0.

## V1-R1 — v1 release-control-plane hardening

Make the existing durable release path capable of:

- `v1.0.0` immutable tag;
- stable moving `v1` Action channel;
- stable GitHub Release (not prerelease);
- Alpha-line behavior remaining intact;
- exact stable-tag source/artifact/Action/registry proofs.

Add/verify immutable v1 tag rules before release.

## V1-R2 — external public-product evidence

In parallel, obtain one qualified current external execution/use / real-workload record against public Alpha.6.

Do not contaminate independence with live troubleshooting before the initial result is preserved.

## V1-R3 — frozen RC compatibility and operational qualification

Only after R0-R2 are sufficiently closed:

- freeze RC;
- replay P9.3;
- upgrade Alpha.6 -> RC;
- rollback;
- multi-environment consume;
- performance characterization;
- full destruction/adversarial/productization suite;
- `@v1` consumer proof.

## V1-R4 — independent critical release decision

Only then choose:

`RELEASE_V1_0`, `REWORK_REQUIRED`, or `REMAIN_ALPHA_BETA`.

---

# Current recommendation

**Do not start implementing v1 features.**

The strongest next internal action after this gap analysis is:

**V1-R0 — Stable Contract Freeze**

because several remaining blockers are decisions about what v1 promises, not missing code.

In parallel, continue the external Alpha.6 real-workload evidence path because it is an independent dependency and cannot be manufactured later by internal engineering.

The current evidence does not authorize a v1 RC yet.
