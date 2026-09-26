# M8.2 — Independent Anti-Drift Review

Date: 2026-09-26
Status: **ACCEPTED WITH EXPLICIT OPEN BOUNDARIES**
Tracking: #34

## Review role

This review applies the fixed Deviation Prevention / Scientific Integrity role independently of the stack-selection preference.

The review asks whether M8.2 remained aligned with the ExecSurface objective, preserved evidence failures, protected licensing/privacy/installability boundaries, and avoided promoting feasibility evidence into stronger claims.

## Findings

### 1. Objective alignment — PASS

M8.2 stayed scoped to a pluggable runtime observation backend for execution-surface drift.

It did not expand ExecSurface into EDR, antivirus, sandboxing, threat intelligence, general telemetry, or an agent platform.

### 2. Existing product path preservation — PASS

The feasibility implementation remained isolated under `experiments/m8-ebpf/**` plus M8-specific CI/docs.

The normal product workspace, default ptrace runtime, public installation path, baseline/diff/policy/verdict/report semantics, and stable Action contract were not silently migrated to eBPF.

### 3. Failure preservation — PASS

Ten negative-evidence records are retained. Failures in dependency resolution, hosted-runner attachment policy, GPL-restricted kernel access, tracefs permissions, native vendoring prerequisites, rustfmt/toolchain assumptions, and the file-event decoder were not erased after fixes.

### 4. Licensing boundary — PASS

When the kernel rejected `task_struct` access and a helper path under the Apache-only eBPF program contract, the experiment changed the observation semantic to an Apache-compatible syscall-exit path.

It did **not** relabel the program GPL merely to pass the verifier while leaving the repository licensing model unchanged.

### 5. E9 semantic-completeness check — PASS AFTER CORRECTION

The initial process-only feasibility evidence did not satisfy the protocol's E9 requirement for a network destination or file-related metadata event.

The review blocked closure until both candidates implemented and proved file-related metadata. The corrected common run `36242866289` succeeded for Host audit, Aya, and libbpf-rs.

This correction is material evidence that the anti-drift gate changed the work rather than rubber-stamping it.

### 6. Privacy boundary — PASS FOR THE M8.2 FEASIBILITY SCHEMA

The final controlled record remains metadata-only and 16 bytes. The file experiment persists process identity plus successful returned fd; it does not persist filename, file contents, argv sentinel, environment sentinel, stdin, or network payload.

This does not imply future richer event classes are automatically privacy-equivalent. Each new class requires its own audit.

### 7. Loss handling — PARTIAL / SUFFICIENT FOR FEASIBILITY ONLY

Both stacks demonstrated producer-side drop accounting under forced ring-buffer pressure.

This proves loss visibility is technically feasible. It does **not** yet prove the complete M8.4 fail-closed contract for all teardown, decode, consumer-lag, attachment, truncation, and observer-failure cases.

Therefore eBPF-derived PASS remains unauthorized.

### 8. Stack selection integrity — PASS

libbpf-rs was not selected because Aya failed: both candidates passed the common hard gates.

The selection is supported by integration evidence: product-MSRV-compatible userspace build, successful isolated vendored runtime packaging, and a path aligned with the Linux/libbpf ecosystem for later compatibility/parity work.

The review explicitly rejects the stronger claim that libbpf-rs has already proved better performance, better semantic fidelity, or universal portability.

Aya remains a validated alternative.

### 9. Packaging isolation — PASS WITH COST RECORDED

Vendored libbpf packaging removed `libelf`, `libz`, and `libzstd` from runtime linkage on the reference runner, but required a materially larger build-tool surface including native/autotools/gettext/autopoint/flex/bison inputs.

That cost is acceptable only while quarantined in eBPF-specific build/release infrastructure. It must not become a prerequisite for the normal ptrace-only install.

### 10. Portability — OPEN

M8.2 proves the reference GitHub-hosted Ubuntu/kernel environment only.

No universal Linux-kernel compatibility claim is authorized. Multi-kernel/platform evidence belongs to later M8 gates.

### 11. Performance — OPEN / NO CLAIM

M8.2 did not establish an authoritative ptrace-vs-eBPF performance result.

Event counts, drop counts, individual build times, and probe timing from different controlled runs must not be presented as runtime superiority evidence.

## Required M8.3 constraints

M8.3 may proceed only under these constraints:

- ptrace remains default and correctness reference;
- libbpf-rs eBPF backend is explicit opt-in / experimental;
- backend identity, capabilities, unsupported semantics, and limitations are machine-visible;
- incomplete or lost evidence cannot yield PASS;
- no silent baseline compatibility between ptrace and eBPF;
- no product-wide Rust MSRV increase caused by the optional backend;
- no native eBPF build prerequisites added to normal ptrace installation;
- privacy review is repeated for every added event class;
- later cross-backend parity evidence is mandatory before any evidence-equivalence decision.

## Review verdict

**M8.2 ACCEPTED for entry into M8.3.**

This verdict authorizes implementation work on the selected experimental libbpf-rs backend only. It does not authorize eBPF as default, evidence-equivalent, production-ready, or faster than ptrace.
