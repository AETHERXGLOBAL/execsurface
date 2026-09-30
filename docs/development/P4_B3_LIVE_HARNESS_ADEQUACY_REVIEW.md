# ExecSurface — P4-B3 Live Harness Adequacy Review

Date: 2026-09-30
Parent program: #100
Parent P4: #107
Parent P4-B: #109
Reviewed source: `a401bad87246e23de849a4cae57a747aaf16c7e4`
Status: **TEST-ADEQUACY CORRECTION REQUIRED — NO B3 SCIENTIFIC DECISION**

## Scope

This review evaluates whether the mandatory live Linux x86_64 ptrace corpus in `tests/b3_ptrace_connect_live.rs` actually demonstrates the ten frozen live conditions in `P4_B3_CONNECT_DESTINATION_PROTOCOL.md`.

It does **not** change the B3 proposition, authority semantics, fixtures' intended scientific meaning, model thresholds, public v2 semantics, or the required `10/10` live acceptance count.

## Retained execution history

- Run `36726950576` at source `a17efa3f394ad2d25d177bdba78c7019e96ef4bf` stopped at `clippy::zombie_processes` before the live corpus. Classification: engineering/static-gate failure; no live scientific result.
- Source `a401bad87246e23de849a4cae57a747aaf16c7e4` made child lifecycle closure unconditional without changing B3 semantics. Its workflow execution remains retained regardless of outcome.

## Independent adequacy finding

The frozen protocol requires live evidence for exact FD/destination plus actor/entry/exit binding, pending semantics, substitution/reuse rejection, observer-loss fail-closed behavior, deterministic proof identity, and later-state non-reclassification.

The reviewed ten-test live harness does not fully demonstrate those claims:

1. The nonblocking test accepts either `Pending` **or** `Failure`; therefore it can pass without proving the frozen `EINPROGRESS -> Pending` live condition.
2. Destination substitution only compares raw sockaddr bytes; it does not prove that substituted evidence cannot preserve the authority/proof identity.
3. FD reuse test only checks monotonic sequence numbers; it does not attempt authority transfer or verify a changed pairing identity.
4. Actor/entry test does not construct a mismatched actor or entry-exit pair and therefore does not prove fail-closed substitution.
5. No live test injects observer incompleteness/loss into evidence derived from an actual traced connect and proves non-success.
6. Determinism is checked only for destination bytes, not the composed evidence/proof serialization/digest required by the protocol.

The synchronous success, deterministic negative result, malformed sockaddr, and later-state non-reclassification directions are useful but are insufficient by themselves to satisfy the frozen closure contract.

## Classification

`TEST_HARNESS_ADEQUACY_DEFECT / PRE-CLOSURE`

This is not a B3 semantic counterexample and is not permission to weaken any test. A green run of the reviewed harness is evidence that those exact tests ran, but it is **not sufficient evidence to close B3**.

## Authorized bounded correction

Strengthen the existing ten live tests so that they directly compose actual ptrace-derived connect evidence through the already-frozen B0/B3 evidence model and demonstrate the frozen conditions:

- synchronous `rc=0` -> bounded success with actor + entry + FD + destination binding;
- negative return -> explicit failure;
- actual nonblocking `-EINPROGRESS` -> Pending and non-success;
- destination substitution -> build rejection / changed proof identity;
- FD substitution and reused pairing identity -> no authority transfer;
- actor and entry-sequence substitution -> Ambiguous/non-success;
- malformed sockaddr -> no authoritative context;
- controlled observer loss on actual traced evidence -> Lost/non-success;
- identical actual evidence -> byte/digest deterministic proof identity;
- later socket state -> cannot relabel the recorded connect result.

The correction must keep exactly ten mandatory live tests and the existing `10 passed; 0 failed` workflow threshold. It may not add a broad whitelist, accept `Failure` as proof of `Pending`, infer success from later I/O, or alter public code under `crates/`.

## Decision rule

Only a new workflow run from the strengthened harness may be considered for B3 closure, and only if static gates, all ten live tests, the frozen 20/20 model, predecessor reproofs, public M11, and immutable release boundaries are independently verified.
