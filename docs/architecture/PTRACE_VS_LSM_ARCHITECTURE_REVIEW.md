# PTRACE vs LSM Architecture Review

Date: 2026-09-28  
Tracking: #87  
Source-of-truth HEAD at review start: `99d5f7c71c7cec59f5a1fa1cd9ef731e8212a669`  
Classification: **HYBRID_ARCHITECTURE_RECOMMENDED**

## 0. Executive decision

The external criticism is technically substantive.

It does **not** establish that SELinux or AppArmor are drop-in replacements for ExecSurface's current observer. ExecSurface is primarily an **evidence-generation system built on observation**, while SELinux/AppArmor are primarily **policy-mediation/enforcement systems built on LSM hooks**.

However, the criticism exposes a real architectural weakness in treating userspace `ptrace` as the strongest possible evidence authority for all observed object identity. `ptrace` gives ExecSurface a useful combination of per-command scope, descendant following, syscall entry/exit pairing, selected metadata reads, fd lifecycle modeling, and low-friction self-service deployment. It also imposes material stop/resume cost and creates observation/identity races that a kernel-hook observer can avoid for some event classes.

The correct conclusion is therefore neither "ptrace is wrong" nor "LSM is irrelevant".

**Decision: `HYBRID_ARCHITECTURE_RECOMMENDED`.**

- Keep `ptrace` as the public `v0.1.0-alpha.3` correctness-reference backend for its **currently bounded semantics**.
- Do not claim ptrace-derived metadata is universally kernel-authoritative.
- Open a new architecture milestone to test a **kernel-hook evidence path**, prioritizing BPF LSM + stable tracepoints (and only then other primitives) as an additive backend.
- Use ptrace as a differential/reference oracle until proposition-level parity and stronger authority are proved.
- Do not grant a new backend PASS, `learn/check`, baseline interchangeability, or automatic selection until explicit evidence gates close.

No runtime semantics are changed by this review.

---

## 1. Review method and team

The review began by trying to **kill** the hypothesis:

> `H0: ptrace is the correct correctness-reference mechanism for the current ExecSurface evidence contract.`

### Fixed roles

- **Innovative Systems Architect** — search for mechanisms that can provide stronger object-grounded evidence or lower perturbation than current ptrace.
- **Anti-Deviation / Skeptical Reviewer** — reject defenses based on sunk cost, existing code, historical tests, or installed-base inertia.

### Dynamic specialists

- Linux kernel / LSM architecture
- SELinux / AppArmor policy mediation
- seccomp and userspace notification
- eBPF / BPF LSM / tracepoints
- fanotify / Linux Audit / Landlock
- ptrace lifecycle and signal semantics
- namespaces / containers / privilege boundaries
- Rust systems implementation
- evidence semantics / comparability / fail-closed behavior
- Marketplace / release compatibility

---

## 2. Why ExecSurface currently uses ptrace

The historical reason is narrower than "ptrace is the best Linux security mechanism".

`ADR-0002` selected native ptrace after the original strace path failed the metadata-only privacy invariant. Native ptrace allowed ExecSurface to:

1. launch the declared target itself;
2. follow fork/vfork/clone descendants;
3. stop at syscall entry/exit;
4. read only selected pointer targets needed for the evidence contract;
5. avoid dereferencing argv/envp and avoid generic process-memory capture;
6. pair successful syscall returns with later fd/lifecycle effects;
7. fail explicitly on unsupported observer conditions.

By M6.5 the reference contract had expanded to include:

- syscall entry/exit pairing;
- successful open -> fd identity;
- actual fd-attributed read/write;
- fd duplication/close lifecycle;
- fork inheritance;
- `CLONE_FILES` sharing;
- close-on-exec handling;
- `openat/openat2` trace-time path handling;
- explicit incompleteness and fail-closed behavior.

This is why ptrace remained the **reference**, not because it was proved optimal.

Repository evidence:

