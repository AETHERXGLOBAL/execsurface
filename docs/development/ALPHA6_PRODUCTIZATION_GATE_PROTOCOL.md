# ExecSurface Alpha.6 — Productization Gate Protocol

Date: 2026-10-08  
Release under test: `v0.1.0-alpha.6`  
Public moving channel: `v0.1`  
Scope: Linux x86_64 + native `ptrace`

## Classification

This gate evaluates product usability, execution semantics, evidence correctness, and release/consumer consistency.

It is **not cybersecurity research**.

Hostile/falsification tests in this gate are used to break product claims and user-path assumptions, not to perform penetration testing or malware/security research.

## Goal

A new user arriving at the repository must be able to move through this path without hidden semantic traps:

`landing page -> install -> doctor -> learn baseline -> unchanged check -> controlled REVIEW -> project init -> GitHub Action`

The gate is executable. Documentation claims alone cannot close it.

## Fixed roles

- Productization Lead — owns end-to-end user path.
- Innovation Lead — searches for high-leverage friction reduction without scope expansion.
- Anti-Drift Reviewer — blocks maturity inflation and preserves Alpha.6 semantic boundaries.
- Independent Falsifier — attacks onboarding assumptions and generated automation.
- Critical Reviewer — verifies exact release identity and final bounded claim.

## PZ0 — Source-of-truth freeze

Before any productization change:

- `main` is the source of truth;
- `v0.1.0-alpha.6` and `v0.1` must resolve to the accepted Alpha.6 release source;
- Alpha.5 remains immutable historical evidence;
- no historical issue/PR/comment/review/engagement record is rewritten.

## PZ1 — Landing-page truthfulness

The repository landing page must:

- identify Alpha.6 as the **current supported Alpha**, not a final/stable product;
- provide a direct no-Rust installation path;
- provide an exact crates.io path when registry publication exists;
- point to the Five-Minute Start;
- state the target-outcome boundary before a user can mistake PASS for target success.

No active/current guidance may present Alpha.5 as the current release.

## PZ2 — Install and identity

On a clean Linux x86_64 consumer:

- GitHub release archive exists;
- checksum verifies;
- binary reports `execsurface 0.1.0-alpha.6`;
- `doctor` succeeds inside the supported environment;
- stable `v0.1` resolves to the immutable Alpha.6 source.

The registry path must be tested separately and cannot be assumed from GitHub publication.

## PZ3 — First-run workflow

The documented five-minute path must execute:

1. learn a controlled baseline;
2. unchanged check -> PASS/0;
3. controlled drift -> REVIEW/10;
4. baseline bytes remain unchanged after checks.

No relearn-after-result is permitted.

## PZ4 — Real-project initialization

`execsurface init --command <cmd> --github-actions` must:

- never execute the target during init;
- generate policy + workflow without overwriting existing files unless explicitly forced;
- generate the stable `@v0.1` Action channel;
- generate a **separate target-command correctness step** before ExecSurface, because Alpha.6 target exit status is not verdict-bearing;
- use the same `/bin/bash -lc` wrapper for the ExecSurface command contract;
- require the externally anchored custody pins already qualified in Stage-2;
- print actionable next steps for baseline learning and trusted-variable setup.

A generated workflow that can appear green after the target command itself fails is a Productization Gate blocker.

## PZ5 — Action consumer path

The public `@v0.1` Action must prove, from a zero-contact consumer path:

- PASS;
- REVIEW;
- BLOCK;
- ERROR;
- custody-required behavior;
- evidence artifact outputs where enabled.

The Action may not silently build from an unreviewed branch or `main`.

## PZ6 — Documentation consistency

Current-facing documents must agree on:

- current release = Alpha.6;
- Alpha.5 = historical/rollback reference;
- supported platform = Linux x86_64;
- reference observer = native ptrace;
- normalization profile 4 for Alpha.6;
- profile-3 Alpha.5 baselines require explicit relearn rather than silent migration;
- PASS does not prove target-command success.

Historical Alpha.5 evidence is excluded from “stale current wording” failures when clearly marked historical.

## PZ7 — Destruction / friction corpus

At minimum challenge:

1. stale current-version text;
2. generated workflow without separate target gate;
3. missing custody setup instructions;
4. wrapper mismatch between learn and Action;
5. existing-file overwrite;
6. release/tag/stable-channel disagreement;
7. missing public asset/checksum;
8. unchanged path not producing PASS;
9. controlled drift not producing REVIEW/10;
10. Action failure semantics being weakened.

Any valid counterexample keeps the gate RED.

## PZ8 — Full qualification

Before closeout:

- Productization-specific tests PASS;
- CI PASS;
- Stage-2 Final Internal Gate PASS;
- Adversarial Regression PASS;
- P9.3 Compatibility PASS;
- Registry Packaging PASS;
- Public Consumer Smoke PASS;
- Technical Evaluation PASS where triggered;
- no predecessor assertion is weakened.

## Decision rule

The gate closes only as:

**ALPHA6_PRODUCTIZATION_GATE_PASS_BOUNDED**

This means Alpha.6 is coherent and usable for the documented supported path. It does **not** promote the release to v1.0, production-stable status, universal Linux support, or independent external validation.
