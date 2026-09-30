# ExecSurface — P4-B3 `P4.NET.CONNECT_DESTINATION` Result

Date: 2026-09-30
Parent program: #100
Parent P4: #107
Parent P4-B: #109
Protocol: `docs/development/P4_B3_CONNECT_DESTINATION_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

`P4_B3_CONNECT_DESTINATION_PASS_BOUNDED_RESEARCH_ONLY`

Boundary:

`NO_PUBLIC_INTEGRATION_OR_SECOND_BACKEND_AUTHORIZED`

Within the frozen research scope, targeted Linux x86_64 ptrace connect-entry/syscall-exit evidence established bounded synchronous-success authority for `P4.NET.CONNECT_DESTINATION` only when exact actor, originating entry, socket FD, destination context, matched exit, healthy observation, and `rc=0` are jointly present.

`-EINPROGRESS` remains Pending. Negative errno remains Failure. Malformed or incomplete sockaddr evidence remains non-authoritative. Later socket state or I/O does not relabel the original connect result.

This result does not alter public raw-v2 bytes or meaning, the default observer, `main`, `v0.1.0-alpha.4`, stable `v0.1`, or release artifacts.

## Accepted source and live evidence

Accepted source:

`d82d1501f69658793df4737dace420c5add4be0c`

Mandatory B3 workflow:

- workflow run `36733292944`
- job `109948295436`
- runner: GitHub-hosted Ubuntu 24.04 / Linux x86_64
- Rust toolchain: `1.90.0`
- conclusion: `success`
- static gates: rustfmt PASS; clippy `-D warnings` PASS
- B3 live ptrace corpus: **10/10 PASS**
- B3 frozen model: **20/20 PASS**
- evidence artifact: `11105911160`
- artifact SHA-256: `af9b5b64e5b849972520e44001f9285a0decf8b60a2597282b4dbd80906c7196`

Immutable-boundary checks in the accepted workflow proved:

- accepted B2 source remains an ancestor;
- `v0.1.0-alpha.4^{commit}` remains `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- `v0.1^{commit}` remains `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- no `crates/` changes occurred between the accepted B2 source and B3 accepted source.

## Independent cross-gate reproof on the same source

Canonical authority workflow run `36733292694`, job `109948293585`, executed from the same source SHA `d82d1501f69658793df4737dace420c5add4be0c` and concluded `success`.

It re-proved:

- authority model: **8/8 PASS**
- A2 authority-gap matrix: **16/16 PASS**
- A1.3U attempt-authority contract: **5/5 PASS**
- B0 success-evidence contract: **18/18 PASS**
- B1 model: **24/24 PASS**
- B1 live Linux ptrace: **10/10 PASS**
- B2 model: **23/23 PASS**
- B2 live Linux ptrace: **10/10 PASS**
- B3 model: **20/20 PASS**
- B3 live Linux ptrace: **10/10 PASS**
- A1 adversarial corpus: **18/18 PASS**
- A1.2 mapping: **10/10 PASS**
- Semantics v3: **7/7 PASS**
- public M11 shared-FD fail-closed contract: **6/6 PASS**

The authority-workflow evidence artifact is `11105434091`, archive SHA-256 `3979f077ba26beebaf57968932ab8714003f94be798927cfc94d4ba3a0910d58`.

## What the accepted live gate demonstrated

The strengthened ten-test live corpus demonstrated from actual ptrace-derived connect evidence:

1. synchronous loopback `rc=0` binds actor + originating entry + socket FD + exact IPv4 destination and yields bounded success;
2. a negative connect result remains explicit Failure and never success;
3. actual nonblocking `-EINPROGRESS` is Pending and non-success;
4. destination substitution changes the evidence binding and cannot preserve success authority;
5. FD substitution, reused pairing identity, and replay cannot transfer authority;
6. actor and originating-entry substitution fail closed as Ambiguous/non-success;
7. malformed sockaddr evidence fails closed before authoritative destination construction;
8. controlled observer incompleteness turns otherwise successful traced evidence into Lost/non-success;
9. repeated construction from identical actual evidence is byte-identical and proof-digest deterministic;
10. later socket activity/state cannot relabel a recorded Failure.

The frozen 20-test model additionally covered IPv6 identity fields, Unix-domain representation, unexpected positive return -> Ambiguous, file-proposition laundering rejection, non-monotonic exits, same-FD/different-destination identity changes, and pending non-upgrade.

## Retained negative and engineering evidence

All pre-acceptance failures remain part of the evidence ledger.

### Static lifecycle failure

Run `36726950576` at source `a17efa3f394ad2d25d177bdba78c7019e96ef4bf` stopped at `clippy::zombie_processes` before live execution. The correction made child lifecycle closure unconditional without changing B3 semantics.

### Live 9/10 fixture failure

Run `36728346314` at source `a401bad87246e23de849a4cae57a747aaf16c7e4` reached the original live corpus and produced **9/10 PASS**. The synchronous-loopback fixture observed `ETIMEDOUT` and remained Failure. Artifact `11103254994`, archive SHA-256 `e1cfdb72cd5071d4598a5b56c70de2bf6d1c537f582e107c00789c03e3fdc0b4`.

The failure exposed a fixture IPv4 byte-order defect, not permission to relabel timeout. The correction was limited to constructing the intended loopback network-order address.

### Independent live-harness adequacy defect

`docs/development/P4_B3_LIVE_HARNESS_ADEQUACY_REVIEW.md` recorded that the original ten tests did not fully prove the frozen acceptance properties. In particular, Pending, substitution, FD reuse/replay, observer loss, and proof determinism were under-tested.

The harness was strengthened while preserving exactly ten mandatory live tests and the exact `10 passed; 0 failed` threshold. No B3 semantic rule or acceptance criterion was weakened.

### Formatting/static-only attempts

Subsequent runs that stopped at rustfmt or per-target dead-code lint were retained as pre-scientific engineering failures. The final dead-code correction did not use a lint whitelist; it made the frozen non-live destination model variants reachable to the test target without claiming live authority for them.

## Scientific interpretation

The A2 `P4.NET.CONNECT_DESTINATION` immediate-success gap does **not** currently justify a second runtime backend. Under the tested proposition and bounded Linux x86_64 scope, ptrace supplies sufficient entry/result evidence to establish synchronous connect success without laundering Pending, Failure, Lost, Ambiguous, replayed, or substituted evidence.

This is not a universal networking completeness claim. B3 does not establish later asynchronous connect completion authority, universal address-family coverage, or backend interchangeability.

## Next authorized gate

The only next P4-B gate authorized by this result is the already-preregistered **Cross-Proposition Falsification Gate** in `P4_B_TARGETED_SUCCESS_AUTHORITY_PROTOCOL.md`.

It must attack B1 + B2 + B3 together for evidence reuse, actor/causal substitution, FD reuse, replay, entry/exit mismatch, success/attempt/pending laundering, loss masking, proposition confusion, deterministic identity failure, and backend/profile-name authority inflation.

Only after that joint falsification may P4-B take one of its frozen bounded decisions. No second backend is justified merely because it appeared in an earlier plan.