- `docs/architecture/ADR-0002-metadata-only-ptrace.md`
- `docs/milestones/M6_5_BACKEND_DECISION.md`
- `crates/execsurface-observe/src/linux_ptrace.rs`

---

## 3. What property do we actually need from ptrace?

There is no single property. The current observer depends on a bundle:

### P1 — Per-command scoped lifecycle

Start one declared command and follow its descendant process/thread topology without requiring ambient host-wide telemetry.

### P2 — Entry/exit correlation

See both syscall arguments/intention at entry and success/failure/result at exit.

### P3 — Selective metadata access

Read pathname/sockaddr/open metadata while deliberately refusing argv/envp/content/payload capture.

### P4 — FD-state reconstruction

Correlate successful opens with fd identity, then track dup/close/fork/`CLONE_FILES`/CLOEXEC semantics so later I/O can be attributed.

### P5 — Explicit health/completeness

If the observer cannot preserve the declared proposition, evidence must become incomplete/error rather than silently PASS.

### P6 — Self-service deployment

Work for a normal launched command on supported Linux x86_64 environments without requiring installation of a host MAC policy or privileged persistent daemon.

An alternative backend does **not** need to imitate ptrace. It needs to prove the propositions represented by P1–P6 or explicitly declare a narrower capability set.

---

## 4. Layer separation

| Layer | Meaning in ExecSurface | Typical mechanism |
|---|---|---|
| **Observation** | Collect facts/events about a target execution | ptrace, tracepoints/eBPF, audit, fanotify |
| **Syscall interception** | Stop/filter/redirect at the userspace→kernel syscall boundary | ptrace, seccomp (`TRACE`/`USER_NOTIF`) |
| **Policy mediation** | Evaluate a security decision at a kernel security hook against a kernel object/credential context | LSM, SELinux, AppArmor, Landlock, BPF LSM |
| **Enforcement** | Prevent/allow an operation or constrain future behavior | SELinux, AppArmor, Landlock, seccomp, LSM/BPF-LSM policy |
| **Evidence generation** | Turn typed observations + health into canonical surface, baseline, diff, policy verdict and report | ExecSurface core |

ExecSurface's distinguishing layer is **evidence generation**. Its current observer happens to use syscall interception/tracing. SELinux/AppArmor principally solve policy mediation/enforcement.

Therefore the comparison is indirect but still important: LSM hooks may be **better observation anchor points** for some evidence propositions because they see kernel-resolved objects closer to the authoritative decision point.

---

## 5. Practical comparison

