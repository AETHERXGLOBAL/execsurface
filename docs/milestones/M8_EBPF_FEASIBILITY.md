# M8.2 — eBPF Stack Feasibility & Selection Protocol

Date: 2026-09-26
Status: **CLOSED — SELECTION COMPLETE**
Tracking: #34
Decision record: `docs/milestones/M8_2_STACK_DECISION.md`

## Closure note

This file preserves the experiment protocol and its original hypotheses. M8.2 is now closed: the evidence-backed implementation choice for experimental M8.3 work is **libbpf-rs / libbpf**. Aya remains a viable preserved alternative and is not killed. eBPF-derived evidence is still **not authorized** to produce evidence-equivalent PASS; ptrace remains the correctness reference.

Post-selection E9 hardening also proved the required file-related metadata event on both candidate stacks on the reference environment. The corrected common hard-gate run is `36242866289` (**SUCCESS** for Host audit, Aya, and libbpf-rs). The failed pre-correction libbpf file-event decoder run remains preserved in `M8_2_NEGATIVE_010_LIBBPF_FILE_EVENT_DECODER_GAP.md`.

## Decision to make

Select the implementation stack for ExecSurface's experimental eBPF observation backend.

Candidates:

1. **Aya** — Rust-native userspace + Rust eBPF program stack.
2. **libbpf-rs / libbpf** — Rust userspace bindings around the upstream libbpf/CO-RE ecosystem with C eBPF programs/skeleton generation where appropriate.

No candidate is preferred by language affinity, popularity, or intuition. Selection requires reproducible evidence against the ExecSurface product contract.

## Governing constraints

M6.5 and M8.0 remain authoritative:

- ptrace remains the correctness reference;
- eBPF remains experimental and cannot produce evidence-equivalent PASS;
- event loss or observer uncertainty cannot silently become PASS;
- metadata-only privacy must be at least as strict as the ptrace reference;
- backend capability and limitations must be explicit;
- existing baseline/diff/policy/verdict/report semantics are not changed by this selection.

## Current ecosystem snapshot

### Aya

Observed current public release line at protocol creation:

- `aya` 0.14.0;
- `aya-ebpf` 0.2.1.

Relevant architectural properties documented by the project:

- Rust-native userspace API;
- Rust eBPF program support;
- BTF / CO-RE support;
- no runtime dependency on BCC or libbpf;
- eBPF build requires a specialized BPF target/toolchain path and currently uses nightly Rust plus `rust-src` and `bpf-linker` in the standard development flow.

### libbpf-rs / libbpf

Observed current public release line at protocol creation:

- `libbpf-rs` 0.26.2;
- release line depends on libbpf-sys / libbpf 1.7.x in the current series.

Relevant architectural properties:

- close alignment with the upstream Linux libbpf model;
- BTF / CO-RE support through libbpf;
- generated skeleton workflow through libbpf-cargo;
- native-system build dependencies and C/FFI surface must be evaluated for distribution and reproducibility.

Versions above are evidence inputs, not permanent pins. Exact versions used in experiments must be recorded in the resulting evidence.

## Team for M8.2

### Fixed

- Innovation Scientist / Architect
- Deviation Prevention / Scientific Integrity

### Dynamic specialists

- Linux kernel / eBPF engineer
- Rust systems engineer
- BTF / CO-RE compatibility engineer
- release and supply-chain engineer
- GitHub Actions / CI portability engineer
- privacy/security red team
- independent reproducibility reviewer

## Historical hypotheses at protocol creation

### H-A — Aya operational advantage

Aya may reduce distribution complexity for a Rust-native product by avoiding a runtime libbpf dependency and by enabling Rust code sharing between userspace and eBPF components.

This hypothesis was evaluated during M8.2. Aya feasibility was proved on the reference environment, but the final stack decision selected libbpf-rs / libbpf for M8.3 for the reasons recorded in `M8_2_STACK_DECISION.md`.

### H-L — libbpf ecosystem advantage

libbpf-rs may reduce kernel-compatibility and tooling risk through closer alignment with the upstream Linux BPF/CO-RE ecosystem.

This hypothesis was evaluated during M8.2 and contributed to the accepted libbpf-rs / libbpf implementation decision together with the MSRV and packaging evidence recorded in the decision document.

### H-N — neither stack is automatically acceptable

If neither candidate can meet ExecSurface's loss-accounting, fail-closed, privacy, packaging and reproducibility requirements without disproportionate complexity, M8.2 may return **KILLED / DEFERRED** for eBPF rather than forcing a selection.

This contingency was not triggered: both candidates passed the common feasibility hard gates on the reference environment.

## Required experiments

All experiments must run from a clean Ubuntu 24.04 GitHub-hosted runner and record toolchain versions.

### E1 — Reproducible clean build

For each candidate:

- start from a clean runner;
- install only explicitly declared dependencies;
- build a minimal tracepoint-style eBPF program and userspace loader;
- record wall-clock setup/build time;
- record all packages/toolchains introduced;
- rerun to prove determinism of the declared build path where practical;
- retain build logs for failures.

Gate question:

> Can a contributor and release workflow reproduce the eBPF artifact from declared inputs without hidden machine state?

### E2 — Load / attach / detach lifecycle

For each candidate:

- detect host BTF availability;
- load a minimal BPF program;
- attach to a documented tracing hook;
- observe a controlled event;
- detach cleanly;
- verify no lingering pinned maps/programs/links remain unless intentionally configured.

Gate question:

> Is lifecycle management explicit, diagnosable, and safe enough for an optional developer/CI backend?

### E3 — Privilege and host-policy audit

Record:

