# ExecSurface — P3 V2 GCC Ephemeral Grammar Protocol

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **V2 PREREGISTERED — NO NORMALIZATION CHANGE AUTHORIZED**

## Objective

Test whether the repeated GCC temporary assembly-file drift preserved in Issue #62 can be classified by a narrow producer/role grammar without hiding meaningful paths or turning a filename pattern into blanket permission.

This gate is research-only. Public `v0.1.0-alpha.4`, canonical schema v2 and normalization profile v3 remain unchanged.

## Evidence source

Pinned workload:
`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Preserved discriminator run:
`36279728908`

Artifact:
`10918895441`

Artifact digest:
`sha256:3247cfc896efb16899c991337d374b6783d09e8ff66f2d6d489cfc5e46bb4469`

Observed baseline GCC temp identity:
`$TMP/ccVwi22P.s`

Observed check identities:
- `$TMP/ccvi9G26.s`
- `$TMP/ccbVlG4H.s`
- `$TMP/cceq5pJm.s`
- `$TMP/ccsKO93q.s`
- `$TMP/ccpk08mk.s`
- `$TMP/cchd1i4f.s`

For every preserved check, the added identity and the baseline identity were associated with:
- actor path `/usr/bin/gcc`;
- actor family `gcc`;
- a path-level `open` carrying create/write intent;
- a path-level `delete` for the same identity.

Issue #62 also preserves genuine varying Go cache/module/stdlib reads. Those are explicitly out of scope for this grammar and MUST NOT be normalized by V2.

## Frozen hypothesis

A path-only rule such as `$TMP/cc*.s` is too broad.

The bounded candidate is a **producer/role-correlated ephemeral identity** requiring all of:

1. path class is the declared semantic temp class;
2. canonical path is exactly one component under `$TMP`;
3. basename grammar is exactly `cc[A-Za-z0-9]{6}.s`;
4. actor path is exactly `/usr/bin/gcc` for this bounded experiment;
5. actor family is `gcc`;
6. the same actor/path identity has an observed create-capable open and a delete in the same trusted surface;
7. no conflicting path-level effect for the same identity is attributed to a different actor.

Only if all conditions hold may the research classifier emit the candidate identity:
`$TMP/cc<gcc-ephemeral>.s`.

This is an eligibility classification, not authorization and not a public normalization rule.

## Required positive fixtures

The classifier must accept:
- all seven identities preserved by #62 under the declared GCC create+delete role;
- input ordering permutations that preserve the same semantic effect set.

## Required negative/collision fixtures

The classifier must reject at minimum:
- matching basename outside semantic `$TMP`;
- nested path such as `$TMP/sub/ccABC123.s`;
- wrong suffix such as `.o`;
- token lengths other than six;
- non-alphanumeric token characters;
- actor family/path mismatch;
- matching filename with create but no delete;
- matching filename with delete but no create-capable open;
- same matching identity also used by a different actor;
- similar unseen filename that is not an exact grammar match.

## Falsification rule

Any negative/collision fixture classified as ephemeral kills this candidate.

The candidate also fails if real #62 identities are not classified without broadening the grammar after seeing test results.

No post-hoc widening, thresholding, wildcard cache/source normalization or actor-family-only shortcut is permitted.

## Allowed V2 outcomes

- `P3_V2_GCC_EPHEMERAL_GRAMMAR_ACCEPTED_BOUNDED`
- `P3_V2_GCC_GRAMMAR_COLLISION_RISK`
- `P3_V2_EVIDENCE_INSUFFICIENT`

Even a positive result does not modify public normalization. A later integration gate would need an observation-level transformation design, normalization-profile bump, compatibility analysis and real-workload requalification.
