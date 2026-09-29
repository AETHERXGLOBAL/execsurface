# P4-B — Targeted Success-Authority Requalification

Status: **PREREGISTERED — IMPLEMENTATION MAY START ONLY WITHIN THIS BOUNDED RESEARCH GATE**

Parent program: #100  
Parent P4: #107  
Accepted predecessor: `P4_A2_PROPOSITION_GAPS_ESTABLISHED_BOUNDED`  
Accepted A2 source: `43d6c05e47b23b5a3f0a1ebb9f98558af22f4ea6`  
Accepted A2 artifact SHA-256: `af92dfac5c2cb0f4d58ab4536ea1e82be9dcdbbbb503520ad1c3ff37e9377e5d`

Public `v0.1.0-alpha.4`, stable `v0.1`, `main`, public raw-v2 bytes/semantics, and the default observer remain unchanged.

## Scientific question

Can **targeted ptrace syscall-exit/state evidence** close the three A2 success/observation authority gaps without introducing a second runtime backend and without laundering attempts, pending operations, ambiguous identity, or incomplete observations into successful effects?

A second backend is not the default implementation path. It becomes justifiable only proposition-by-proposition if ptrace fails a preregistered binding/completeness gate.

## Fixed roles

1. **Innovation Scientist / Linux Runtime Architect** — seek the smallest proof-bearing ptrace extension that closes the exact A2 gap without expanding product scope.
2. **Anti-Drift / Scientific Integrity Reviewer** — block success laundering, backend-first implementation, public-v2 reinterpretation, threshold changes, and unsupported equivalence claims.
3. **Independent Falsifier / Red Team** — attack syscall entry/exit pairing, EINTR/restart, EINPROGRESS, fd reuse, actor substitution, PATH-TOCTOU, clone/shared-fd interactions, failed rename/delete, observer loss and truncation.
4. **Independent Milestone Reviewer** — verify remote source, protocol freeze, exact proposition, test inventory, CI source, artifact digest and allowed decision before closure.

## Dynamic specialists

Select per sub-gate from:

- Linux ptrace syscall entry/exit semantics
- VFS/openat/openat2 and file-descriptor lifecycle
- renameat/renameat2/unlink/unlinkat/rmdir semantics
- Linux socket/connect state and asynchronous connect semantics
- process/thread/fd-table identity under clone/exec
- formal semantics / proposition-scoped evidence authority
- concurrency / restartable syscall behavior
- CI evidence sealing and reproducibility
- adversarial runtime / TOCTOU testing

## Non-negotiable boundaries

- no broad eBPF/BPF-LSM collector implementation in P4-B;
- no backend promotion based on name, event volume, maturity or performance;
- no public raw-v2 byte or meaning change;
- research evidence must be isolated from the default public path;
- syscall entry intent never implies syscall success;
- return value must be causally paired to the originating actor/syscall/arguments before it can strengthen authority;
- negative return values remain failure evidence, never success;
- `EINPROGRESS` is not successful connect authority;
- ambiguous or incomplete pairing is fail-closed;
- observer warnings/loss/truncation block dependent success authority;
- fd reuse must not transfer authority from an earlier operation;
- causal/actor substitution must change proposition identity or block authority;
- no test, threshold, workload or assertion may be weakened after observing a failure;
- all failures remain preserved.

## B0 — Evidence-interface contract freeze

Before collector experimentation, define an internal research-only evidence record capable of representing:

- originating TID/process identity;
- syscall family and normalized operation identity;
- syscall-entry arguments required by the target proposition;
- syscall-exit return value and errno interpretation;
- an explicit entry/exit pairing identity;
- fd/socket identity where applicable;
- observation-health/completeness dependencies;
- raw evidence digest/reference;
- proposition-specific authority outcome.

The record must distinguish at minimum:

- `AttemptObserved`
- `SuccessObserved`
- `FailureObserved`
- `PendingObserved`
- `Ambiguous`
- `Lost`

B0 passes only if invalid state combinations are structurally rejected and deterministic serialization is proved.

## B1 — `P4.FILE.OPEN_OBJECT`

### Hypothesis

Ptrace can establish bounded successful-open authority by pairing an open/openat/openat2 entry with its syscall-exit result and binding a non-negative returned fd to the exact originating actor and open attempt, even if no later covered I/O occurs.

### Required positive cases

