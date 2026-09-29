# ExecSurface — P4-A1.2 Ptrace Evidence Mapping Protocol

Date: 2026-09-29
Tracking: #108 / #100
Predecessor: `P4_A1_1_AUTHORITY_MODEL_PASS_RESEARCH_ONLY`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — MAPPING IMPLEMENTATION MAY START**

## Question

Can current raw-v2 ptrace evidence be translated into the canonical P4 proposition/authority model without promoting attempts into successful effects, hiding incomplete evidence, or claiming unsupported successful-open / fd-table facts?

## Pre-implementation representability audit

Current raw-v2 provides:
- `ProcessSpawn` from ptrace lifecycle events;
- `ProcessExec` emitted only on confirmed ptrace exec transition;
- pathname/open/delete attempts recorded from userspace arguments before kernel result;
- FD-attributed read/write emitted only after positive-byte syscall result and bound through tracked runtime FD state;
- rename paths recorded at syscall entry (attempt semantics);
- connect destination recorded from syscall-entry sockaddr (attempt semantics);
- observation-wide `complete` plus warnings.

Important gaps:
1. raw-v2 does not emit a first-class successful-open object event. Internal ptrace state tracks successful opens for subsequent FD attribution, but a raw-v2 pathname-open event itself is still an attempt. Therefore `P4.FILE.OPEN_OBJECT` is **unsupported from raw-v2 alone** unless a future explicit proof source is supplied.
2. raw-v2 intentionally does not retain exact clone/clone3 `CLONE_FILES` evidence. Therefore `P4.FDTABLE.RELATION` is **unsupported/unknown from raw-v2 alone**. The P3 research certificate remains separate and is not silently imported.
3. Semantics-v3 prototype lacks first-class proposition variants for successful-open object, observer health/loss, and fd-table relation. These research-only proposition variants must be added before A1.2 mapping; no dummy proposition may be used.

## Frozen mapping rules

For admitted complete/warning-free ptrace raw-v2 evidence:
- `ProcessSpawn` -> `P4.PROC.CREATE_RELATION`, direct lifecycle evidence;
- `ProcessExec` -> `P4.EXEC.SUCCESS`, direct successful lifecycle transition evidence;
- ordinary `FilePathAccess` / `FileOpenAt2` -> `P4.PATH.ACCESS_ATTEMPT`, `attempt_only`;
- canonical read/write whose target resolution is `KernelFdResolved` -> `P4.FD.IO_ATTRIBUTION`, bounded direct runtime-FD evidence;
- `FileRename` -> `P4.FILE.RENAME_DELETE` only as rename **attempt** evidence, never successful-result authority;
- `NetworkConnectAttempt` -> `P4.NET.CONNECT_DESTINATION` only as attempt evidence;
- execution-chain data may support `P4.CAUSAL.EXEC_LINEAGE` only as bounded derivation with explicit derivation identity;
- observer health is emitted as a first-class record;
- successful-open object and exact fd-table relation remain explicit unsupported records from raw-v2 alone.

For incomplete or warning-bearing observations:
- no effect record may be `direct+complete`;
- observer health/loss record must state the incomplete condition/reason codes;
- mapping may fail closed rather than canonicalize an incomplete observation.

## Evidence identity

The adapter computes a SHA-256 digest over deterministic JSON serialization of the supplied raw observation. Every mapped record references that digest. Derived causal records additionally name their derivation rule.

## Mandatory A1.2 falsification tests

1. failed/open pathname attempt cannot become successful-open object authority;
2. pathname lexical evidence cannot satisfy kernel-object requirement;
3. FD read/write with `KernelFdResolved` can satisfy only the declared FD-attribution proof requirement;
4. rename entry evidence remains attempt-only;
5. connect entry evidence remains attempt-only;
6. confirmed exec maps to success; no pathname exec-attempt surrogate is accepted;
7. incomplete observation emits no admissible complete effect records;
8. warning-bearing observation cannot silently retain complete session authority;
9. raw-v2 clone observation cannot establish exact fd-table relation;
10. deterministic mapping output/digest is stable under deterministic input serialization;
11. causal-lineage record requires named derivation and exact chain evidence;
12. public raw-v2 structures and public M11 behavior remain unchanged.

## Allowed A1.2 outcomes

- `P4_A1_2_PTRACE_MAPPING_PASS_RESEARCH_ONLY`
- `P4_A1_2_FALSE_AUTHORITY_PATH_FOUND`
- `P4_A1_2_SEMANTICS_GAP_BLOCKED`
- `P4_A1_2_PUBLIC_CONTRACT_REGRESSION`
- `P4_A1_2_INCOMPLETE_EVIDENCE`

Only PASS_RESEARCH_ONLY may authorize A1.3 adversarial integration closure. No new backend is authorized by A1.2.
