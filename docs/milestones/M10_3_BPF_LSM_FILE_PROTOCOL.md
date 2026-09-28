# M10.3 — BPF-LSM-FILE-001 Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer`
Status: **PREREGISTERED — NO RESULT YET**

## Objective

Test whether an **audit-only, research-only BPF LSM observer** can provide stronger kernel-object-grounded file evidence for selected propositions exposed as weak by M10.1 and M10.2, without changing current public runtime semantics.

This gate does **not** authorize product integration, enforcement, PASS authority, `learn/check`, backend auto-selection, baseline interchangeability, or any change to `main`/Marketplace behavior.

## Fixed team roles

- **Innovative Systems Architect** — seek a stronger evidence anchor than current userspace ptrace snapshots while preserving the ExecSurface evidence model.
- **Anti-Deviation / Skeptical Reviewer** — reject BPF/LSM promotion based on novelty, external criticism, or implementation enthusiasm; require executable evidence and preserve negative results.

Dynamic specialists for M10.3:
- Linux LSM / BPF LSM
- libbpf / verifier / BTF
- VFS and file-object semantics
- namespaces / credentials / containers
- event-loss and session-membership semantics
- Rust/C systems integration
- reproducibility / CI evidence

## Hypothesis under test

`H_FILE_AUTHORITY`:

> For a bounded file/open proposition, an audit-only BPF LSM hook can emit evidence tied to the kernel object participating in security mediation, and can be correlated to a declared ExecSurface research session with explicit health/loss state, providing stronger object authority than current ptrace entry-time pathname snapshots.

The hypothesis is **not** that BPF LSM replaces ptrace generally.

## Frozen implementation boundary

The prototype must:

1. live only under `experiments/m10/bpf-lsm-file/` and M10-specific workflow/docs;
2. return allow from every LSM program — **no enforcement**;
3. collect no file contents, argv, envp, stdin, network payloads, or arbitrary process memory;
4. emit only bounded metadata needed by this experiment;
5. keep an explicit session identifier / target membership model;
6. expose setup/attach/transport/parse failures as non-successful evidence states;
7. record host capability facts, including whether BPF LSM is enabled in the active LSM list;
8. never fall back silently to ptrace or a non-LSM hook and call that an LSM success.

## First proposition

The first bounded proposition is file-object identity at an LSM mediation point.

Candidate hook priority:

1. `file_open` if attachable and semantically suitable on the runner;
2. otherwise another documented file-related BPF LSM hook only if its changed proposition is recorded before interpreting results.

Minimum emitted object metadata:
- TID/PID identity needed for session correlation;
- file inode number;
- device identity sufficient to distinguish controlled A/B files;
- access/open metadata available at the selected hook without reading payload/content;
- monotonic event sequence or timestamp adequate for bounded correlation.

Path strings are **not required** for the authority proposition. Kernel object identity is the target of this gate.

## Controlled fixture

Use two distinct regular files A and B with:
- distinct inode identities;
- known one-byte contents only as independent userspace truth when needed;
- repeated deterministic opens/reads from one declared target session.

Where feasible, include the M10.1 pathname-buffer mutation fixture as an adversarial subcase so that:
- ptrace entry pathname intent may diverge;
- BPF LSM object identity is compared against independent kernel-opened truth.

If the runner cannot attach BPF LSM, the gate must classify capability failure rather than fabricate parity.

## Mandatory health facts

Every interpreted run must preserve:
- attach success/failure;
- target/session registration success;
- emitted event count;
- userspace decode errors;
- ring-buffer/perf-buffer loss signal where available;
- unmatched target events;
- teardown status;
- explicit host LSM list / BPF-LSM availability.

Any unknown loss state is insufficient for future PASS authority.

## Frozen classifications

Exactly one top-level classification must be used:

- `BPF_LSM_FILE_AUTHORITY_PASS` — selected bounded proposition is object-grounded and matches independent truth with explicit healthy transport/session state.
- `BPF_LSM_FILE_AUTHORITY_PARTIAL` — useful kernel-object evidence exists, but one or more authority/health/session properties remain unproved.
- `BPF_LSM_FILE_HOST_UNAVAILABLE` — tested host cannot load/attach required BPF LSM program under declared environment.
- `BPF_LSM_FILE_COUNTEREXAMPLE` — attached prototype emits wrong/ambiguous object evidence under a controlled accepted run.
- `BPF_LSM_FILE_INFRA_FAILURE` — harness/workflow failure prevents scientific interpretation.

## Success criteria

`BPF_LSM_FILE_AUTHORITY_PASS` requires all of:

1. BPF LSM attachment is real and verified — no substitute hook;
2. all selected target file events are correlated to the declared session;
3. kernel object identity distinguishes controlled A/B truth correctly for all accepted samples;
4. no accepted sample has unknown transport loss or decode failure;
5. the program remains audit-only and returns allow;
6. metadata-only privacy boundary is preserved;
7. result does not depend on modifying the public ptrace observer.

## Kill criteria

The candidate is killed for this proposition if any accepted healthy run demonstrates kernel object identity inconsistent with independent controlled truth, or if loss/session ambiguity can occur while the harness still labels evidence authoritative.

Host unavailability is **not** semantic falsification; it is a product/deployment constraint and must be recorded separately.

## Product freeze

Until M10 closeout:
- `main` unchanged by research implementation;
- `v0.1.0-alpha.3` unchanged;
- Marketplace Action unchanged;
- public ptrace authority unchanged;
- no BPF-LSM PASS authority;
- no baseline migration/interchangeability;
- no privileged helper/daemon productization.
