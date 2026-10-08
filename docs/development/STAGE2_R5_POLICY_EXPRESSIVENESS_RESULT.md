# ExecSurface — Stage-2 R5 Policy Expressiveness Result

Date: 2026-10-08  
Parent remediation: #165 / PR #166  
Protocol: `docs/development/STAGE2_R5_POLICY_EXPRESSIVENESS_PROTOCOL.md`  
Qualified source: `d01e7ba485982cc3101525ae6cec210ad5449903`  
Decision: **R5_POLICY_EXPRESSIVENESS_PASS_BOUNDED**

## Decision

R5 is GREEN for the preregistered bounded proposition:

> Policy schema v3 can opt into exact matching over existing canonical evidence for primary path resolution, file-open intent, and rename-source identity, while policy schemas v1/v2 retain their previous meanings.

No observer, normalization, diff, baseline-custody, verdict-precedence, or release semantics were changed by R5.

## Implemented boundary

The public/default policy version remains schema v2. R5 adds an explicit opt-in schema v3 constant and does not repurpose v1/v2 history.

Schema v3 adds:

- `path_resolution` on the existing primary path;
- `open_intent` partial exact matcher over:
  - `read`
  - `write`
  - `create`
  - `truncate`
  - `append`
  - `path_only`
  - `resolve_flags`
  - `other_flags`;
- `rename_from_class`;
- `rename_from_prefix`;
- `rename_from_resolution`.

Existing rename primary-path behavior remains destination (`to`) for v1/v2 compatibility.

## Controlled fail-first replay

The original fail-first commit was initially prevented from reaching semantic execution by a lockfile dependency mismatch. That was not accepted as sufficient falsification evidence.

A controlled replay was therefore performed on the same remediation branch:

- replay commit: `9965f00c4d26d203adf30f891c2380c40181f870`;
- pre-fix policy matcher source restored exactly from `f8baf3a7b87230af3fb52790672cb91be969253d`;
- current R5 acceptance corpus and corrected lockfile retained;
- formatting and clippy passed before tests;
- CI run `37713772787` reached `r5_policy_expressiveness.rs`;
- result: **2 PASS / 12 FAIL**.

The failures were semantic and matched the preregistered gap:

- `open_intent` rejected as an unknown matcher field;
- `path_resolution` rejected as an unknown matcher field;
- `rename_from_prefix` rejected as an unknown matcher field.

The two controls that did not require new v3 expressiveness remained green:

- v2 rename primary-path semantics stayed destination-based;
- unknown matcher fields remained parse errors.

No failure was rewritten or deleted.

## Final hostile corpus

At qualified source `d01e7ba485982cc3101525ae6cec210ad5449903`:

`r5_policy_expressiveness.rs`: **14/14 PASS**

Coverage includes:

1. truncate vs non-truncate open-intent separation;
2. lexical vs kernel-fd-resolved path separation;
3. same rename destination separated by source prefix;
4. explicit rejection of v3-only fields under v1/v2;
5. empty `open_intent` rejection;
6. inapplicable `open_intent`/effect combination rejection;
7. most-restrictive-wins unchanged;
8. absent open intent is not synthesized as an all-false intent;
9. rename source prefixes preserve component boundaries;
10. v2 rename primary-path semantics remain destination-based;
11. INET effects do not acquire invented path-resolution metadata;
12. open-intent flag constraints are exact;
13. empty rename-source prefix rejection;
14. unknown v3 matcher fields remain parse errors.

## Qualification workflows

All required and adjacent workflows passed against the same qualified source:

- CI — run `37713855429` — SUCCESS
- Adversarial Regression — run `37713855415` — SUCCESS
- P9.3 Compatibility Contract — run `37713855414` — SUCCESS
- Registry Packaging Gate — run `37713855506` — SUCCESS
- Public Consumer Smoke — run `37713855416` — SUCCESS
- P8 A3.4 consumer contract red team — run `37713855468` — SUCCESS
- P8 A3 typed report prototype — run `37713855431` — SUCCESS
- P8 A3 typed evidence output — run `37713855440` — SUCCESS
- Ptrace Lifecycle Regression — run `37713855479` — SUCCESS

## Anti-drift decision

- Alpha.5 remains immutable historical evidence.
- Policy schemas v1/v2 are not silently reinterpreted.
- The built-in/default/generated policy remains v2 during R5.
- R5 does not infer evidence absent from the canonical surface.
- Verdict precedence is unchanged.
- No merge or release is authorized by R5.
- R6 CI hardening may now open.
