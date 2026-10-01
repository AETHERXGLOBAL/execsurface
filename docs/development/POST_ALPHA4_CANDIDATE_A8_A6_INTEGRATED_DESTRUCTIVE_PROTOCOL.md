# Post-alpha.4 Candidate A8-A6 — Integrated Destructive Closeout Protocol

Status: **PREREGISTERED — NO PUBLIC RELEASE AUTHORIZED**

Candidate branch: `release/post-alpha4-candidate-assembly`

Immutable public reference: `v0.1.0-alpha.4` / `v0.1` at `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`.

## Question

Does the clean post-alpha.4 product candidate preserve the accepted A1–A5 contracts when the admitted features are exercised together, including hostile semantic, authority, attestation, variance, observer, packaging, CLI, and backward-compatibility conditions?

## Fixed roles

- Release/Integration Engineer — assembles and reproduces the candidate without importing research ancestry.
- Formal Semantics & Authority Reviewer — attacks proposition binding, ambiguity, completeness, backend authority, and schema boundaries.
- Runtime/Observer Reviewer — attacks shared-FD, PATH-TOCTOU, ptrace lifecycle, loss, and incompleteness behavior.
- Attestation/Provenance Reviewer — attacks substitution, source/baseline/current/verifier binding, and metadata laundering.
- Developer Experience / Packaging Reviewer — attacks install, package, CLI, and public Action compatibility.
- Innovation Scientist — searches for cross-layer failure modes not covered by single-gate tests; may add *stricter* attacks but may not relax an existing test or boundary.
- Anti-Drift / Scientific Integrity Reviewer — blocks scope drift, claim inflation, test weakening, deletion of negative evidence, and self-certification of P8.
- Independent Falsifier / Red Team — attempts to produce false PASS, false authority, false completeness, or unsafe migration.
- Independent Critical-Milestone Reviewer — accepts closeout only from retained executable evidence.

## Frozen invariants

1. Alpha.4 and `v0.1` remain at the immutable alpha.4 source SHA.
2. Clean candidate ancestry remains rooted at alpha.4; no merge-based import from research/promotion branches.
3. Raw observation schema v2 remains the public/default schema boundary.
4. Semantics v3 remains explicit opt-in and may not silently reinterpret v2.
5. Proposition requirements remain bound to the required proposition/subject; wrong-proposition proof cannot satisfy them.
6. Invalidating ambiguity cannot coexist with PASS-eligible completeness.
7. Empty/default proof requirements cannot authorize arbitrary v3 proof admission.
8. Backend name, collection source, frequency, recurrence, similarity, signature, signer, verifier, or attestation metadata cannot raise semantic authority by itself.
9. GCC legitimate variance remains bounded, raw-evidence preserving, and non-authoritative.
10. Shared-FD uncertainty, PATH-TOCTOU limits, observer loss, unsupported states, and malformed evidence remain fail-closed.
11. Baseline-v2 frozen digest/serialization and legacy rejection remain unchanged.
12. Public PASS/REVIEW/BLOCK/ERROR and privacy contracts remain compatible.
13. Native arm64 remains non-promoted; no support claim is inferred from internal evidence.
14. P8 external validation remains open unless qualifying independent external evidence exists; this gate cannot close P8.
15. Failed runs and counterevidence are retained. No post-hoc threshold/test/acceptance weakening is allowed.

## Frozen execution matrix

### Static/full-workspace
- `cargo fmt --all -- --check`
- default clippy with `-D warnings`
- all-features clippy with `-D warnings`
- default full-workspace tests
- all-features full-workspace tests
- release workspace build

### A1–A4 cross-layer replay
- Semantics-v3 repair corpus: 6/6
- Semantics-v3 promotion corpus: 9/9
- Authority candidate corpus: 16/16
- Attestation candidate corpus: all tests PASS
- GCC variance candidate corpus: 12/12

### Runtime adversarial replay on Ubuntu 22.04 and Ubuntu 24.04
- `m11_shared_fd_ambiguity`
- `m12_adversarial`
- `ptrace_linux`

### Public/product boundary
- frozen baseline-v2 digest vector
- legacy lock schema rejection instead of reinterpretation
- candidate package creation
- path install of candidate CLI
- CLI `--version`, `--help`, `doctor`, `learn`, PASS and controlled REVIEW behavior
- immutable alpha.4 / stable `v0.1` refs
- no arm64 public-support promotion

## Kill criteria

A8-A6 is blocked if any of the following occurs:

- any frozen test fails for a scientific/semantic reason;
- any wrong proposition/subject, ambiguity conflict, incomplete/lost evidence, backend-name, frequency/similarity, or attestation metadata obtains authority or PASS it did not previously possess;
- v2 is silently reinterpreted as v3;
- shared-FD/PATH-TOCTOU/loss behavior becomes falsely complete or PASS-eligible;
- public PASS/REVIEW/BLOCK/ERROR, package/install, or privacy compatibility regresses;
- alpha.4 or `v0.1` moves;
- arm64 is promoted without new qualifying evidence;
- P8 is internally self-certified;
- an existing test, threshold, or acceptance rule is weakened to manufacture PASS.

Harness-only failures may be corrected minimally, but the failed run must remain retained and every frozen assertion must remain unchanged or become stricter.

## Allowed decision

Only if every frozen job passes on the same candidate SHA:

`POST_ALPHA4_CANDIDATE_A8_A6_INTEGRATED_DESTRUCTIVE_PASS_BOUNDED_INTERNAL`

This decision means the assembled candidate survived the preregistered internal destructive closeout. It does **not** authorize public release, main merge, stable-tag movement, arm64 support, or P8 closure.
