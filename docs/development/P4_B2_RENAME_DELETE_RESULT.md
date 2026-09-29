# ExecSurface — P4-B2 `P4.FILE.RENAME_DELETE` Result

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Protocol: `docs/development/P4_B2_RENAME_DELETE_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

`P4_B2_FILE_RENAME_DELETE_PASS_BOUNDED_RESEARCH_ONLY`

Boundary:

`NO_PUBLIC_INTEGRATION_OR_SECOND_BACKEND_AUTHORIZED`

B2 establishes, within the frozen research scope and pinned Linux x86_64 CI environment, that targeted ptrace syscall-entry/syscall-exit evidence can establish bounded successful-effect authority for `P4.FILE.RENAME_DELETE` when the exact operation context is captured and the causally matched syscall result is `rc=0`.

This result is research-only. It does not alter public raw-v2 bytes or meaning, the default observer, `main`, `v0.1.0-alpha.4`, stable `v0.1`, or release artifacts.

## Accepted model evidence

Accepted model source:

`85f5b18cfb2186b5b15a194b76c67433e306aeeb`

Model workflow:

- workflow run `36598958309`
- job `109511055761`
- conclusion: `success`
- B2 model corpus: **23/23 PASS**
- B1 model: **24/24 PASS**
- B0 success-evidence contract: **18/18 PASS**
- A2 authority-gap matrix: **16/16 PASS**
- A1.3U attempt-authority contract: **5/5 PASS**
- A1 adversarial/mapping corpus: **18/18 PASS**
- A1.2 mapping: **10/10 PASS**
- Semantics v3: **7/7 PASS**
- public M11: **6/6 PASS**

Model-stage decision:

`P4_B2_MODEL_CONTRACT_PASS_RESEARCH_ONLY`

Boundary at model stage:

`LIVE_PTRACE_REQUALIFICATION_STILL_REQUIRED`

## Accepted live ptrace evidence

Accepted live source:

`8aef762dce8121c4ef87be9df24c095c649ace8c`

Mandatory live ptrace workflow:

- workflow run `36601419697`
- job `109519483857`
- runner: Ubuntu 24.04 / Linux x86_64
- kernel: `Linux runnervmtr4k5 6.17.0-1022-azure #22-Ubuntu SMP Mon Jul 27 17:24:03 UTC 2026 x86_64 GNU/Linux`
- conclusion: `success`
- B2 live ptrace corpus: **10/10 PASS**
- evidence artifact: `11048254357`
- artifact SHA-256: `3b8ce006291761b061384d0d33a296849db0cd0afb013b74bea503e5366942b6`

The accepted live workflow also re-proved:

- B2 model: **23/23 PASS**
- B1 hardened model: **24/24 PASS**
- B0 success-evidence contract: **18/18 PASS**
- A2 authority-gap matrix: **16/16 PASS**
- A1.3U attempt-authority contract: **5/5 PASS**
- A1 adversarial/mapping corpus: **18/18 PASS**
- A1.2 mapping: **10/10 PASS**
- Semantics v3: **7/7 PASS**
- public M11 shared-FD fail-closed contract: **6/6 PASS**

Rustfmt and clippy `-D warnings` passed before the accepted scientific execution.

## What the live gate demonstrated

The accepted live ptrace gate demonstrated from actual traced syscalls:

1. `rename` exact source/target entry context paired to `rc=0`;
2. `renameat` relative source/target plus real directory-FD arguments paired to `rc=0`;
3. `renameat2` `RENAME_NOREPLACE` flags were observed and classified, with success on the accepted runner;
4. `unlink` paired to `rc=0`;
5. `unlinkat` exact `AT_FDCWD`/flags context paired to `rc=0`;
6. `rmdir` paired to `rc=0`;
7. missing-source rename remained `ENOENT` failure and never became success authority;
8. source-context substitution could not reuse valid successful evidence;
9. repeated construction from the same traced effect produced deterministic authority bytes;
10. filesystem postcondition alone could not substitute for a negative syscall result.

The proof therefore derives from paired syscall result and entry context. Filesystem postcondition checks are corroboration only.

## Retained counterexample and correction

An earlier live execution reached the scientific corpus and produced **9/10 PASS**. The sole failure was the `unlinkat` dirfd fixture: expected `AT_FDCWD=-100`, but the harness decoded the x86_64 register value as `4294967196` because a 32-bit signed syscall argument had been zero-extended before being cast directly to `i64`.

This failure is retained as evidence. It exposed a live-harness ABI decoding defect rather than an authority-rule failure.

The correction was strictly limited to sign-extending 32-bit `dirfd` syscall arguments before interpretation:

`i64::from(value as u32 as i32)`

The same frozen ten live tests were then rerun. No test, assertion, proposition, success rule, failure rule, operation scope, or acceptance threshold was weakened or removed.

## Interpretation

The A2 `P4.FILE.RENAME_DELETE` success-semantics gap does **not** currently justify a second runtime backend. Under the frozen tested scope, ptrace supplies enough exact entry context and syscall-exit result evidence to establish bounded success authority for the covered rename/delete families.

This is not a universal filesystem completeness claim. It is a proposition-scoped, bounded research result only.

## Next authorized gate

B3 may now be preregistered for:

`P4.NET.CONNECT_DESTINATION`

B3 must test targeted ptrace connect-entry/result evidence first. In particular:

- `rc=0` may represent synchronous successful connect only when bound to the exact socket FD and destination context;
- `-EINPROGRESS` must remain pending and must not be laundered into success;
- other negative errno results remain failures;
- destination substitution, actor/entry substitution, FD reuse, unsupported address families, and observer loss must fail closed;
- later network IO or socket existence cannot substitute for the connect result.

No second backend is justified for B2 by this result.