| Mechanism | Primary layer | Per-command descendant scope | Entry + exit result | Kernel-resolved object context | File coverage | Network/process coverage | Can enforce | Event-loss/race model | Privilege / deployment | Fit for ExecSurface |
|---|---|---:|---:|---:|---|---|---:|---|---|---|
| **ptrace (current)** | syscall interception / observation | **Strong** for launched descendants | **Strong** | **Mixed** — raw args at entry; some post-success fd resolution | selected syscall + fd lifecycle | spawn/exec/connect selected | technically can alter tracee, but ExecSurface does not use it as enforcement | scheduling perturbation; userspace-pointer and shared-fd races; selected-syscall gaps | ptrace policy/Yama/user-ns dependent; no persistent privileged daemon in normal case | **Current reference; useful but not strongest authority** |
| **LSM framework** | policy mediation | Not an event stream by itself | Hook-specific, not syscall entry/exit pairing | **Strong at security hook/object level** | broad hook surface | process/file/socket hooks | **Yes** | hook semantics are kernel-local; userspace export must define loss/health | kernel integration / LSM configuration | **Strong anchor architecture, not a drop-in collector** |
| **SELinux** | MAC policy mediation/enforcement | Host/domain policy scoped, not ExecSurface session scoped by default | No generic syscall-pair stream | **Strong policy/object context** | strong access-control mediation | strong security policy coverage | **Yes** | audit output depends on policy/audit decisions, not neutral full event stream | requires SELinux enabled + policy | **Not a drop-in replacement; useful external policy context** |
| **AppArmor** | MAC profile mediation/enforcement | profile/task scoped, not ExecSurface session model | No generic syscall-pair stream | kernel mediation with profile/path-oriented policy | strong mediated filesystem operations | policy-dependent | **Yes** | policy/audit dependent | AppArmor enabled + profile loaded | **Not a drop-in replacement; same layer issue as SELinux** |
| **BPF LSM** | programmable LSM mediation/audit | Can add explicit session membership maps | Hook-specific; combine with tracepoints for lifecycle/returns | **Potentially strong**: hook receives kernel objects/credentials | strong candidate for open/permission/exec object evidence | candidate hooks for exec/socket/security events | Can enforce, but an ExecSurface observer should initially return allow and audit only | ring-buffer/map loss and lifecycle must be measured/fail-closed | privileged on many hosts; kernel/BTF/LSM availability; packaging complexity | **Best candidate for stronger hybrid evidence authority** |
| **eBPF tracepoints/fentry** | kernel observation | Feasible; repository has bounded prototype | event-specific; not inherently syscall-pair semantics | depends on attachment point | feasible but path/fd reconstruction hard | strong lifecycle tracing | generally observation-only unless combined with policy mechanisms | producer/ring-buffer loss, routing/session races | privilege/kernel/version constraints | **Useful hybrid component; current repo proves only bounded parity** |
| **seccomp filter** | syscall filtering/enforcement | inherited by descendants | pre-execution filter; not normal post-success object stream | raw syscall data, not resolved kernel objects | syscall-number based | syscall-number based | **Yes** | target state is changed by installed filter | `no_new_privs`/capability semantics; inherited filter | **Poor as neutral observer; useful only as bounded selective-stop experiment** |
| **seccomp user notification** | syscall mediation to userspace | inherited filter can scope descendants | intercepts before execution; supervisor decides/continues | raw args; pointer reads have explicit TOCTOU risk | selected syscalls | selected syscalls | **Yes/mediate** | notification abort/signal behavior; pointer TOCTOU; supervisor races | changes target syscall filtering state | **Not a stronger evidence reference by itself** |
| **fanotify** | filesystem notification/permission mediation | process IDs may be available only under privilege; mark scoped | event/permission based, not generic syscall pairing | file object/file-handle oriented | **Good but file-only**; misses mmap-induced access | no complete process/network model | permission events can enforce | queue can overflow; documented mmap/msync/munmap blind spots | powerful modes require `CAP_SYS_ADMIN`; mark/mount constraints | **Possible file-evidence complement, not full backend** |
| **Linux Audit** | kernel audit event generation | filters can approximate target criteria but are system policy/rule based | syscall exit rules + other records | kernel audit context | broad, includes filesystem and io_uring rule paths | broad audit event families | not the primary enforcement mechanism | kernel backlog/internal queue can overflow; rule cost | normally system/root configuration + auditd | **Useful validation/cross-check, poor self-service default** |
| **Landlock** | unprivileged stackable LSM sandbox | self + descendants | no neutral observation stream | kernel access-control objects | filesystem + bounded network restrictions | restricted by Landlock ABI | **Yes, restrictive only** | changes allowed behavior by design | designed for unprivileged self-restriction | **Potential optional enforcement mode, not baseline observer** |

---

## 6. Why SELinux/AppArmor are not direct replacements

The Linux Security Module framework is primarily an access-control hook framework. SELinux and AppArmor load policy into those hooks and decide whether operations are allowed under their respective models.

ExecSurface currently needs a neutral-ish per-command experiment:

```text
run target -> collect bounded evidence -> canonicalize -> learn/check -> diff -> policy verdict
```

Replacing the observer with SELinux/AppArmor would introduce host policy state into the experiment and still would not automatically provide:

