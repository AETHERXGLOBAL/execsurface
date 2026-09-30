# ExecSurface — P4-B Cross-Proposition Falsification Result

Date: 2026-09-30
Parent program: #100
Parent P4: #107
P4-B issue: #109
Protocol: `docs/development/P4_B_TARGETED_SUCCESS_AUTHORITY_PROTOCOL.md`
Cross protocol: `docs/development/P4_B_CROSS_PROPOSITION_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

`P4_B_PTRACE_SUCCESS_AUTHORITY_ESTABLISHED_BOUNDED`

Boundary:

`NO_SECOND_BACKEND_JUSTIFIED_BY_THE_TESTED_P4_B_GAPS`

`NO_PUBLIC_INTEGRATION_OR_RELEASE_PROMOTION_AUTHORIZED`

Within the frozen Linux x86_64 research scope, targeted ptrace entry/result/state evidence now closes the three A2 success-authority gaps tested by P4-B:

1. `P4.FILE.OPEN_OBJECT`
2. `P4.FILE.RENAME_DELETE`
3. `P4.NET.CONNECT_DESTINATION`

The result is proposition-scoped, not backend-global. It does not imply universal ptrace completeness, backend interchangeability, public-v3 readiness, or superiority over other collectors.

## Accepted source and executable evidence

Accepted source SHA:

`e471075e460e86b70e4a3d191b83de4f9be02aac`

Cross-proposition workflow:

- run `36735679528`
- job `109956619318`
- runner: GitHub-hosted Ubuntu 24.04 / Linux x86_64
- Rust: `1.90.0`
- conclusion: `success`
- static gates: rustfmt PASS; clippy `-D warnings` PASS
- frozen cross-proposition corpus: **12/12 PASS**
- backend-name anti-inflation reproof: **1/1 PASS**
- artifact: `11106972286`
- artifact SHA-256: `d64423dbf3ef1f06496f2847f9dc7efcd719d9f975471f1dfb3a963a92f119b8`

Frozen-boundary verification on the accepted run established:

- accepted B3 source `d82d1501f69658793df4737dace420c5add4be0c` remains an ancestor;
- `v0.1.0-alpha.4^{commit}` remains `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- `v0.1^{commit}` remains `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no changes under `crates/` occurred between accepted B3 and the accepted cross-gate source.

## Joint falsification result

All preregistered attacks passed fail-closed behavior:

1. entry-only evidence did not become success across B1/B2/B3;
2. negative-result evidence remained non-success;
3. connect `EINPROGRESS` remained Pending and could not satisfy file propositions;
4. actor and originating-entry substitution failed closed;
5. duplicate/replayed pairing was rejected;
6. FD/object/socket reuse required new identity and could not inherit prior authority;
7. incomplete/warning-bearing observation produced Lost/non-success;
8. pathname/binding presence without a successful open exit did not create object success;
9. success evidence for one proposition was rejected by the other proposition builders;
10. identical evidence rebuilt byte-identically with deterministic proof digests;
11. causal-chain substitution failed closed;
12. otherwise similar attempts remained domain-separated by proposition and operation.

The separate A0 anti-inflation test also re-proved that backend/profile naming cannot upgrade weak evidence.

## Same-source predecessor reproof

The accepted workflow re-proved on `e471075e460e86b70e4a3d191b83de4f9be02aac`:

- B0 success-evidence contract: **18/18 PASS**
- B1 model: **24/24 PASS**
- B1 live Linux ptrace: **10/10 PASS**
- B2 model: **23/23 PASS**
- B2 live Linux ptrace: **10/10 PASS**
- B3 model: **20/20 PASS**
- B3 live Linux ptrace: **10/10 PASS**
- A2 matrix: **16/16 PASS**
- A1.3U: **5/5 PASS**
- A1 adversarial: **18/18 PASS**
- A1.2 mapping: **10/10 PASS**
- Semantics v3: **7/7 PASS**
- public M11 shared-FD fail-closed contract: **6/6 PASS**

The historical A2 matrix remains preserved as the prerequalification statement of what raw-v2 alone lacked. P4-B does not rewrite that history; B1/B2/B3 are research-only strengthening evidence layered after A2.

## Retained failure history

No failed attempt was removed or rewritten.

The first cross-proposition execution attempt stopped before scientific execution at rustfmt only:

- run `36735166881`
- job `109954842041`
- boundary verification PASS
- cross corpus: not executed
- predecessor reproof: not executed
- artifact `11106357849`
- artifact SHA-256 `0cd1797bff0034f1a24e2ac0b1e7f5058e310149bbed8867336cfdcd9c56f407`

A pre-existing A0/A1 workflow triggered by the same harness addition independently stopped at the same formatting condition:

- run `36734791850`
- job `109953539307`
- artifact `11107145756`
- artifact SHA-256 `c871532a40246409b5cedb9f0c8f6697ddecb64b9d0c9d0c9723c88a0828e624`

The correction was rustfmt-equivalent only. The frozen 12-test inventory, assertions, thresholds, authority rules, and proposition boundaries were unchanged.

All earlier B1/B2/B3 negative and engineering evidence remains part of the repository ledger, including B3 lifecycle lint, 9/10 loopback fixture failure, harness-adequacy finding, formatting failures, and target-reachability lint failure.

## Scientific interpretation

The P4-B question was whether targeted ptrace evidence could close the three A2 success-authority gaps before introducing another runtime backend. Under the bounded tested environment, the answer is supported: all three proposition-specific gates and the joint falsification gate pass while preserving failure, pending, ambiguity, loss, replay, substitution, FD-generation, and proposition-domain separation.

Therefore a second backend is **not justified by these three tested gaps**. This is not a claim that another evidence source can never add value. A future second source must be justified by a newly demonstrated proposition-specific evidence deficiency, portability requirement with its own evidence contract, or interoperability requirement—not by backend diversity for its own sake.

## Next authorized P4 work

Proceed to the P4-C external trace import experiment defined by the parent P4 protocol.

P4-C must test interoperability while preserving ExecSurface authority semantics. Imported trace data is evidence input only; external producer/backend names do not grant authority. Unsupported, incomplete, ambiguous, unmappable, or provenance-deficient imported evidence must fail closed.

No public integration, `main` modification, release/tag movement, raw-v2 reinterpretation, or backend promotion is authorized by this result.