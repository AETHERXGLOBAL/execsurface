# ExecSurface — P3 V2 Bounded GCC Temporary Assembly Grammar Protocol

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Predecessor: `P3_V1_VARIANCE_ANALYZER_RESULT.md` — **PASS_RESEARCH_ONLY**
Status: **PREREGISTERED BEFORE NORMALIZER CHANGE**

## Question

Can ExecSurface remove the stable GCC temporary-assembly identity churn observed in #62 using a narrow, evidence-backed temp-name grammar without hiding changes in actor, operation, execution chain, root, suffix, or non-conforming filename?

## Empirical evidence frozen before implementation

Preserved discriminator artifact:
- workflow run `36279728908`;
- artifact `10918895441`;
- artifact digest `sha256:3247cfc896efb16899c991337d374b6783d09e8ff66f2d6d489cfc5e46bb4469`.

Across the learned baseline plus six consecutive checks, the GCC paths observed were:

- `$TMP/ccVwi22P.s` — baseline identity;
- `$TMP/ccvi9G26.s`;
- `$TMP/ccbVlG4H.s`;
- `$TMP/cceq5pJm.s`;
- `$TMP/ccsKO93q.s`;
- `$TMP/ccpk08mk.s`;
- `$TMP/cchd1i4f.s`.

Every observed variable basename is exactly:

```text
cc + 6 ASCII alphanumeric characters + .s
```

For each check the four GCC drift findings were:
- added `file_open` on the new temp identity;
- added `file_delete` on the new temp identity;
- removed `file_open` on the baseline temp identity;
- removed `file_delete` on the baseline temp identity.

The actor remained `/usr/bin/gcc`, the execution chain remained `bash -> go -> gcc`, and the open intent/operation semantics were otherwise unchanged.

## External producer corroboration

GCC/libiberty documentation describes temporary-name generation from patterns of the form `path/ccXXXXXXsuffix`, where the six `X` positions are replaced to make the name unique. GCC's spec documentation also describes `%g` temporary names and gives `cc... .s` examples.

These external facts corroborate the preserved artifact; they do not broaden this gate beyond the declared grammar.

## Candidate grammar

A path is a V2 GCC-temp candidate only when all of the following are true:

1. canonical path class is `temp`;
2. the tokenized path is a **direct child** of `$TMP`;
3. basename length is exactly 10 bytes: `ccXXXXXX.s`;
4. bytes 3–8 are ASCII alphanumeric only;
5. prefix is exactly lowercase `cc`;
6. suffix is exactly lowercase `.s`.

Candidate canonical identity:

```text
$TMP/cc<ephemeral>.s
```

## Important semantic boundary

The candidate normalizes only the path identity.

ExecSurface canonical effects continue to retain independently:
- actor executable;
- execution chain;
- operation (`open`, `delete`, etc.);
- open intent;
- path resolution/class.

Therefore two effects with the same normalized GCC-temp path but different actors, chains, operations, or intents remain different canonical effects.

## Required positive fixtures

Must normalize:
- every preserved #62 GCC path listed above;
- additional synthetically varied six-character ASCII alphanumeric names under the declared `$TMP` root.

## Required negative/collision fixtures

Must **not** normalize:
- `/workspace/ccABC123.s`;
- `$HOME/ccABC123.s`;
- `$TMP/sub/ccABC123.s`;
- `$TMP/ccABC12.s` (5 variable chars);
- `$TMP/ccABC1234.s` (7 variable chars);
- `$TMP/ccABC-23.s`;
- `$TMP/CCABC123.s`;
- `$TMP/ccABC123.S`;
- `$TMP/ccABC123.o`;
- `$TMP/ccABC123.s.extra`;
- parent-traversal variants;
- neighboring non-GCC temp names.

## Actor/operation laundering fixtures

After normalization:
- gcc `open` != gcc `delete`;
- gcc `open` != another actor `open`;
- different execution chains remain distinct;
- different open intents remain distinct.

The grammar therefore cannot be accepted by path-only unit tests alone; canonical-effect tests must prove these dimensions survive.

## Profile/version rule

If the rule is integrated into the development normalizer, it changes canonical equality and MUST bump the normalization profile from `3` to `4`.

Existing profile-3 artifacts remain profile 3 and are not silently reinterpreted. Cross-profile comparison remains fail-closed under current comparability rules.

## Prohibited shortcuts

- no generic `$TMP/cc*` normalization;
- no wildcard suffix;
- no recursive/subdirectory normalization;
- no cache/module/source normalization;
- no actor removal;
- no operation/intention collapse;
- no threshold change;
- no claim that all GCC versions/platforms use this grammar universally.

## Acceptance

V2 may close `PASS_RESTRICTED` only if:
1. all preserved #62 GCC identities collapse to one candidate identity;
2. all declared negative/collision fixtures remain distinct;
3. actor/operation/chain/intent distinctions survive;
4. normalization profile is bumped if integrated;
5. existing Go temp-root normalization still passes;
6. full normalizer fmt/clippy/tests pass;
7. public alpha.4 remains unchanged.

A V2 pass authorizes only V5-style real-workload requalification of the bounded candidate. It does not by itself authorize public release.
