# ExecSurface — P4-B Cross-Proposition Falsification Protocol

Date: 2026-09-30
Parent program: #100
Parent P4: #107
Parent P4-B: #109
Accepted B3 decision: `P4_B3_CONNECT_DESTINATION_PASS_BOUNDED_RESEARCH_ONLY`
Accepted B3 scientific source: `d82d1501f69658793df4737dace420c5add4be0c`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — JOINT FALSIFICATION ONLY**

## Scientific question

After B1, B2, and B3 each passed their bounded proposition-specific gates, can evidence or authority from one proposition be reused, confused, replayed, substituted, or laundered to create false success authority in another proposition or in a different causal/FD/observation context?

This gate does not seek more coverage. It tries to break composition across the three accepted proposition-specific models:

1. `P4.FILE.OPEN_OBJECT`
2. `P4.FILE.RENAME_DELETE`
3. `P4.NET.CONNECT_DESTINATION`

## Fixed roles

1. **Innovation Scientist / Evidence-Composition Architect** — seek a minimal proposition-domain separation invariant that remains composable for future heterogeneous evidence sources.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks backend scores, public-v2 reinterpretation, post-hoc assertion weakening, broad allowlists, and any inference from frequency or backend name to authority.
3. **Independent Falsifier / Red Team** — attacks evidence reuse, actor/causal substitution, FD reuse, replay, entry/exit mismatch, success laundering, attempt/pending laundering, loss masking, proposition confusion, proof-identity collision, and backend/profile-name inflation.
4. **Independent Critical-Milestone Reviewer** — verifies source SHA, exact test inventory, static gates, predecessor reproofs, artifact digest, immutable tags, retained failures, and allowed decision.

Dynamic specialists: formal evidence semantics, Rust type/API design, Linux FD/socket lifecycle, VFS object identity, deterministic serialization, cryptographic domain separation, CI/reproducibility.

## Frozen boundaries

- no changes under `crates/`;
- no public-v2 reinterpretation;
- no release/tag movement;
- no second backend implementation;
- no backend-name authority;
- no global backend score;
- no threshold reduction after execution;
- every failure remains retained;
- cross-proposition PASS cannot authorize public integration by itself.

## Frozen executable corpus

The dedicated cross-proposition harness must contain exactly **12 tests**. The acceptance threshold is **12/12 PASS** with no ignored or filtered tests.

1. **Entry-only containment** — entry-only evidence for B1/B2/B3 remains non-success even if later context objects are available.
2. **Failure containment** — matched negative-result evidence for B1/B2/B3 remains non-success.
3. **Pending containment** — connect `EINPROGRESS` remains Pending/non-success and cannot satisfy either file proposition.
4. **Actor/entry substitution** — mismatched actor or originating-entry identity becomes Ambiguous/non-success for the affected pairing.
5. **Replay containment** — duplicate/replayed pairing is rejected deterministically and cannot inflate authority.
6. **FD/object/socket reuse containment** — reused FD numbers or substituted socket/object identity cannot inherit prior authority; new generations/pairings must change proof identity.
7. **Loss containment** — incomplete or warning-bearing observation converts dependent evidence to Lost/non-success across all three propositions.
8. **Pathname/object separation** — pathname/argument identity without matched successful open result and post-exit object binding cannot become `P4.FILE.OPEN_OBJECT` success.
9. **Cross-proposition success rejection** — a valid success record for any one of B1/B2/B3 must be rejected by builders for the other propositions.
10. **Deterministic proof reconstruction** — identical evidence reconstructed twice yields byte-identical records and identical proof digests for B1/B2/B3.
11. **Causal substitution containment** — causal-chain substitution cannot preserve successful object authority; entry/exit causal disagreement must fail closed.
12. **Proposition-domain separation** — otherwise similar attempts from different target propositions/operations cannot share the same pairing identity; proof identity must remain proposition-scoped.

## Mandatory anti-inflation reproof

The accepted workflow must additionally execute the existing canonical A0 test:

`a0_backend_name_never_upgrades_weak_evidence`

This is not counted among the 12 dedicated cross tests. It proves that arbitrary backend/profile naming cannot upgrade weak evidence.

## Mandatory same-source reproof

On the accepted source SHA, the workflow must also re-prove at minimum:

- B0: **18/18 PASS**
- B1 model: **24/24 PASS**
- B1 live: **10/10 PASS**
- B2 model: **23/23 PASS**
- B2 live: **10/10 PASS**
- B3 model: **20/20 PASS**
- B3 live: **10/10 PASS**
- A1 adversarial: **18/18 PASS**
- A1.2 mapping: **10/10 PASS**
- Semantics v3: **7/7 PASS**
- public M11: **6/6 PASS**
- immutable `v0.1.0-alpha.4` and `v0.1` source boundary.

## Kill criteria

Any path that yields false success, cross-proposition evidence reuse, replay inflation, pending/failure/lost -> success laundering, actor/entry/causal substitution success, FD reuse authority transfer, proposition-domain proof collision, or backend-name authority inflation closes the affected gate as failed. Such a counterexample must be retained and must not be patched by weakening the corpus.

## Allowed decisions

- `P4_B_PTRACE_SUCCESS_AUTHORITY_ESTABLISHED_BOUNDED`
- `P4_B_PARTIAL_PTRACE_GAPS_REMAIN`
- `P4_B_FALSE_AUTHORITY_PATH_FOUND`
- `P4_B_INCOMPLETE_EVIDENCE`

`P4_B_SECOND_BACKEND_PROTOTYPE_JUSTIFIED_BOUNDED` is allowed only if a concrete proposition-specific missing-evidence gap remains after the frozen ptrace gate. A cross-proposition implementation defect or test failure does not itself justify another backend.

No outcome here authorizes public integration, release promotion, raw-v2 reinterpretation, or backend interchangeability.