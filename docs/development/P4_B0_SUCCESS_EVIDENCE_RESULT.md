# ExecSurface — P4-B0 Success-Evidence Contract Result

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Protocol: `docs/development/P4_B_TARGETED_SUCCESS_AUTHORITY_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

`P4_B0_EVIDENCE_CONTRACT_PASS_RESEARCH_ONLY`

Boundary:

`NO_COLLECTOR_CHANGE_AUTHORIZED_BY_B0`

B0 closes only the internal research evidence-interface contract. It does not establish live ptrace collection coverage for B1/B2/B3 and does not authorize public integration, raw-v2 reinterpretation, backend promotion, release, or tag movement.

## Accepted evidence

- source: `2246f7b5016fd7e17ceeeaeaa5ff35b53cd99b95`
- workflow: `36593660113`
- job: `109492909628`
- artifact: `11044334371`
- uploaded artifact SHA-256: `36a254686911a13d6040a9936efa3a8b8a25c9a5b74eb7efe39cbf338eb81e1a`
- Rust toolchain: `1.90.0`
- runner: Ubuntu 24.04

## Accepted gates

- branch/public-boundary verification: PASS
- isolated dependency lock generation: PASS
- rustfmt: PASS
- clippy `-D warnings`: PASS
- B0 success-evidence falsification: **18/18 PASS**
- A2 authority-gap matrix reproof: **16/16 PASS**
- A1.3U attempt-authority reproof: **5/5 PASS**
- A1 adversarial/mapping corpus: **18/18 PASS**
- A1.2 ptrace-v2 mapping reproof: **10/10 PASS**
- Semantics v3 reproof: **7/7 PASS**
- public M11 shared-FD contract reproof: **6/6 PASS**

## What B0 establishes

The research-only evidence contract structurally distinguishes:

- `AttemptObserved`
- `SuccessObserved`
- `FailureObserved`
- `PendingObserved`
- `Ambiguous`
- `Lost`

Within the frozen B0 model:

- entry-only evidence cannot become success;
- successful open requires a causally paired non-negative return and returned-FD identity;
- negative returns remain failure evidence;
- connect `EINPROGRESS` remains pending and non-success;
- rename/delete/connect zero-success operations require `rc=0`;
- wrong actor, wrong entry sequence, or non-monotonic exit pairing cannot acquire success authority;
- incomplete or warning-bearing observation becomes `Lost` rather than success;
- duplicate/replayed pairing is rejected;
- forged negative-success and returned-FD mismatch records are rejected;
- pairing identity and serialization are deterministic.

## Preserved negative/pre-test evidence

Earlier B0 attempts that stopped before scientific tests remain retained in GitHub Actions history. They included formatting, legacy A2 reproof lifetime, and strict-clippy failures. No assertion, threshold, proposition definition, test count, or acceptance condition was weakened to obtain the accepted run.

The final legacy-test lint correction preserved the serialized A2 `no_gap` value explicitly while satisfying strict clippy; A2 proposition classifications were not redefined.

## Public isolation

The accepted workflow reverified that:

- `v0.1.0-alpha.4` still resolves to `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- stable `v0.1` still resolves to the same source;
- no `crates/` path changed after the accepted A2 source during B0;
- public raw-v2 meaning and default observer remain unchanged.

## Next authorized gate

**P4-B1 — `P4.FILE.OPEN_OBJECT` targeted ptrace success-authority requalification.**

B1 must preregister and falsify live/research ptrace entry/exit binding for `open`, `openat`, and supported `openat2` cases, including successful open followed immediately by close, failed opens, unmatched/mismatched entry-exit evidence, fd reuse, PATH-TOCTOU, CLOEXEC/exec, clone/shared-fd interaction, and observer loss/truncation.

No B2/B3 conclusion may inherit B1 results, and no second backend is justified unless the proposition-specific ptrace gate fails under the frozen criteria.
