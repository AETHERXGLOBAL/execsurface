# ExecSurface — P3 V2 GCC Ephemeral Grammar Result

Date: 2026-09-29
Tracking: #103
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`

## Formal decision

`P3_V2_GCC_EPHEMERAL_GRAMMAR_ACCEPTED_BOUNDED`

This is a research-only classification result. It does **not** change public `v0.1.0-alpha.4`, canonical schema v2, public normalization profile v3, baseline semantics, policy, or stable `@v0.1`.

## Evidence basis

Preserved real-workload source:
- Issue #62
- `junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`
- discriminator run `36279728908`
- artifact `10918895441`
- digest `sha256:3247cfc896efb16899c991337d374b6783d09e8ff66f2d6d489cfc5e46bb4469`

Preserved GCC temporary assembly identities:
- `$TMP/ccVwi22P.s`
- `$TMP/ccvi9G26.s`
- `$TMP/ccbVlG4H.s`
- `$TMP/cceq5pJm.s`
- `$TMP/ccsKO93q.s`
- `$TMP/ccpk08mk.s`
- `$TMP/cchd1i4f.s`

The preserved evidence associated these identities with `/usr/bin/gcc`, family `gcc`, create/write-capable open, and delete behavior. Genuine varying Go cache/module/stdlib reads remain distinct evidence and are not normalized by this gate.

## Protocol

Preregistered protocol:
`docs/development/P3_V2_GCC_EPHEMERAL_GRAMMAR_PROTOCOL.md`

Protocol commit:
`9cdd75b3871719b0eaa4aca310647e7d614727ea`

The protocol explicitly rejected a path-only `$TMP/cc*.s` rule as too broad.

## Accepted bounded classifier

Accepted source:
`1fb3be69b66a84f1bb5ce42e93dc2a876cea70dd`

Candidate eligibility requires all of:
1. semantic `Temp` path class;
2. exactly one component under `$TMP`;
3. exact basename grammar `cc[A-Za-z0-9]{6}.s`;
4. actor path exactly `/usr/bin/gcc` in this bounded experiment;
5. actor family exactly `gcc`;
6. the same path identity has an observed create+write-capable open and delete;
7. execution-chain tail matches the actor;
8. no conflicting matching-path use by another actor or incompatible path operation;
9. matching rename evidence is treated as conflict rather than collapsed.

Only eligible research effects are projected to:
`$TMP/cc<gcc-ephemeral>.s`

This token is not public authorization and not a public normalization rule.

## Accepted CI

Run:
`36557953004`

Job:
`109371569116`

Environment:
Ubuntu 24.04.5, GitHub-hosted runner.

Result:
- `cargo fmt ... --check` — PASS
- `cargo clippy ... --all-targets -- -D warnings` — PASS
- `cargo test ... --all-targets` — PASS
- unit tests: **7 passed / 0 failed**

The accepted suite covers:
- all seven preserved Issue #62 GCC identities;
- missing create/delete role rejection;
- read-only-open rejection;
- second-actor collision rejection;
- wrong actor path/family rejection;
- root/nesting/token/suffix/case collision rejection;
- parent-traversal rejection;
- preservation of existing Go build-root normalization as a distinct rule.

## Preserved failure

Run `36557762900` failed at formatting only before clippy/tests. The failure is retained. The correction commit `1fb3be69b66a84f1bb5ce42e93dc2a876cea70dd` applied only the rustfmt-prescribed formatting changes; no grammar, collision rule, threshold or test was relaxed.

## Scientific interpretation

The evidence supports a bounded conclusion: the stable GCC assembly-file noise preserved by #62 can be recognized more safely as a **producer/role-correlated ephemeral identity** rather than as a filename wildcard.

This result does not show that every `ccXXXXXX.s` file is ephemeral, does not authorize ignoring GCC activity, and does not solve the genuine varying Go cache/module/stdlib effect-set problem. Those remain inputs to the multi-run accepted-variance model.

## Next gate

V3 — explicit accepted-variance contract.

V3 must preserve the distinction between:
- invariant behavior;
- merely observed variable behavior;
- explicitly accepted variable behavior;
- unseen behavior.

Frequency or repeated observation must never become automatic authorization. Every accepted variable effect must remain bound to learning-run provenance and compatibility constraints.