- ExecSurface session/epoch membership;
- the exact descendant topology model;
- current fd lifecycle reconstruction;
- an event for every proposition ExecSurface currently records;
- portable GitHub-hosted self-service behavior;
- a backend-independent completeness contract.

Therefore "use SELinux/AppArmor" is not sufficient as an implementation proposal.

But "move authority-sensitive evidence closer to LSM/kernel-object hooks" **is** a valid architectural proposal.

---

## 7. Why BPF LSM is materially different

Linux documents BPF LSM as allowing privileged runtime instrumentation of LSM hooks for system-wide MAC and Audit policies.

That creates a useful design point:

```text
stable process lifecycle tracepoints
        +
BPF LSM security hooks for object-grounded file/exec/socket facts
        +
explicit session membership + loss accounting
        +
existing ExecSurface backend capability/comparability gate
```

Potential advantage over ptrace:

- evidence can be emitted at a kernel hook after the kernel has resolved security-relevant objects;
- no per-syscall stop/resume tax for unrelated syscalls;
- no need to infer some successful object identities from a later `/proc/<tid>/fd/N` snapshot;
- less target scheduling perturbation from tracing stops.

Not proved yet:

- full path identity under all mount/namespace/rename semantics;
- fd lifecycle/read-write equivalence;
- zero-loss session transport;
- GitHub-hosted privilege/packaging viability;
- same privacy contract;
- same or lower end-to-end cost;
- proposition-level baseline equivalence.

The repository's existing eBPF work already proves bounded lifecycle parity but explicitly does **not** prove file/fd/path/network equivalence or public correctness authority.

---

## 8. Known ptrace risk analysis

### 8.1 Performance — REAL / MEASURED

Post-M9 exact-host evidence measured:

- pinned ripgrep direct median ~113.9 ms vs ptrace ~514.7 ms (`4.52x`);
- pinned fzf direct median ~113.8 ms vs ptrace ~364.3 ms (`3.20x`);
- adding irrelevant `getpid` syscalls produced exactly two additional syscall-info stops each and near-linear additional overhead (`R^2 ~ 0.99955`).

Conclusion: all-syscall stop/resume tax is a causal cost component, not speculation.

### 8.2 Userspace pointer TOCTOU — REAL ARCHITECTURAL RISK

At syscall-entry stop, ExecSurface may read a pathname/sockaddr/open metadata pointer from tracee memory. The stopped thread is paused, but another thread sharing the same address space can mutate that memory before the kernel consumes it when the stopped thread resumes.

Counterexample:

1. Thread A calls `openat(dirfd, shared_buffer, ...)` and stops at syscall entry.
2. Observer copies `shared_buffer == "allowed.txt"`.
3. Thread B changes the shared buffer to `"different.txt"`.
4. Thread A resumes; kernel consumes the changed value.

The observer's `path access intent` string can therefore differ from the kernel-consumed pathname.

This is a critical distinction between **observed userspace argument metadata** and **kernel-authoritative object identity**.

### 8.3 Shared fd-table race — REAL ARCHITECTURAL RISK

After a successful open returns fd `N`, ExecSurface may resolve `/proc/<tid>/fd/N`. With `CLONE_FILES`, another thread can close/dup/reuse that fd while the opening thread is stopped. Unless all mutations are atomically excluded, a procfs snapshot can theoretically identify a later object rather than the object returned by the original open.

The existing fd-lifecycle state machine reduces ambiguity but does not turn procfs lookup into an atomic kernel object capture.

### 8.4 Scheduling perturbation — ACKNOWLEDGED

Ptrace stops/resumes tracees and can materially change scheduling/timing. Trace-aware programs can behave differently under tracing. This is already an explicit repository limitation.

### 8.5 Multithreading / clone semantics — ACKNOWLEDGED

