# Rust Community Code Review Engagement

Status: **PREPARED — FORUM POSTING PENDING AUTHENTICATED ACCOUNT**  
Date opened: **2026-09-29**  
Last aligned to public release: **2026-10-08**  
External referral: Rust Foundation → official Rust Users Forum `code review` category

## 1. Source event

AETHER X requested independent technical evaluation of ExecSurface from the Rust Foundation. Abi Broom (Director of Finance & Operations, Rust Foundation) replied that the Foundation does not generally provide project reviews and pointed AETHER X to the official Rust Users Forum `code review` category as the most relevant Rust Project space.

This is a routing/referral event, not validation, endorsement, or technical review.

## 2. Current source-of-truth baseline

The original outreach named an earlier alpha; that reference is historical only.

Current public review target:

- repository: `AETHERXGLOBAL/execsurface`
- public release: `v1.0.0`
- release source commit: `e70169b959f2163715090c371335fa6c5591e3e4`
- immutable Action: `AETHERXGLOBAL/execsurface@v1.0.0`
- stable Action: `AETHERXGLOBAL/execsurface@v1`
- registry package: `execsurface = 1.0.0`
- platform boundary: Linux x86_64
- public reference observer: native `ptrace`
- product boundary: runtime behavioral-integrity and verification for observed execution-surface drift; not antivirus, EDR, malware detection, sandboxing, or proof of software safety.

Stable v1.0.0 keeps incomplete or ambiguous observation non-PASS-eligible, preserves conservative shared-FD incompleteness where exact attribution is not established, and keeps BPF-LSM/kernel-hook work research/managed and non-default.

## 3. Why ask the Rust community

The requested review is deliberately narrower than a general architecture endorsement.

Primary implementation target:

`crates/execsurface-observe/src/linux_ptrace.rs`

Review should focus on implementation correctness and maintainability at the Rust/Unix FFI boundary, especially:

1. `unsafe` boundaries around `fork`, `ptrace`, `waitpid`, `execvp`, and process-memory metadata reads;
2. child-after-`fork` safety assumptions before `exec`;
3. ptrace lifecycle/state-machine correctness across fork/vfork/clone/exec/exit/signal paths;
4. ownership/state modeling for fd lifecycle and shared-FD ambiguity;
5. error propagation and fail-closed completeness semantics;
6. robustness of syscall-entry/exit pairing and ESRCH/ECHILD edge handling;
7. opportunities to reduce unsafe surface or make invariants easier to audit;
8. idiomatic Rust concerns hidden by the Linux-specific system interface.

Architecture questions such as ptrace vs LSM/BPF are tracked separately in `docs/architecture/PTRACE_VS_LSM_ARCHITECTURE_REVIEW.md`.

## 4. Evidence available to reviewers

- `docs/STATUS.md` — current public state
- `docs/TECHNICAL_EVALUATION.md` — independent technical evaluation path
- `docs/architecture/PTRACE_VS_LSM_ARCHITECTURE_REVIEW.md` — observer-authority review
- `docs/releases/v1.0.0.md` — current release record
- public CI/adversarial fixtures under `.github/workflows/` and `.github/m12-fixtures/`
- issue #118 — stable-v1 external evaluation and post-release review hub

Failures, REVIEWs, incomplete observations, usability friction, and code defects are useful evidence and must be retained rather than tuned away.

## 5. Proposed Rust Users Forum post

### Title

**Code review request: Rust/Linux ptrace observer with explicit fail-closed evidence semantics**

### Body

Hello,

The Rust Foundation pointed me to this category as the appropriate place to ask for community code review.

I maintain **ExecSurface**, an Apache-2.0 Rust CLI/GitHub Action for runtime behavioral-integrity verification on Linux x86_64. The current public release is **v1.0.0**.

I am **not** looking for endorsement or a broad security claim. I would specifically value criticism of the Rust/Linux implementation boundary, especially the native ptrace observer:

`crates/execsurface-observe/src/linux_ptrace.rs`

The observer launches one declared command, follows descendants, pairs selected syscall entry/exit events, tracks a bounded fd lifecycle model, and marks evidence incomplete rather than allowing PASS when declared observation invariants cannot be justified.

Areas where review would be especially useful:

- whether the `unsafe`/libc boundaries are correctly minimized and documented;
- child-after-`fork` assumptions before `exec`;
- ptrace state-machine handling across fork/vfork/clone/exec/exit/signals;
- fd lifecycle/shared-FD modeling and conservative incompleteness;
- ESRCH/ECHILD and lifecycle edge cases;
- whether the Rust state model makes invariants auditable or hides possible bugs;
- places where a safer or more idiomatic Rust structure would reduce review burden.

Known limitations are intentionally public. Pathname data copied at syscall entry is access-attempt metadata rather than kernel-object identity, ptrace can perturb scheduling, and some shared-FD concurrency is conservatively marked incomplete. BPF-LSM/kernel-hook work remains research/managed and non-default; ptrace remains the bounded public reference observer.

Repository: `https://github.com/AETHERXGLOBAL/execsurface`

Current release: `https://github.com/AETHERXGLOBAL/execsurface/releases/tag/v1.0.0`

Observer implementation: `https://github.com/AETHERXGLOBAL/execsurface/blob/main/crates/execsurface-observe/src/linux_ptrace.rs`

Technical evaluation: `https://github.com/AETHERXGLOBAL/execsurface/blob/main/docs/TECHNICAL_EVALUATION.md`

Post-release review hub: `https://github.com/AETHERXGLOBAL/execsurface/issues/118`

Negative findings are welcome. If something is unsafe, non-idiomatic, overcomplicated, racy, or hard to audit, that is exactly the feedback requested.

Ahmed Younis  
AETHER X GLOBAL

## 6. Posting rule

Immediately before posting, re-check the latest public release and `main` state. Do not send stale version, platform, authority, or validation claims.

## 7. Response handling

Classify substantive forum replies as one of:

- `RUST_CORRECTNESS_DEFECT`
- `UNSAFE_BOUNDARY_CONCERN`
- `LIFECYCLE_STATE_MACHINE_CONCERN`
- `IDIOMATIC_RUST_IMPROVEMENT`
- `ARCHITECTURE_OBSERVATION`
- `REPRODUCTION_RESULT`
- `NO_ACTION / OPINION_ONLY`

For technical criticism use:

`CLAIM -> CRITICISM -> TEST -> EVIDENCE -> DECISION`

Do not defend the implementation by default. Reproduce or falsify criticism before closing it.

## 8. Claim boundary

A Rust community review, even if positive, must not be represented as:

- Rust Foundation validation;
- Rust Project endorsement;
- Rust Users Forum endorsement;
- proof of ExecSurface safety;
- production-readiness certification.

A review is evidence only about the specific code and conditions actually examined.
