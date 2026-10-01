# M10.3D — Protocol-Faithful BPF-LSM File Authority Result

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103d`
Protocol: `docs/milestones/M10_3D_PROTOCOL_FAITHFUL_FILE_AUTHORITY_PROTOCOL.md`

## Classification

`M10_3_OBJECT_IDENTITY_MATCH`

## Accepted execution

Workflow run: `36436257545`  
Job: `108974845199`  
Source SHA: `a2d20ce79b480681f806e25791954b37223db30d`  
Artifact: `10976285106`  
Artifact digest: `sha256:24dc490dbc8cb572f17ef4d4c2a760b759065ac614dda4c89006297967326e96`

Workflow conclusion: `success`.

## Frozen evidence

Guest environment:

- kernel `6.8.0-142-generic`;
- active LSM list `lockdown,capability,bpf`;
- BPF LSM active;
- kernel BTF readable;
- probe completed.

Target-side ground truth from `fstat(2)` on the exact returned fd:

- `ground_dev=2`;
- `ground_ino=21`;
- `ground_ok=1`.

Accepted BPF-LSM event:

- `event_dev=2`;
- `event_ino=21`;
- `event_tgid=87`;
- `events_total=1`;
- `malformed=0`;
- `wrong_session=0`;
- `drops=0`.

Target:

- `target_exit=0`;
- `identity_match=1`.

Frozen evaluator emitted:

`M10_3_OBJECT_IDENTITY_MATCH`

## What this proves

Within the declared one-target / one-open controlled fixture, the audit-only BPF LSM `file_open` hook emitted a unique session-scoped kernel object identity whose raw `(s_dev, i_ino)` exactly matched target-side `fstat(2)` ground truth for the returned fd, with zero recorded producer drops and no malformed or wrong-session events.

This is evidence that a kernel-hook observer can provide stronger object-grounded authority for this bounded file-open proposition than the ptrace entry-time pathname metadata disproved by M10.1.

## What this does not prove

This result does **not** establish:

- pathname equivalence or canonical path identity;
- universal file-operation coverage;
- fd read/write lifecycle equivalence;
- exec or network semantics;
- zero loss outside this controlled fixture;
- production portability;
- acceptable performance or packaging cost;
- public BPF-LSM PASS authority;
- `learn/check` authority;
- ptrace/BPF-LSM baseline interchangeability;
- backend auto-selection;
- ptrace replacement.

## Decision

M10.3D is **CLOSED / MATCH**.

Authorized next step only: preregister and execute M10.4, testing whether a hybrid exec architecture can distinguish **exec attempt** from **successful exec** while grounding accepted executable identity in kernel objects and preserving explicit loss/session semantics.

No change to `main`, `v0.1.0-alpha.3`, Marketplace behavior, public ptrace authority, or current baseline semantics is authorized.