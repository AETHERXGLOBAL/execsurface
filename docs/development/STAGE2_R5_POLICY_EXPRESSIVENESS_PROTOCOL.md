# ExecSurface — Stage-2 R5 Policy Expressiveness Protocol

Date: 2026-10-08
Parent remediation: #165 / PR #166
Predecessor: R4 PASS_BOUNDED
Status: **PREREGISTERED — FAIL-FIRST REQUIRED**

## Objective

R5 addresses one bounded policy-language gap: canonical evidence already carries distinctions that the current rule matcher cannot express.

The gate asks:

> Can policy rules match the existing canonical semantics that materially distinguish file-open intent, path-resolution authority, and the source side of a rename, without changing the meanings of existing policy schema v1/v2 constructs?

This gate changes policy expressiveness only. It does not change observation, normalization, diff construction, baseline custody, or verdict precedence.

## Fixed review roles

1. **Innovation / Architecture** — select the smallest matcher model that maps directly to canonical evidence.
2. **Anti-Drift / Goal Alignment** — prevent policy convenience from changing evidence meaning or reopening R2/R3/R4.
3. **Independent Destruction / Falsification** — own semantic-collision, over-match, under-match and schema-compatibility attacks.
4. **Independent Critical Reviewer** — require fail-first evidence, backward-compatibility evidence and explicit schema boundaries before R5 may close.

## Current reproduced design gap

The current matcher exposes:

- change;
- effect;
- primary path class/prefix;
- executable family;
- network IP/port.

But canonical effects additionally carry:

- `FilePathAccess.open_intent`;
- `CanonicalPath.resolution`;
- `FileRename.from` and `FileRename.to`.

The current `EffectMetadata::from_effect` projects only `FileRename.to` as the path visible to policy. Therefore two rename effects with the same destination but materially different sources are policy-indistinguishable.

## Schema decision

R5 MUST NOT silently reinterpret policy schema v1 or v2.

Selected boundary:

- schema v1 remains legacy;
- schema v2 remains the current public/default policy schema;
- schema v3 is opt-in for the new matcher fields;
- the built-in/default/generated policy remains v2 during R5;
- old v1/v2 policies must retain their current meanings;
- v1/v2 policies using any v3-only field must be rejected explicitly;
- no automatic policy migration is introduced.

The implementation may introduce a dedicated v3 constant, but MUST NOT repurpose the existing v2 constant as though public history had changed.

## Candidate v3 matcher surface

### 1. Primary path resolution

`path_resolution`: optional exact match against the resolution of the same primary path already used by `path_class` / `path_prefix`.

For `FilePathAccess`, primary path = target.
For `FileRename`, primary path remains destination (`to`) to preserve existing v1/v2 semantics.
For path-bearing process/unix-network effects, it follows their existing primary path.

### 2. Open intent

`open_intent`: optional partial matcher valid only when the canonical effect has an `OpenIntent`.

Candidate fields are independently optional:

- `read`;
- `write`;
- `create`;
- `truncate`;
- `append`;
- `path_only`;
- `resolve_flags`;
- `other_flags`.

A supplied field is an exact constraint. Unspecified fields are wildcards.

An empty `open_intent: {}` matcher is invalid.

If `effect` is explicitly specified, `open_intent` requires `effect=file_open`.

### 3. Rename source

The existing primary path continues to mean rename destination for compatibility.

Schema v3 adds independent source-side constraints:

- `rename_from_class`;
- `rename_from_prefix`;
- `rename_from_resolution`.

If `effect` is explicitly specified, any rename-source matcher requires `effect=file_rename`.

An empty `rename_from_prefix` is invalid.

## Frozen fail-first acceptance corpus

Before implementation, tests must demonstrate that current source cannot satisfy these v3 policies:

1. **Open-intent separation**
   - same path/effect;
   - `truncate=true` must be blockable while `truncate=false` remains allowed.

2. **Path-resolution separation**
   - same file-read path/class;
   - `kernel_fd_resolved` must be distinguishable from `lexical`.

3. **Rename-source separation**
   - same rename destination;
   - source under a blocked prefix must be blockable while a different source remains allowed.

4. **Schema anti-drift**
   - v1/v2 semantics remain unchanged;
   - v3-only fields under v1/v2 are rejected, not ignored.

5. **Most-restrictive-wins remains unchanged**
   - the new fields only affect whether a rule matches; they do not alter rule precedence.

Tests may be expanded after new counterexamples, but these invariants may not be weakened to obtain green.

## Kill conditions

R5 must stop rather than ship a matcher if any implementation:

- infers information not present in canonical evidence;
- rewrites rename destination semantics for existing v1/v2 rules;
- treats lexical and kernel-resolved paths as equivalent when v3 explicitly constrains resolution;
- turns absent `open_intent` into a synthetic all-false intent;
- lets an empty nested matcher match everything;
- silently accepts v3-only fields under v1/v2;
- changes verdict precedence or baseline/diff semantics;
- requires an observation/normalization schema change to satisfy the policy feature.

## R5 close condition

R5 may become GREEN only when:

- fail-first evidence is retained;
- all three semantic distinctions are proved by positive and negative controls;
- v1/v2 compatibility tests remain green;
- malformed/inapplicable v3 matchers fail explicitly;
- full CI, adversarial regression, P9.3 compatibility and packaging remain green;
- independent destruction finds no policy over-match that collapses the intended distinctions.

No merge or release is authorized by R5 alone.