- successful `open` returning fd;
- successful `openat` with resolved dirfd semantics;
- successful `openat2` within the already-supported argument scope;
- successful open followed immediately by close, with success retained as the open transition rather than inferred from later I/O.

### Required negative/adversarial cases

- `ENOENT`;
- `EACCES`/permission failure where reproducible;
- negative return in general;
- entry without matched exit;
- exit without matching entry;
- mismatched TID/actor;
- fd reuse after close;
- duplicate/replayed exit;
- PATH-TOCTOU substitution between lexical argument and later filesystem state;
- `CLOEXEC` and exec transition;
- clone/shared-fd interaction;
- observer loss/truncation.

### Acceptance

Only a causally paired non-negative syscall result may establish `P4.FILE.OPEN_OBJECT` success authority. Pathname evidence alone remains AttemptOnly.

## B2 — `P4.FILE.RENAME_DELETE`

### Hypothesis

Ptrace syscall-exit evidence can distinguish successful rename/delete effects from attempts without changing pathname attempt semantics.

### Required coverage

- rename/renameat/renameat2 where already observable;
- unlink/unlinkat/rmdir where already observable;
- rc=0 success;
- failed source/target path;
- permission/semantic failure where reproducible;
- actor/source/target substitution;
- `RENAME_NOREPLACE` / `RENAME_EXCHANGE` only if the current collector already exposes enough arguments; otherwise explicit unsupported state.

### Acceptance

Only an rc=0 result causally paired to the exact originating operation may strengthen the proposition to successful-effect authority. Failure remains explicit and cannot be collapsed into absence.

## B3 — `P4.NET.CONNECT_DESTINATION`

### Hypothesis

Ptrace can provide bounded immediate-connect success authority and explicit pending/failure states without treating asynchronous connect as success.

### Mandatory states

- immediate success (`rc=0`);
- immediate failure (`rc<0`, excluding pending semantics);
- pending (`EINPROGRESS` and any preregistered equivalent async state);
- ambiguous/lost pairing.

### Required attacks

- fd/socket reuse;
- wrong actor with same destination;
- destination substitution;
- duplicated/replayed syscall exit;
- `EINTR`/restart path;
- `EINPROGRESS` laundering attempt;
- observer loss/truncation;
- pending connection later becoming writable must **not** be called success unless a separate named bounded derivation is preregistered and falsified.

### Acceptance

P4-B may establish direct authority only for immediate successful connect. Pending remains a distinct non-success state unless a later separately gated derivation is proved.

## Cross-proposition falsification gate

The independent falsifier must demonstrate that:

1. entry-only evidence never becomes success;
2. failure never becomes success;
3. pending never becomes success by default;
4. mismatched actor/TID blocks pairing;
5. replay/duplicate exit blocks or deduplicates deterministically without authority inflation;
6. fd reuse cannot transfer object/socket identity;
7. observer incompleteness blocks dependent success authority;
8. pathname identity cannot substitute for kernel/object success evidence;
9. success for one proposition cannot satisfy another proposition;
10. deterministic serialization/digest remains stable.

Any false-success path closes the affected sub-gate as failed and blocks integration.

## Kill / escalation criteria

A proposition may justify a tiny second-backend prototype only if all are true:

1. ptrace has been tested under the frozen sub-gate;
2. the missing evidence is shown to be unavailable or non-bindable with sufficient correctness under ptrace;
3. the limitation is proposition-specific and documented;
4. no weaker semantic interpretation would answer the product question honestly;
5. the second backend is scoped only to the missing proposition evidence;
6. no public/backend promotion occurs automatically.

If ptrace can close the gap safely, a second backend is **not justified by that proposition**.

## Allowed outcomes

- `P4_B_PTRACE_SUCCESS_AUTHORITY_ESTABLISHED_BOUNDED`
- `P4_B_PARTIAL_PTRACE_GAPS_REMAIN`
- `P4_B_SECOND_BACKEND_PROTOTYPE_JUSTIFIED_BOUNDED`
- `P4_B_FALSE_AUTHORITY_PATH_FOUND`
- `P4_B_INCOMPLETE_EVIDENCE`

Outcomes must also be reported per B1/B2/B3; one proposition may not inherit another's result.

## Promotion boundary

No P4-B result by itself authorizes:

- public integration;
- raw-v2 reinterpretation;
- alpha.4 modification;
- stable `v0.1` movement;
- backend promotion;
- broad eBPF/BPF-LSM implementation;
- universal runtime completeness/security claims.

Any later product integration requires a separate compatibility/release gate.