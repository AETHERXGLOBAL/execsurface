# ExecSurface — P4 Current State

Date: 2026-09-29
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`

## Current status

- P3 closed: `P3_EPHEMERAL_ONLY_VALUE_ESTABLISHED`.
- P4 master protocol preregistered.
- P4-A0 proposition inventory frozen.
- P4-A0 independent review matrix preregistered and reviewed.
- P4-A0 decision: `P4_A0_PROPOSITION_CONTRACT_ACCEPTED_BOUNDED`.
- P4-A1 ptrace reference-adapter protocol preregistered.
- P4-A1 implementation is the only currently authorized implementation step.

## Public boundary

Unchanged:
- release `v0.1.0-alpha.4`;
- stable Action `@v0.1`;
- public/default ptrace raw-v2 behavior;
- public M11 fail-closed semantics;
- `main` contains no P3/P4 runtime integration.

## Next action

Implement P4-A1.1 research-only proposition record types + deterministic serialization, then run the frozen type-level falsification tests before binding records to ptrace evidence.
