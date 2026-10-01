# M10.4 — Hybrid Exec Authority Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m104`
Protocol: `docs/milestones/M10_4_EXEC_AUTHORITY_PROTOCOL.md`

## Classification

`M10_4_HYBRID_EXEC_MATCH`

## Accepted execution

Workflow run: `36438088623`  
Job: `108981122627`  
Source SHA: `a1c9e3b3158f666847a5692c21ad0f78b4357eb0`  
Artifact: `10976148266`  
Artifact digest: `sha256:830a7c495c7b8ff851dcf000307cb86b6368ee09db86670f96f612bad83ae087`

Workflow conclusion: `success`.

## Frozen evidence

Guest environment:
- active LSM list: `lockdown,capability,bpf`;
- BPF LSM active;
- kernel BTF readable;
- probe completed.

Successful role ground truth:
- `ground_dev=2`;
- `ground_ino=14`;
- `ground_ok=1`.

Successful role BPF evidence:
- `success_candidates=1`;
- `success_confirms=1`;
- candidate `(dev=2, ino=14)`;
- success-confirm `(dev=2, ino=14)`;
- target exit `0`.

Failed role evidence:
- `fail_candidates=0`;
- `fail_confirms=0`;
- `fail_errno=2` (`ENOENT`);
- target exit `0`.

Health evidence:
- `malformed=0`;
- `wrong_session=0`;
- `unknown_role=0`;
- `drops=0`.

Frozen evaluator emitted:

`M10_4_HYBRID_EXEC_MATCH`

## Interpretation

Within the declared bounded fixture, the hybrid design separated pre-commit exec mediation from successful exec confirmation and grounded the successful executable in the same kernel object identity independently measured by target-side `fstat(2)`.

The failed exec role produced no success confirmation and was not promoted to successful-exec evidence.

## What this proves

For this fixture:
- BPF LSM can provide object-grounded executable candidate identity;
- `sched_process_exec` can provide a distinct success-confirmation layer;
- the two signals can be correlated by session/role without recorded loss;
- success vs failure can be represented without treating an LSM attempt as success.

## What this does not prove

This result does **not** establish:
- universal exec coverage across scripts/interpreters/binfmt_misc;
- namespace/container equivalence;
- zero loss under load;
- acceptable production overhead;
- portability beyond the tested kernel/harness;
- public BPF PASS authority;
- `learn/check` authority;
- baseline interchangeability;
- backend auto-selection;
- ptrace replacement.

## Decision

M10.4 is **CLOSED / MATCH**.

Authorized next step only: preregister and execute M10.5 CONNECT authority, requiring a kernel-derived destination plus an independent successful-connect confirmation and explicit failed-connect non-promotion.

No change to `main`, `v0.1.0-alpha.3`, Marketplace behavior, public ptrace authority, or baseline semantics is authorized.