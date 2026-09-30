# ExecSurface — P4-B3 `P4.NET.CONNECT_DESTINATION` Targeted Ptrace Success-Authority Protocol

Date: 2026-09-30
Parent program: #100
Parent P4: #107
Parent P4-B: #109
Predecessor: `P4_B2_FILE_RENAME_DELETE_PASS_BOUNDED_RESEARCH_ONLY`
Accepted B2 result commit: `8d1e7afc034f96969eecac96d7d57e2198deeedf`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — RESEARCH ONLY**

## Research question

Can targeted Linux x86_64 ptrace connect-entry/syscall-exit evidence establish bounded authority for the exact socket FD and destination when `connect(2)` returns synchronous `rc=0`, while preserving `EINPROGRESS` as pending and every other negative result as failure?

B3 does not authorize public-v2 reinterpretation, collector promotion, release movement, a second backend, or changes under `crates/`.

## Fixed team

1. **Innovation Scientist / Linux Network Runtime Architect** — seek the smallest evidence contract that closes only the measured connect-success gap.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks `EINPROGRESS` laundering, later-I/O inference, socket-existence inference, address-family scope creep, public-v2 reinterpretation, threshold weakening, and backend-first implementation.
3. **Independent Falsifier / Red Team** — attacks actor/entry substitution, destination substitution, FD reuse, replay, malformed sockaddr evidence, unsupported families, negative results, pending connects, observer loss, and deterministic-proof identity.
4. **Independent Milestone Reviewer** — verifies source SHA, frozen fixtures, live syscall evidence, exact test counts, retained failures, artifact digest, public isolation, and allowed decision.

## Dynamic specialists

Linux ptrace/syscall lifecycle; socket/connect semantics; sockaddr ABI decoding; IPv4/IPv6/Unix-domain identity; nonblocking connect state; FD lifecycle/reuse; concurrency/restart semantics; formal evidence semantics; Rust typed API; deterministic serialization; CI evidence engineering.

## Frozen inputs

- `docs/development/P4_B_TARGETED_SUCCESS_AUTHORITY_PROTOCOL.md`
- `docs/development/P4_B2_RENAME_DELETE_RESULT.md`
- `docs/development/P4_B1_FILE_OPEN_OBJECT_RESULT.md`
- `docs/development/P4_B0_SUCCESS_EVIDENCE_RESULT.md`
- `docs/development/P4_A2_AUTHORITY_GAP_RESULT.md`
- canonical research model: `experiments/p4-backend-authority`
- immutable alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

No success condition, pending condition, failure rule, fixture, test, operation scope, or acceptance threshold may be weakened after execution starts.

## Frozen operation scope

B3 classifies only `connect(2)` when all required entry evidence is captured. Initial bounded address-family scope:

- `AF_INET` — exact IPv4 address and port;
- `AF_INET6` — exact IPv6 address and port, with flow/scope fields preserved when present;
- `AF_UNIX` — only when the captured sockaddr can be represented losslessly by the research model.

Unsupported, truncated, unreadable, malformed, or unclassified sockaddr evidence fails closed and cannot become success authority.

## Frozen authority contract

`P4.NET.CONNECT_DESTINATION` may become bounded synchronous-success authority only when all hold:

1. exact actor/TID and connect entry are observed;
2. socket FD and destination bytes are captured at entry;
3. sockaddr family/length decode is supported and complete;
4. canonical destination digest equals the argument digest frozen at entry;
5. causally matched exit belongs to the same actor and entry sequence;
6. raw return is exactly `0`;
7. observation health is complete and warning-free;
8. pairing has not been consumed/replayed;
9. deterministic proof identity is stable.

`-EINPROGRESS` is **Pending**, never Success. Any other negative Linux errno is Failure. Entry without exit is AttemptOnly. Unexpected nonnegative return is Ambiguous. Later socket I/O, socket existence, peer state, or application success cannot substitute for the connect syscall result in B3.

## Required model falsification

At minimum the model corpus must cover:

- synchronous `rc=0` success;
- `EINPROGRESS` pending and not success;
- `ECONNREFUSED` or equivalent negative failure;
- entry-only attempt;
- actor mismatch;
- entry-sequence mismatch;
- destination substitution;
- socket-FD substitution;
- duplicate/replay rejection;
- non-monotonic exit ordering;
- unsupported address family;
- malformed/truncated sockaddr;
- observer incomplete/warning -> lost;
- unexpected positive return -> ambiguous;
- connect authority cannot satisfy file propositions;
- backend name cannot upgrade weak evidence;
- deterministic serialization/proof digest;
- same destination on a reused FD is not the same pairing identity;
- different destination on the same FD changes identity;
- pending evidence cannot be upgraded by later-I/O-only evidence.

## Mandatory live Linux ptrace gate

Model-only evidence is insufficient. A closure candidate must execute live Linux x86_64 ptrace tests demonstrating from actual traced syscalls:

1. loopback synchronous successful connect with exact FD + destination and matched `rc=0`;
2. deterministic failure retained as failure;
3. a nonblocking connect producing `EINPROGRESS` when reproducibly obtainable; if the runner resolves synchronously, record the environment result and use a controlled fixture that can prove pending semantics without relabeling synchronous success;
4. destination substitution cannot preserve proof identity;
5. FD substitution/reuse cannot transfer authority;
6. entry/actor substitution cannot preserve authority;
7. unsupported/malformed sockaddr fails closed;
8. observer-loss case remains non-success;
9. repeated construction from identical evidence is byte/digest deterministic;
10. later socket state/I/O cannot override the original connect result classification.

No test may infer connect success solely from later application behavior.

## Cross-gate reproof

Before closure, the accepted workflow must re-prove the frozen B3 model/live corpus plus B2 **23/23**, B1 **24/24**, B0 **18/18**, A2 **16/16**, A1.3U **5/5**, A1 adversarial **18/18**, A1.2 mapping **10/10**, Semantics v3 **7/7**, public M11 **6/6**, and immutable alpha.4/stable-tag boundaries.

## Kill / escalation criteria

B3 fails or remains incomplete if ptrace cannot bind exact destination + FD + actor + matched exit; `EINPROGRESS` can become success; negative results can become success; unsupported sockaddr can become authoritative; FD reuse/replay transfers authority; observer loss can produce success; or safe closure would require weakening public alpha.4 semantics.

Only a concrete proposition-specific missing-evidence result after the frozen gate may justify a tiny second-backend prototype for this proposition.

## Allowed B3 decisions

- `P4_B3_CONNECT_DESTINATION_PASS_BOUNDED_RESEARCH_ONLY`
- `P4_B3_CONNECT_DESTINATION_INCOMPLETE_EVIDENCE`
- `P4_B3_CONNECT_DESTINATION_FALSE_AUTHORITY_PATH_FOUND`
- `P4_B3_INCOMPLETE_FOR_SAFE_PTRACE_AUTHORITY`

Only bounded PASS may authorize the cross-proposition P4-B falsification/decision gate. No B3 outcome alone authorizes public integration or a second backend.