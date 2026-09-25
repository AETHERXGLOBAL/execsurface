# M5 — Policy / Verdict Evidence

Date: 2026-09-25

Gate verdict: **ACCEPTED**

Branch: `milestone/m5-policy-verdict`

Issue: #10

## Result

M5 adds a policy layer strictly above M4 evidence.

Implemented:

- `execsurface-policy` crate;
- policy schema version 1;
- verdict schema version 1;
- ALLOW / REVIEW / BLOCK finding actions;
- PASS / REVIEW / BLOCK / ERROR verdict model;
- built-in REVIEW-unmatched-drift policy;
- explicit `--policy PATH`;
- retained `--diff-only`;
- stable CLI exit codes.

## Separation result

Policy does not mutate:

- the M3 baseline;
- candidate canonical surface;
- M4 added/removed/changed evidence.

No drift resolves to PASS before any default action is applied.

A custom policy with `default_action=block` cannot manufacture a BLOCK when there are zero findings.

Status: **PROVED BY IMPLEMENTATION + COMPUTATIONAL_EVIDENCE**

## Rule-resolution contract

For each finding:

1. collect all matching rules;
2. if none match, use `default_action`;
3. otherwise choose the most restrictive matched action:
   `BLOCK > REVIEW > ALLOW`.

Matched rule IDs are sorted.

The same ALLOW+BLOCK rule pair is tested in opposite file orders and produces identical BLOCK semantics.

Status: **COMPUTATIONAL_EVIDENCE**

## Matching v1

Rules can match:

- change kind;
- effect kind;
- path class;
- canonical path prefix;
- executable family;
- network IP;
- network port.

Every rule requires at least one match field.

Path prefixes use component boundaries, so `$WORKSPACE/foo` does not match `$WORKSPACE/foobar`.

Changed findings are matched against the candidate/after state while retaining complete before/after evidence.

## CLI verdict evidence

Successful CI:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36177539859

### PASS

Real learn→check of unchanged `/bin/true`:

- verdict PASS;
- exit code 0;
- no findings.

### REVIEW

Controlled extra execution under built-in policy:

- drift exists;
- unmatched findings resolve REVIEW;
- exit code 10.

### BLOCK

Explicit policy rule matching added process exec:

- matching finding resolves BLOCK;
- overall verdict BLOCK;
- exit code 20.

### ERROR

Duplicate policy rule IDs:

- policy validation fails;
- CLI exits 2;
- no policy verdict is fabricated.

## Policy engine evidence

9 unit/adversarial tests PASS:

- no drift is PASS even with BLOCK default;
- built-in policy reviews unmatched drift;
- ALLOW rule can accept matching drift;
- most restrictive rule wins independent of order;
- path-prefix component boundary;
- duplicate rule IDs rejected;
- empty matcher rejected;
- changed rule matches after state;
- explicit ERROR report state.

## Full workspace evidence

**56 tests PASS, 0 failed**

Strict gates:

- format — PASS
- strict Clippy — PASS
- locked tests — PASS
- Cargo.lock integrity — PASS

## Preserved negative evidence

### Initial M5 implementation

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36177418131

Failure: rustfmt only.

Status: **KILLED / FIXED**

### First formatted M5 build

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36177475373

Clippy found:

- one unused import;
- a large enum variant for changed evidence.

The enum was fixed with indirection rather than suppressing the lint.

Status: **KILLED / FIXED**

## Exit semantics

- PASS: 0
- ERROR: 2
- REVIEW: 10
- BLOCK: 20
- successful `--diff-only`: 0

Target process exit status remains separate evidence and does not itself become execution-surface drift policy.

## M5 non-claims

M5 does not claim its example policy is universally correct security policy.

Policy choices are explicit user/project configuration.

M5 does not implement SARIF or GitHub Action packaging.

## Gate decision

**M5 ACCEPTED.**

M6 — GitHub Action is authorized next, but is not part of this gate.
