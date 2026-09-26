# M8.2 — eBPF Stack Feasibility & Selection Protocol

Date: 2026-09-26
Status: **OPEN — EVIDENCE COLLECTION REQUIRED**
Tracking: #34

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

## Hypotheses

### H-A — Aya operational advantage

Aya may reduce distribution complexity for a Rust-native product by avoiding a runtime libbpf dependency and by enabling Rust code sharing between userspace and eBPF components.

This hypothesis is **OPEN** until build, packaging, verifier diagnostics, release and host-compatibility evidence is collected.

### H-L — libbpf ecosystem advantage

libbpf-rs may reduce kernel-compatibility and tooling risk through closer alignment with the upstream Linux BPF/CO-RE ecosystem.

This hypothesis is **OPEN** until equivalent build, packaging, diagnostics and portability evidence is collected.

### H-N — neither stack is automatically acceptable

If neither candidate can meet ExecSurface's loss-accounting, fail-closed, privacy, packaging and reproducibility requirements without disproportionate complexity, M8.2 may return **KILLED / DEFERRED** for eBPF rather than forcing a selection.

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

## Initial implementation direction — not a decision

The first spike should begin with **Aya** because:

- ExecSurface is already Rust-native;
- Aya provides a direct Rust userspace + Rust eBPF path;
- it can test the strongest hypothesis for preserving a self-contained Rust distribution model.

This ordering is experimental only. It is **not** an Aya selection. libbpf-rs must receive an equivalent hard-gate evaluation before M8.2 closes unless Aya is killed by a hard gate first and the evidence justifies changing the experiment sequence.

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

## Current labels

- Aya candidate: **OPEN**
- libbpf-rs/libbpf candidate: **OPEN**
- eBPF stack selected: **OPEN**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace reference backend: **PROVED / RETAINED under M6.5**
