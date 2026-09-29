# ExecSurface — P4-B1 FILE.OPEN_OBJECT Result

Date: 2026-09-29
Parent P4: #107
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/P4_B1_FILE_OPEN_OBJECT_PROTOCOL.md`
Status: **CLOSED — PASS BOUNDED / RESEARCH ONLY**

## Decision

`P4_B1_FILE_OPEN_OBJECT_PASS_BOUNDED_RESEARCH_ONLY`

Boundary:

`NO_PUBLIC_INTEGRATION_OR_SECOND_BACKEND_AUTHORIZED`

B1 establishes, under the frozen Linux x86_64 research scope, that targeted ptrace syscall-entry/syscall-exit/state evidence can provide bounded success authority for `P4.FILE.OPEN_OBJECT` without introducing a second backend. This result does not change public raw-v2 bytes, public alpha.4 semantics, the default observer, `main`, `v0.1.0-alpha.4`, or stable `v0.1`.

## Accepted evidence

- source: `0063ff491b8845f70c9519e8a3b15e968668465f`
- workflow: `36597329721`
- job: `109505490689`
- artifact: `11045949336`
- artifact digest: `sha256:a17531a10ebab3f7739c25b3b5a279b32cd67a5e2dbd932432fb6b80f9799059`
- runner kernel: `Linux 6.17.0-1022-azure x86_64 GNU/Linux`

## Accepted gates

- rustfmt: PASS
- clippy `-D warnings`: PASS
- B1 hardened model: **24/24 PASS**
- mandatory live ptrace kernel-object falsification: **10/10 PASS**
- B0 success-evidence reproof: **18/18 PASS**
- A2 authority-gap matrix reproof: **16/16 PASS**
- A1.3U attempt-authority reproof: **5/5 PASS**
- A1 adversarial corpus reproof: **18/18 PASS**
- A1.2 mapping reproof: **10/10 PASS**
- Semantics v3 reproof: **7/7 PASS**
- public M11 shared-FD fail-closed contract: **6/6 PASS**

## Live ptrace evidence exercised

The mandatory live suite passed all preregistered success/failure classes represented by the frozen harness:

1. ENOENT failed open remains failure and obtains no object identity.
2. numeric FD reuse receives a new generation and new object identity.
3. hardlink pathname variation preserves the same kernel object identity.
4. immediate close requires no later read/write event to establish the open transition.
5. multitask FD-table uncertainty fails closed.
6. successful `open` binds the returned FD to kernel object identity.
7. successful `openat2` is bound on the current runner kernel.
8. `openat` relative to CWD binds the opened kernel object.
9. `openat` with a real dirfd binds the target object rather than laundering the directory pathname.
10. rename while ptrace-stopped does not rewrite the identity of the already-opened object.

## Scientific interpretation

The A2 `P4.FILE.OPEN_OBJECT` gap was not evidence that ptrace was fundamentally incapable. It was a gap in the accepted raw-v2/product representation. Under the isolated B1 research path, causally paired entry/exit evidence plus returned-FD state and kernel object binding is sufficient for bounded proposition authority under the tested Linux x86_64 scope.

This does **not** prove universal ptrace completeness, kernel-version portability, public-schema compatibility, or production readiness. In particular, unresolved observer loss, unclassified FD-table uncertainty, or unsupported syscall semantics must continue to fail closed.

## Preserved negative evidence

Earlier B1 attempts that stopped on missing research module plumbing, rustfmt-only differences, helper-name shadowing, and exact formatting remain retained as engineering evidence. No assertion, test count, proposition definition, threshold, or acceptance condition was weakened to obtain this PASS.

## Consequence for backend strategy

A second backend is **not justified for `P4.FILE.OPEN_OBJECT` by current evidence**. P4-B must continue proposition-by-proposition. The next authorized gate is B2 for `P4.FILE.RENAME_DELETE`, testing targeted ptrace syscall-exit/effect-result evidence first. Only a preregistered B2 failure that leaves a concrete proposition-specific gap may make a second backend eligible for that proposition.
