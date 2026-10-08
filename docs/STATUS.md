# ExecSurface — Current Status

This file is the authoritative current public product state. Historical milestone, prerelease and engagement documents are evidence records and may describe the release that was current when they closed.

## Current public product state

**ExecSurface v0.1.0-alpha.6 is the current supported Alpha release for Linux x86_64 within the documented native-ptrace product boundary.**

Current bounded state:

`ALPHA6_SUPPORTED_ALPHA_WITH_STAGE2_SEMANTIC_REMEDIATION — LINUX_X86_64_PTRACE`

- Public release: `v0.1.0-alpha.6`
- Immutable Action pin: `AETHERXGLOBAL/execsurface@v0.1.0-alpha.6`
- Stable GitHub Action: `AETHERXGLOBAL/execsurface@v0.1`, promoted to Alpha.6 only after immutable release and consumer gates pass.
- Registry package: `execsurface = 0.1.0-alpha.6`, published only after the GitHub release and stable Action gates pass.
- Previous release `v0.1.0-alpha.5` remains immutable and available for historical reproduction and rollback.
- Alpha.5 normalization profile 3 is **not** silently reinterpreted under Alpha.6 profile 4; incompatible Alpha.5 baselines are rejected explicitly and must be relearned under corrected semantics.
- Policy schema v3 is opt-in; v1/v2 meanings remain unchanged.
- Stage-2 adds bounded FD-state, object/path authority, custody, policy-expressiveness, CI, and verdict-materialization correctness repairs.
- Supported public product scope: Linux x86_64.
- Public default/reference observer: native `ptrace`.
- ExecSurface verdict / exit-code contract: PASS `0`, ERROR `2`, REVIEW `10`, BLOCK `20`.

The research/product classification remains runtime behavioral verification, execution semantics and semantic-evidence correctness. It is not cybersecurity research.

Alpha.6 supersedes Alpha.5 as the recommended public Alpha after the release pipeline succeeds. It does not delete or rewrite Alpha.5 artifacts, issues, pull requests, comments, review threads or external-engagement records.

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

Independent external validation remains open as additional evidence, not as a blocker to the bounded Final Supported Alpha state above.

This repository does **not** claim independent validation, endorsement, adoption or approval by external reviewers or organizations merely from internal qualification, downloads, stars, outreach or community participation.

Current external-evidence paths remain:

- evidence qualification/tracking: issue `#114`;
- public Alpha.5 review hub: issue `#118`;
- external real-workload production evidence: issue `#140`.

Any future external failure, counterexample, no-fit result or limitation remains valid evidence and may constrain later claims/releases.

## Historical evidence handling

Historical failures, prerelease decisions, closed-gate workflows and negative evidence are retained. Operationally obsolete files may be moved out of active paths into `docs/archive/` or `.github/workflow-archive/`; that is repository hygiene, not evidence deletion or history rewriting.

For current public facts, use this file, `README.md`, the latest GitHub Release and `docs/releases/v0.1.0-alpha.5.md`.