The observer tracks clone/fork/vfork and `CLONE_FILES`, but the current implementation can mark observation incomplete when clone flags are unavailable. Exec/de-threading and concurrent shared-state changes remain complex.

### 8.6 Signals — MATERIAL COMPLEXITY

Ptrace interposes on signal-delivery/group-stop behavior. Correct signal reinjection and distinction between syscall stops, ptrace events, group stops and exits is part of tracer correctness. A bug can change target behavior, not merely lose telemetry.

### 8.7 Namespaces / containers — BOUNDED PORTABILITY

Ptrace permission checks depend on credentials, user namespaces, capabilities and LSM policy. Yama can restrict `PTRACE_TRACEME`/attach behavior; container seccomp/capability policy can also prevent tracing. A supported run must fail explicitly rather than downgrade to PASS.

### 8.8 Privilege boundaries — REAL LIMIT

Linux deliberately restricts ptrace because tracing can expose/control sensitive process state. `CAP_SYS_PTRACE`, dumpability, namespace relations and Yama affect availability. This makes ptrace unsuitable as a universal host observation mechanism.

### 8.9 Completeness blind spots — CURRENTLY DOCUMENTED

The public ptrace observer is selected-metadata observation, not universal behavior tracing. Current repository limitations already state:

- memory-mapped I/O is not attributed as fd read/write;
- io_uring data access is not attributed by the current fd I/O model;
- path-based open/create/delete/rename records include syscall-attempt semantics;
- hostname intent is not inferred from `connect(2)`;
- only selected syscall families are represented.

These are valid bounded semantics only if public claims stay equally bounded.

---

## 9. Counterexamples that kill an over-strong ptrace claim

The following propositions are **KILLED** if stated without qualification:

### C1 — "The pathname recorded at syscall entry is exactly the pathname consumed by the kernel"

Killed by shared userspace-buffer TOCTOU in a multithreaded process.

### C2 — "A post-open procfs fd snapshot is atomically identical to the file object returned by open"

Killed in principle by concurrent shared-fd-table close/dup/reuse races.

### C3 — "ExecSurface observes all file reads/writes"

Killed by current mmap and io_uring attribution gaps.

### C4 — "Tracing does not change the execution being measured"

Killed by ptrace scheduling/stop semantics and trace-aware behavior.

### C5 — "ptrace works for every Linux/container privilege configuration"

Killed by ptrace access controls, Yama modes, capabilities/user namespaces and container policy.

### C6 — "A traced command includes all causally involved external services"

Killed when the command communicates with a pre-existing daemon/service outside the traced descendant tree. ExecSurface observes the target-side interaction it supports, not the daemon's independent execution surface.

### C7 — "Current ptrace evidence is equivalent to kernel security mediation evidence"

Killed: syscall-boundary observation and LSM object-level mediation are different evidence authorities.

---

## 10. Evaluation of alternatives

### 10.1 Generic LSM / custom kernel module

**Stronger potential authority, unacceptable current product burden.**

A custom LSM could observe/mediate near authoritative kernel objects, but kernel integration, deployment, compatibility and privilege requirements are incompatible with the current self-service Marketplace alpha. It is a research reference, not the immediate product path.

### 10.2 SELinux

**Reject as direct replacement.**

SELinux is a system MAC policy engine. Its audit records are policy/audit dependent and its deployment would couple ExecSurface evidence to host security policy. It can be used as external corroborating context, not as the current neutral per-command observer.

### 10.3 AppArmor

**Reject as direct replacement.**

Same category: profile-based MAC enforcement/mediation. Useful for policy integration, not a complete ExecSurface evidence stream.

### 10.4 seccomp

**Reject as primary evidence backend.**

seccomp is designed to reduce/filter the syscall surface. User notification can mediate syscalls but changes target state and still exposes raw register arguments; kernel documentation explicitly warns about TOCTOU when a supervisor reads tracee memory. A seccomp-assisted selective ptrace experiment remains possible, but must prove target-state transparency.