- whether root is required;
- Linux capabilities required;
- effect of unprivileged-BPF policy;
- relevant sysctls/kernel configuration;
- behavior when permission is insufficient.

Required behavior:

- no automatic privilege escalation;
- no automatic weakening of sysctls/host security;
- a denied environment must produce an explicit unsupported/permission state, not misleading evidence.

### E4 — BTF / CO-RE portability

Test the chosen minimal program against at least the available GitHub-hosted Ubuntu kernel and one additional declared Linux kernel environment when the test infrastructure is available.

Record:

- BTF discovery;
- CO-RE relocation behavior;
- attachment mechanism availability;
- verifier/load diagnostics;
- kernel-version assumptions.

### E5 — Event transport and loss visibility

Evaluate BPF ring buffer as the primary candidate transport.

The Linux kernel ring-buffer design is attractive because it provides a shared MPSC stream and ordering properties across CPUs, useful for causal process lifecycle evidence. The experiment must still prove operational loss visibility rather than assume it.

For each candidate implementation, force buffer pressure and determine whether ExecSurface can observe:

- reservation/output failure;
- producer-side drop counters or equivalent explicit accounting;
- userspace consumer lag;
- teardown before drain completion;
- any decode or sequence gap.

A design that can lose selected events without detectable incompleteness is **KILLED for PASS-capable operation**.

### E6 — Privacy audit

Use sentinel values for:

- environment secrets;
- argv secrets;
- file contents;
- stdin;
- network payload content.

The experiment passes only if persisted evidence remains metadata-only according to the M8 privacy contract.

Temporary kernel-side reads, if needed for metadata reconstruction, must be narrowly scoped and must not appear in persisted evidence.

### E7 — Packaging / release impact

For each candidate record:

- additional workspace crates;
- build-time native dependencies;
- runtime shared-library dependencies;
- static/musl feasibility where relevant;
- effect on current GitHub Release artifact generation;
- effect on crates.io packaging;
- effect on `cargo install execsurface --locked`;
- licensing implications of eBPF-side code and linked components;
- supply-chain inputs that must be pinned or attested.

Critical invariant:

> The existing ptrace-only install path must not become harder merely because the optional eBPF backend exists.

Feature-gating or separate optional artifacts are preferred if required to preserve that invariant.

### E8 — Diagnostics and verifier operability

Deliberately introduce one invalid eBPF program or load condition and evaluate:

- verifier log accessibility;
- error fidelity;
- source-location usefulness;
- failure classification;
- ability to surface a concise user-facing diagnostic without hiding the original evidence.

### E9 — Minimum ExecSurface semantic probe

Each candidate must demonstrate at least:

- process exec observation;
- process lineage context sufficient to scope the launched command tree;
- one network connect destination or one file-related metadata event;
- explicit backend identity;
- explicit completeness/loss state.

This is a feasibility probe, not parity authority.

Post-selection hardening result: both Aya and libbpf-rs demonstrated the file-related metadata event on the reference environment; run `36242866289` is the corrected common-gate evidence.

## Scoring model

No single aggregate score can override a hard gate.

### Hard gates

A candidate is ineligible if it cannot demonstrate:

- detectable event loss/incompleteness;
- metadata-only persisted evidence;
- explicit privilege failure;
- reproducible declared build inputs;
- clean attach/detach lifecycle;
- a path that preserves the existing ptrace install/runtime behavior.

### Comparative dimensions

For candidates that pass hard gates, compare:

1. kernel compatibility / CO-RE confidence;
2. build reproducibility;
3. CI operability;
4. release/packaging complexity;
5. diagnostics/verifier ergonomics;
6. Rust integration complexity;
7. event transport control;
8. supply-chain surface;
9. expected maintenance burden;
10. ability to support the M8.3–M8.6 evidence plan.

No marketing-style numeric total is authoritative. The decision document must explain trade-offs and preserve losing-candidate evidence.

## Attachment preference

The spike should prefer documented stable tracepoints or BTF-aware attachment mechanisms when they establish the required semantic.

`fentry/fexit` may offer lower overhead and BTF-aware argument access on supporting kernels, but they require compatible kernel/BTF support and therefore cannot be assumed universally available.

Kprobes may be used as a controlled fallback experiment but are not automatically a stable product contract because kernel-internal function names/signatures can vary.

## Initial implementation direction — historical ordering, not the final decision

The first spike began with **Aya** because:

- ExecSurface is already Rust-native;
- Aya provides a direct Rust userspace + Rust eBPF path;
- it tested the strongest hypothesis for preserving a self-contained Rust distribution model.

That ordering was experimental only. It was not the final selection. libbpf-rs subsequently received equivalent hard-gate evaluation and was selected for experimental M8.3 implementation.

## Required M8.2 output

Before closing M8.2, commit:

1. exact experiment source/configuration;
2. CI workflow(s) used;
3. versions and environment evidence;
4. raw success/failure logs or durable workflow references;
5. privacy and loss tests;
6. packaging impact analysis;
7. candidate decision or explicit defer/kill result;
8. independent anti-drift review.

These outputs are now satisfied by the committed experiment sources/workflows, preserved negative evidence, packaging evidence, and `M8_2_STACK_DECISION.md`.

## Current labels

- Aya feasibility: **PROVED on reference M8.2 environment**
- libbpf-rs/libbpf feasibility: **PROVED on reference M8.2 environment**
- E9 file-related metadata semantic: **PROVED for both candidates on reference M8.2 environment**
- eBPF stack selected: **libbpf-rs / libbpf — ACCEPTED FOR EXPERIMENTAL M8.3**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace reference backend: **PROVED / RETAINED under M6.5**
