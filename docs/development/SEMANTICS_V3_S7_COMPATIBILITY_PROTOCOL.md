# ExecSurface — Semantics v3 S7 v2 Preservation / v3 Compatibility Protocol

Date: 2026-09-29
Tracking: #101
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Predecessor: `SEMANTICS_V3_S6_SUPPORT_MATRIX_RESULT.md` — **PASS_RESTRICTED**
Status: **PREREGISTERED BEFORE S7 DESIGN FREEZE**

## Question

Can ExecSurface introduce a future Semantics v3 contract while guaranteeing that existing v2 evidence and baseline lockfiles retain their exact historical meaning, digest, verification behavior, and fail-closed boundaries?

## Current frozen v2 facts

The current development source inherits the published v2 contracts:

- raw observation schema: `2`;
- canonical surface schema: `2`;
- baseline lock schema: `2`;
- baseline digest format: `2`;
- diff schema: `2`;
- policy/verdict schema: `2`;
- canonical normalization profile is separately versioned.

Current v2 baseline verification rejects unsupported lock, digest, and canonical schema versions rather than reinterpreting them.

Current v2 diff comparability also gates on explicit platform, observer capability/identity, canonical schema, and normalization-profile compatibility before comparing effects.

## Immutable preservation rules

S7 MUST preserve:

1. **v2 bytes mean v2 forever.** No future parser may assign v3 proof semantics to an existing v2 field merely because a similar v3 proposition exists.
2. **v2 digest stability.** Existing valid v2 baseline digest vectors must remain byte-for-byte verifiable under the frozen v2 digest algorithm.
3. **no silent lock rewrite.** Reading/checking a v2 lock must not mutate it to v3.
4. **no silent candidate upgrade.** A v2 baseline cannot become v3-comparable merely because the current runtime happens to produce richer evidence.
5. **explicit schema dispatch.** Future multi-schema readers must dispatch to a version-specific verifier before semantic interpretation.
6. **cross-schema default = incomparable.** v2↔v3 verification comparison is rejected unless a separately versioned, proved projection explicitly authorizes the requested proposition set.
7. **unknown proof authority is not weak authority.** Missing v3 proof metadata in v2 must not be guessed from observer names/capability strings.
8. **public alpha.4 remains unchanged.** S7 is design-only on the isolated branch.

## S7-A — Version-domain separation

Freeze separate version domains for future v3 rather than overloading existing constants.

Candidate future domains:
- raw observation schema v3;
- canonical/proof surface schema v3;
- baseline lock schema v3;
- digest format v3;
- diff/report schema v3 where proof-drift representation requires it;
- normalization profile only when canonicalization rules actually change.

A version number is changed only when its serialized/semantic contract changes.

## S7-B — Version-specific verification architecture

Future reader architecture must conceptually dispatch as:

```text
bytes
  -> minimal schema discriminator
      -> V2 parser + V2 verifier + V2 digest semantics
      -> V3 parser + V3 verifier + V3 digest/proof semantics
      -> unsupported schema error
```

The discriminator must not deserialize a v2 document into a permissive v3 structure and then infer missing fields.

## S7-C — Frozen v2 baseline verification

Required preservation tests for any later implementation:

1. accepted frozen v2 baseline vector verifies with the same digest;
2. corrupted v2 digest still fails;
3. v2 unsupported lock schema still fails rather than migrating;
4. v2 unsupported digest format still fails;
5. v2 unsupported canonical schema still fails;
6. v2 ordering normalization remains identical;
7. unknown/extra-field handling must not create a path that changes v2 meaning silently.

No v3 implementation may modify the existing v2 digest envelope algorithm.

## S7-D — Future v3 baseline contract

A future v3 baseline must represent both:

- accepted canonical behavior;
- accepted minimum proof/authority contract for each proposition family/effect.

Candidate v3 digest input must bind, deterministically and explicitly:
- v3 lock/digest schema identifiers;
- canonical behavior/proposition values;
- canonical proof profile / required guarantee set;
- required completeness dimensions;
- stable ambiguity class codes where baseline-relevant;
- backend semantic-profile requirement only where semantically necessary, not transient run identity.

Run-local TIDs, timestamps, event sequence numbers, workflow run IDs, transport IDs, and forensic links are excluded from semantic equality unless a later versioned rule proves otherwise.

## S7-E — Cross-schema comparison rule

Default rule:

```text
v2 baseline vs v3 candidate -> INCOMPARABLE_SCHEMA
v3 baseline vs v2 candidate -> INCOMPARABLE_SCHEMA
```

A future projection may be introduced only if all of the following are proved:

1. projection has its own immutable version/identifier;
2. source schema meaning is preserved;
3. target proposition does not claim authority absent from source evidence;
4. completeness dependencies can be represented without guessing;
5. behavior and proof requirements are both satisfied for the requested comparison;
6. counterexamples are preregistered and retained.

### Initial S7 decision on v2 -> v3 proof projection

Because v2 canonical effects do not retain proposition-scoped proof metadata, **v2 must not be projected into v3 as PASS-eligible proof-carrying evidence by default**.

A v2 lock may later have a behavior-only inspection projection for migration tooling, but that projection cannot satisfy v3 proof requirements or produce a verification PASS without separately reacquired/admissible evidence.

## S7-F — Explicit migration concept

If a future migration command is created, it must be user-explicit and non-destructive.

Allowed conceptual behavior:

```text
execsurface migrate --from v2 --to v3 ...
```

must:
- preserve original v2 lock unchanged;
- require fresh observation when v3 proof data is required;
- create a new v3 artifact rather than rewrite the v2 lock in place by default;
- record source v2 digest as provenance, not as proof equivalence;
- never label migration as verification of unchanged runtime behavior unless a fresh admissible check proves it.

No migration CLI is authorized by S7 itself.

## S7-G — Proof-drift compatibility

Future v3 diff must be capable of distinguishing:
- behavior unchanged / proof stronger;
- behavior unchanged / proof weaker;
- behavior unchanged / proof ambiguous/incomplete;
- behavior changed;
- proposition unsupported;
- schema incomparable.

Current v2 diff remains frozen and need not understand v3.

## Mandatory red-team cases

1. v2 lock deserialized into a v3 type with missing proof fields defaulted to strong values;
2. v2 observer name used to infer v3 authority;
3. v2 and v3 canonical strings equal but proof guarantees differ;
4. migration rewriting the original v2 lock/digest;
5. v3 candidate silently compared to v2 baseline because behavior strings match;
6. weaker proof accepted as unchanged under a v3 baseline;
7. stronger proof incorrectly treated as behavioral drift rather than proof-profile change;
8. unstable trace IDs included in v3 baseline digest;
9. old invalid/unsupported v2 schema accepted by a permissive future parser;
10. fresh v3 observation failure hidden by a behavior-only v2 projection.

## Acceptance

S7 design can close `PASS_RESTRICTED` only if:

1. v2 semantic/digest preservation rules are explicit and testable;
2. future v3 version domains are separate and explicit;
3. cross-schema default incomparability is frozen;
4. no default v2 -> v3 PASS-eligible proof projection exists;
5. explicit non-destructive migration rules are frozen;
6. proof-drift states are specified;
7. all red-team cases have a fail-closed classification;
8. no public alpha.4 code/schema behavior is changed.

If these requirements cannot be met without weakening v2 compatibility or inventing v3 authority, record `S7_COMPATIBILITY_DESIGN_INSUFFICIENT` and stop before S8/runtime integration.

## Non-claim

S7 does not authorize a v3 wire schema, public migration command, new baseline format, runtime integration, or release. It freezes the compatibility constraints that any later implementation must satisfy.
