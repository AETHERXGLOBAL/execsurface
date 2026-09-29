# ExecSurface — Semantics v3 S7 v2 Preservation / v3 Compatibility Result

Date: 2026-09-29
Tracking: #101
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/SEMANTICS_V3_S7_COMPATIBILITY_PROTOCOL.md`

## Formal result

**S7 CLOSED — PASS_RESTRICTED (DESIGN FREEZE)**

The compatibility constraints required for any future Semantics v3 implementation are sufficiently precise to preserve current v2 meaning and to prevent silent cross-schema authority promotion.

This is a design result, not a public schema implementation or migration authorization.

## Frozen v2 facts verified from current code/evidence

Current v2 contracts remain explicitly versioned:
- raw observation schema = `2`;
- canonical surface schema = `2`;
- baseline lock schema = `2`;
- baseline digest format = `2`;
- diff schema = `2`;
- policy/verdict schema = `2`.

The current baseline verifier rejects unsupported lock, digest, and canonical schema versions rather than attempting migration or reinterpretation.

The current v2 digest path is frozen around the v2 lock envelope and payload. Existing baseline code includes a deterministic serialization/digest vector and rejects digest corruption.

M12.2 independently preserved:
- frozen baseline-v2 serialization/digest behavior;
- rejection of legacy lock schema `1` rather than reinterpretation;
- established CLI learn/check and verdict/exit-code behavior.

Therefore S7 treats these behaviors as immutable compatibility inputs, not implementation suggestions.

## Frozen future architecture

### 1. Version-specific semantic readers

Future multi-schema support must dispatch to a version-specific parser/verifier before interpreting semantic content:

```text
bytes
  -> schema discriminator
       -> v2 parser / v2 verifier / v2 digest rules
       -> v3 parser / v3 verifier / v3 proof rules
       -> unsupported version error
```

A permissive v3 type with defaults is not an acceptable reader for v2 artifacts.

### 2. v2 is never reinterpreted as v3

Existing v2 observer capability/limitation strings are not proof-carrying proposition metadata.

Therefore:
- v2 observer names do not imply v3 authority;
- v2 canonical paths do not imply v3 kernel-object identity;
- v2 `complete=true` does not manufacture per-proposition v3 completeness records;
- absence of v3 fields in v2 is not treated as a successful default.

### 3. v3 receives distinct version domains

Any future implementation that serializes new proof semantics must use explicit new version domains for the contracts it actually changes. It must not mutate the meaning of a value already identified as schema/version 2.

The exact future integer assignments remain an implementation detail until wire types are frozen, but reuse of `2` for materially different semantics is prohibited.

### 4. v2 digest stability is permanent

The v2 digest algorithm/envelope must remain available to verify historical v2 locks exactly.

A future v3 digest may bind proposition/proof requirements, but it cannot alter the bytes that define a v2 digest.

### 5. Cross-schema default is explicit incomparability

Frozen default:

```text
v2 baseline vs v3 candidate -> INCOMPARABLE_SCHEMA
v3 baseline vs v2 candidate -> INCOMPARABLE_SCHEMA
```

No equality of canonical strings, command identity, observer names, or effect counts overrides this rule.

### 6. No default v2 -> v3 PASS-eligible projection

Because v2 canonical/baseline evidence does not retain proposition-scoped guarantee/completeness metadata, a v2 lock cannot be silently projected into v3 proof-carrying evidence capable of satisfying a v3 PASS contract.

A future behavior-only inspection projection may exist for migration UX, but it is not verification evidence.

### 7. Explicit migration must reacquire evidence where required

A future migration mechanism, if built, must:
- be explicitly invoked;
- preserve the original v2 artifact;
- produce a distinct v3 artifact;
- retain the source v2 digest only as provenance;
- require fresh admissible observation for proof metadata not present in v2;
- never claim unchanged runtime verification merely from format conversion.

No migration command is authorized by S7 itself.

## Proof-drift states frozen for future v3 diff design

Future v3 comparison must distinguish:
- behavior unchanged / proof equivalent;
- behavior unchanged / proof stronger;
- behavior unchanged / proof weaker;
- behavior unchanged / proof incomplete or ambiguous;
- proposition unsupported;
- behavior changed;
- schema incomparable.

This prevents a proof downgrade from being hidden behind identical behavioral strings.

Current v2 diff remains frozen and need not understand v3.

## Red-team classification

### R1 — deserialize v2 into v3 and default missing proof fields
**REJECTED.** Version-specific parser/verifier required; missing proof cannot default to admissible authority.

### R2 — infer v3 authority from v2 observer name
**REJECTED.** Backend/observer identity is not a guarantee set.

### R3 — equal v2/v3 canonical strings with different proof authority
**INCOMPARABLE_SCHEMA by default.** String equality is insufficient.

### R4 — migration overwrites original v2 lock
**REJECTED.** Migration must be non-destructive and explicit.

### R5 — silently compare v3 candidate to v2 baseline
**REJECTED / INCOMPARABLE_SCHEMA.** Separate schema dispatch precedes diff.

### R6 — weaker proof treated as unchanged under v3
**REJECTED.** Future proof-aware diff must surface downgrade/inadmissibility.

### R7 — stronger proof treated as ordinary behavioral drift
**CLASSIFIED SEPARATELY.** Proof-profile change and behavior change are distinct dimensions.

### R8 — unstable trace IDs included in v3 baseline digest
**REJECTED.** Run-local forensic links remain outside canonical proof equality by default.

### R9 — unsupported/legacy v2 schema accepted by future permissive reader
**REJECTED.** Historical v2 verifier semantics, including schema rejection, remain authoritative.

### R10 — fresh v3 observation failure hidden by behavior-only v2 projection
**REJECTED.** Behavior-only projection cannot satisfy v3 proof requirements or PASS.

No unclassified compatibility/authority-laundering path was found in the declared S7 design scope.

## What S7 does not prove

S7 does not prove:
- a final v3 wire layout;
- a production parser implementation;
- public migration UX;
- v3 baseline/diff runtime correctness;
- cross-schema behavioral equivalence;
- release readiness.

Those require later implementation and regression gates.

## Public authority remains unchanged

- public release remains `v0.1.0-alpha.4`;
- public raw/canonical/baseline semantics remain v2;
- ptrace remains the public/default correctness-reference backend;
- research eBPF remains non-PASS-authorized;
- no public v3 artifact or migration path exists.

## Next authorized gate

**S8 — Consolidated Independent Red-Team Gate for Semantics v3 design.**

S8 must challenge the combined S1–S7 design against the mandatory counterexamples in Issue #101 before P2 design closure or any runtime integration gate is authorized.