### 10.5 fanotify

**Reject as complete backend; retain as possible file-object cross-check.**

fanotify has useful file/permission events and file-handle/object properties, but it is file-only, has queue-overflow semantics, privilege limits for richer modes, and documented mmap/msync/munmap gaps.

### 10.6 Linux Audit

**Reject as default self-service backend; retain as validation oracle.**

Linux Audit can record syscall/filesystem/io_uring-related events and kernel audit context, but uses system-level rules/daemon/backlog state, can lose records under queue pressure depending configuration, and introduces host-global operational coupling.

### 10.7 Landlock

**Reject as observation backend; retain as possible future optional enforcement layer.**

Landlock is a stackable self-restriction LSM. Its value is sandboxing/enforcement. Applying it during `learn/check` would change the target's allowed behavior and therefore change current semantics.

### 10.8 BPF LSM + tracepoints

**PROMOTE to architecture milestone candidate.**

This is the strongest identified path for improving evidence authority without converting ExecSurface into SELinux/AppArmor:

- tracepoints for lifecycle/session membership;
- BPF LSM hooks for security-relevant object evidence;
- optional syscall tracepoints only where return/intent evidence is still needed;
- explicit ring-buffer/loss/epoch health;
- existing capability and comparability gates retained.

It is not authorized for product integration by this review.

---

## 11. Impact of the recommendation

### Current semantics

**No immediate change.**

`v0.1.0-alpha.3` remains scoped to observed execution-surface drift under the recorded observer and policy. The review requires tighter wording around authority:

- syscall-entry path/sockaddr data = observer-copied userspace argument metadata;
- successful kernel object identity derived through fd/procfs = runtime evidence with documented concurrency limits;
- neither should be described as equivalent to LSM mediation evidence.

### Backward compatibility

**Preserved.**

No lock schema, baseline, policy, exit code, CLI, Action input, or report semantics change in this review.

A future kernel-hook backend must use versioned backend identity/capability metadata and cannot silently reuse ptrace baselines unless parity is explicitly proved.

### Evidence authority

**Needs refinement, not invalidation.**

Existing ptrace evidence remains authoritative only for its declared observer semantics. The architecture milestone should define evidence-authority classes, e.g.:

- `USER_ARG_OBSERVED`
- `SYSCALL_RESULT_OBSERVED`
- `PROCFS_RUNTIME_OBJECT_RESOLVED`
- `KERNEL_HOOK_OBJECT_OBSERVED`

Names are provisional; no schema change is authorized here.

### Marketplace users

**No break / no migration.**

Current Action remains ptrace-backed. No privileged daemon, SELinux/AppArmor policy, BPF requirement, or new permission is introduced into the current alpha.

### Current alpha release

**No release withdrawal required.**

The current release already makes bounded claims and fails closed on known observer incompleteness. This review does not prove a silent universal false-PASS bug in the shipped contract. It does prove that documentation must not imply stronger kernel-authoritative coverage than the observer can establish.

---

## 12. Required new milestone

Because a real architecture gap is established, open a new milestone before implementation:

**M10 — Kernel-Hook Evidence Authority / Hybrid Observer Gate**

### Goal

Determine whether BPF LSM + stable lifecycle tracepoints can provide stronger object-grounded evidence for selected propositions while preserving:

- metadata-only privacy;
- per-command/session containment;
- explicit loss accounting;
- fail-closed completeness;
- deterministic normalization;
- baseline comparability boundaries;
- no silent privilege escalation;
- practical CI/Marketplace deployment.

### First mandatory experiments

