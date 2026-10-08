# ExecSurface — Current Status

This file is the authoritative current public product state. Historical milestone, prerelease and engagement documents are evidence records and may describe the release that was current when they closed.

## Current public product state

**ExecSurface v1.0.0 is the current supported stable release for Linux x86_64 within the documented native-ptrace product boundary.**

Current bounded state:

`V1_0_0_STABLE_INTERNALLY_QUALIFIED — LINUX_X86_64_PTRACE`

- Public release: `v1.0.0`
- Immutable Action pin: `AETHERXGLOBAL/execsurface@v1.0.0`
- Stable GitHub Action: `AETHERXGLOBAL/execsurface@v1`, promoted to v1.0.0 only after immutable release and consumer gates passed.
- Registry package: `execsurface = 1.0.0`; exact-version zero-contact registry installation passed after publication.
- Previous public Alpha `v0.1.0-alpha.6` remains immutable and available for historical reproduction and rollback.
- Alpha.5 also remains immutable historical evidence.
- Alpha.5 normalization profile 3 is **not** silently reinterpreted under stable profile 4; incompatible Alpha.5 baselines are rejected explicitly. Alpha.6 profile-4 baselines were qualification inputs for v1 and remain rollback-compatible within the frozen contract.
- Policy schema v3 is opt-in; v1/v2 meanings remain unchanged.
- Stage-2 adds bounded FD-state, object/path authority, custody, policy-expressiveness, CI, and verdict-materialization correctness repairs.
- Supported public product scope: Linux x86_64.
- Public default/reference observer: native `ptrace`.
- ExecSurface verdict / exit-code contract: PASS `0`, ERROR `2`, REVIEW `10`, BLOCK `20`.

The research/product classification remains runtime behavioral verification, execution semantics and semantic-evidence correctness. It is not cybersecurity research.

v1.0.0 supersedes Alpha.6 as the recommended stable public release. The v1 release does not delete or rewrite Alpha.6/Alpha.5 artifacts, issues, pull requests, comments, review threads or external-engagement records.

Stable release transaction evidence:

- GitHub Release workflow `37771109825` — SUCCESS;
- crates.io publication workflow `37771295572` — SUCCESS;
- stable `v1` channel resolves to immutable `v1.0.0` source;
- release asset SHA-256: `6892dc54e6f3f842cfecffa77ae0814c9e30aec219f2251c7c39a9f19f02ca7b`;
- external independent validation is not claimed.

## Historical Alpha.5 qualification

The supported-product closeout was executed against the already-published Alpha.5 artifact and immutable Alpha.5 source rather than a replacement build.

Authoritative qualification evidence:

- qualification PR run `37144655462`: SUCCESS;
- post-merge `main` qualification run `37144894250`: SUCCESS;
- post-merge Public Consumer Smoke run `37144894395`: SUCCESS;
- post-merge normal CI run `37144894249`: SUCCESS.

The qualification covered:

- exact published artifact on Ubuntu 22.04 and Ubuntu 24.04;
- checksum and GitHub attestation verification plus archive-safety checks;
- Debian 12 and Fedora 42 userland portability under explicit ptrace capability; these container results do **not** claim distinct-kernel coverage;
- 250 repeated unchanged PASS checks and 100 repeated controlled REVIEW checks with baseline byte immutability;
- malformed, missing and unsupported baseline/policy inputs with fail-closed behavior;
- external interruption of ExecSurface itself cannot complete as PASS;
- immutable Alpha.5-source replay of the 300-run PATH-TOCTOU adversarial suite plus shared-FD falsifier on Ubuntu 22.04 and Ubuntu 24.04;
- ptrace regression and the full frozen workspace suite on both Ubuntu hosts;
- exact crates.io clean installation;
- stable `@v0.1` identity against the immutable Alpha.5 source;
- stable Action PASS outputs/evidence and ERROR fail-closed behavior.

This is bounded product qualification for the declared Linux x86_64/native-ptrace scope. It is not a claim of universal compatibility with every Linux distribution, kernel, container policy or host security configuration.

## Declared Alpha.5 limitation — target outcome is not verdict-bearing

Alpha.5 separates the target command's outcome from the ExecSurface drift verdict.

The wrapped target's exit code or terminating signal is recorded in the report, but **does not by itself change PASS / REVIEW / BLOCK in Alpha.5**. A target that exits nonzero or is signalled can therefore receive `ExecSurface: PASS` when no policy-relevant execution-surface finding exists.

Therefore:

- `ExecSurface: PASS` means the selected observation/comparison/policy found no review/block execution-surface drift;
- it does **not** mean the wrapped target command succeeded;
- CI users must preserve the target command's own success/failure gate when target correctness matters;
- the GitHub Action `exit-code` output is the ExecSurface verdict code, not the native target exit code.

This limitation was discovered during final qualification and is retained as issue `#143`. Alpha.5 semantics are not silently rewritten after publication; any future target-outcome enforcement change must be versioned and compatibility-reviewed.

## Distribution state

Alpha.5 completed the release and post-publication chain: deterministic dual build and compatibility checks; checksum and provenance generation; GitHub prerelease publication; public-artifact byte/source equivalence; clean binary consumption on Ubuntu 22.04 and Ubuntu 24.04; immutable-tag installation and Action verdict checks; stable `v0.1` promotion only after public proofs; stable-channel public Action checks; registry publication; zero-contact registry install/smoke; and the later Final Alpha.5 qualification described above.

Canonical public-release workflow run: `36910725515`.

## Product boundary

ExecSurface is a **runtime behavioral-integrity and verification layer for observed execution-surface drift**.

It is not antivirus, EDR, SIEM, malware detection, a sandbox, a general mandatory-access-control system or proof that software is safe.

- Observed behavior is not all possible behavior.
- Incomplete or ambiguous observation evidence cannot silently become PASS.
- A baseline is not policy.
- Backend identity alone does not create semantic authority.
- Native ptrace remains the bounded public reference observer.
- Windows support is not claimed.
- ARM64 support is not claimed.
- BPF-LSM/kernel-hook work remains research/managed and non-default unless a later evidence gate explicitly changes that boundary.

## External validation state

Independent external validation remains open as additional evidence. Under the recorded v1 governance amendment it was not a release blocker, and v1.0.0 does not claim independent external validation.

This repository does **not** claim independent validation, endorsement, adoption or approval by external reviewers or organizations merely from internal qualification, downloads, stars, outreach or community participation.

Current external-evidence paths remain:

- evidence qualification/tracking: issue `#114`;
- public Alpha review hub (opened during Alpha.5 and retained for continuity): issue `#118`;
- external real-workload production evidence: issue `#140`.

Any future external failure, counterexample, no-fit result or limitation remains valid evidence and may constrain later claims/releases.

## Historical evidence handling

Historical failures, prerelease decisions, closed-gate workflows and negative evidence are retained. Operationally obsolete files may be moved out of active paths into `docs/archive/` or `.github/workflow-archive/`; that is repository hygiene, not evidence deletion or history rewriting.

For current public facts, use this file, `README.md`, the latest GitHub Release and `docs/releases/v1.0.0.md`. Alpha.6 and Alpha.5 release records remain historical evidence.