1. **PATH-TOCTOU-001** — adversarial multithreaded pathname-buffer mutation; compare ptrace entry string vs kernel-consumed object identity.
2. **FD-SHARE-RACE-001** — shared `CLONE_FILES` close/reuse adversary; test successful-open identity attribution.
3. **BPF-LSM-FILE-001** — audit-only BPF LSM file/open proposition with explicit object identity and no enforcement.
4. **BPF-LSM-EXEC-001** — exec object/credential proposition correlated with lifecycle tracepoints.
5. **BPF-LSM-CONNECT-001** — socket-connect mediation-point proposition vs ptrace sockaddr observation.
6. **LOSS-AUTHORITY-001** — force event transport loss; prove no PASS-capable evidence survives undetected.
7. **PRIVILEGE-MATRIX-001** — GitHub-hosted, container, user-namespace, Yama/LSM/BPF capability matrix.

No new backend receives PASS authority until these and subsequent parity gates close.

---

## 13. External technical criticism verdict

### Did the criticism reveal a real problem?

**YES — but the problem is more precise than "ptrace should have been SELinux/AppArmor."**

The real issue is that ptrace lives at a userspace syscall-tracing boundary and can require snapshots/inference where kernel security hooks can observe resolved objects closer to the authoritative operation. This matters for TOCTOU, shared state and evidence authority.

### Is current ExecSurface architecture invalid?

**NO.**

The backend abstraction, capability model, incompleteness states and cross-backend non-equivalence rules are already compatible with adding a stronger observer. The current alpha is valid only under its bounded observer semantics.

### Is documentation-only clarification sufficient?

**NO.**

Documentation tightening is required immediately, but the identified object-authority races justify a new architecture milestone.

### Should ptrace be removed now?

**NO.**

There is no proved replacement with equivalent product usability and evidence coverage. Existing eBPF evidence demonstrates bounded lifecycle parity only and does not establish full file/fd/path/network equivalence.

### Final classification

# `HYBRID_ARCHITECTURE_RECOMMENDED`

Keep ptrace as the current alpha reference while developing and adversarially testing kernel-hook evidence for the propositions where ptrace is weakest. Promote only proposition-by-proposition, with explicit evidence authority and no baseline interchangeability by assumption.

---

## 14. Evidence references

### Repository evidence

- `docs/architecture/ADR-0002-metadata-only-ptrace.md`
- `docs/milestones/M6_5_BACKEND_DECISION.md`
- `docs/milestones/M8_EBPF_ARCHITECTURE.md`
- `docs/milestones/POST_M9_PTRACE_COST_ATTRIBUTION_RESULT.md`
- `docs/milestones/POST_M9_EBPF_E3_SHARED_LIFECYCLE_PARITY_RESULT.md`
- `crates/execsurface-observe/src/lib.rs`
- `crates/execsurface-observe/src/linux_ptrace.rs`

### Linux/kernel primary documentation

- Linux Security Modules: https://docs.kernel.org/security/lsm.html
- LSM userspace API: https://docs.kernel.org/userspace-api/lsm.html
- BPF LSM programs: https://docs.kernel.org/bpf/prog_lsm.html
- AppArmor: https://docs.kernel.org/admin-guide/LSM/apparmor.html
- seccomp filter / userspace notification: https://docs.kernel.org/userspace-api/seccomp_filter.html
- Landlock: https://docs.kernel.org/userspace-api/landlock.html
- ptrace: https://man7.org/linux/man-pages/man2/ptrace.2.html
- fanotify: https://man7.org/linux/man-pages/man7/fanotify.7.html
- fanotify_init: https://man7.org/linux/man-pages/man2/fanotify_init.2.html
- Linux Audit rules: https://man7.org/linux/man-pages/man7/audit.rules.7.html
- auditctl: https://man7.org/linux/man-pages/man8/auditctl.8.html

## 15. Change control

This review is documentation/evidence only.

It does **not** authorize:

- runtime code changes;
- ptrace removal;
- BPF/LSM production integration;
- changes to Action permissions;
- a new daemon/service;
- schema migration;
- cross-backend baseline interchangeability;
- new performance/security claims.

Implementation authority belongs to the new M10 gate after preregistration and evidence.