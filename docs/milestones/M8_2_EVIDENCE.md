# M8.2 — eBPF Feasibility Evidence Closure

Date: 2026-09-26
Status: **CLOSED / ACCEPTED**
Tracking: #34
Decision: `docs/milestones/M8_2_STACK_DECISION.md`
Protocol: `docs/milestones/M8_EBPF_FEASIBILITY.md`

## Scope

M8.2 evaluated Aya and libbpf-rs/libbpf as implementation stacks for an **experimental** eBPF observation backend.

This closure proves feasibility on the stated reference environment. It does **not** prove ptrace parity, production readiness, universal kernel portability, lower overhead, or authority for eBPF-derived PASS.

Native ptrace remains the default and correctness reference.

## Reference environment

The common reference runner was GitHub-hosted Ubuntu 24.04.5, Linux `6.17.0-1022-azure`, x86_64.

The host audit established:

- kernel BTF was available;
- tracefs/debugfs were mounted;
- unprivileged BPF was disabled by host policy;
- the relevant syscall-exit tracepoint record layouts were inspected directly;
- `ret` was verified at offset 16 with size 8 for `clone`, `clone3`, `fork`, `vfork`, and `openat` before those direct-context reads counted as evidence.

## Common hard-gate evidence

Corrected common E9 run:

- workflow: `M8 eBPF Feasibility`
- run: `36242866289`
- source head: `6e4ba2dc0fde94c77ca63abbdf7e4eac9a9162f1`
- result: **SUCCESS**

All three jobs succeeded:

1. Host / BTF / privilege audit;
2. Aya / build + lifecycle probe;
3. libbpf-rs / build + lifecycle probe.

Normal repository CI on the same source head:

- run: `36242866298`
- result: **SUCCESS**

Both candidate stacks proved the same minimum feasibility contract:

- clean declared build path;
- load / attach / detach lifecycle;
- explicit unprivileged denial rather than privilege escalation;
- explicit invalid-attach diagnostic;
- process-exec metadata transport;
- launched-tree parent -> child lineage using Apache-compatible syscall-exit semantics;
- file-related metadata using successful `openat` exit metadata;
- producer-side loss accounting under deliberate ring-buffer pressure;
- metadata-only 16-byte persisted records;
- argv/environment privacy sentinels absent from persisted proof output.

## Aya E9 evidence

The corrected common run emitted:

- `M8_AYA_ATTACH_PASS`;
- `M8_AYA_EVENT_TRANSPORT_PASS events=50`;
- `M8_AYA_LINEAGE_PASS ... rooted_edges=1 ...`;
- `M8_AYA_FILE_METADATA_PASS ... successful_opens=19`;
- `M8_AYA_LOSS_ACCOUNTING_PASS dropped=2972 ...`;
- `M8_AYA_PRIVACY_SCHEMA_PASS event_bytes=16 fields=kind,pid,value,reserved`;
- persisted-output privacy PASS;
- explicit permission-denial and invalid-attach diagnostic PASS.

The observed event/drop counts are workload evidence only. They are **not** a performance comparison against libbpf-rs.

## libbpf-rs E9 evidence

The corrected common run emitted:

- `M8_LIBBPF_ATTACH_PASS`;
- `M8_LIBBPF_EVENT_TRANSPORT_PASS events=220`;
- `M8_LIBBPF_LINEAGE_PASS ... rooted_edges=1 ...`;
- `M8_LIBBPF_FILE_METADATA_PASS ... successful_opens=19`;
- `M8_LIBBPF_LOSS_ACCOUNTING_PASS dropped=2905`;
- `M8_LIBBPF_PRIVACY_SCHEMA_PASS event_bytes=16 fields=kind,pid,value,reserved`;
- persisted-output privacy PASS;
- explicit permission-denial and invalid-attach diagnostic PASS.

The observed event/drop counts are not used as a relative performance claim.

## Vendored libbpf packaging closure

Final post-E9 packaging run:

- workflow: `M8 eBPF Packaging Probe`
- run: `36242882478`
- source head: `81b273dc667ffaae8b3b08ae9acd3ed4794fa61c`
- result: **SUCCESS**

Normal repository CI on that source head:

- run: `36242882475`
- result: **SUCCESS**

Measured vendored release evidence:

- product build compiler: Rust `1.82.0`;
- lock resolver: Cargo `1.87.0` with incompatible-Rust fallback to generate a Rust-1.82-compatible lock;
- libbpf-rs/libbpf-cargo: `0.27.0` in the experiment;
- measured release build: **56 seconds** on the reference hosted runner;
- binary size: **1,043,120 bytes**;
- SHA-256: `aa10b60be7fbef7dccc88e60ff823adcb92626dc1f2be56f005ca450905adaa7`;
- runtime linked libraries included `libgcc_s` and `libc` but not `libelf.so`, `libz.so`, or `libzstd.so`;
- `M8_LIBBPF_VENDORED_RUNTIME_DEPENDENCY_PASS`;
- `M8_LIBBPF_VENDORED_SEMANTICS_PASS`;
- file metadata: 19 successful opens rooted at the controlled child;
- explicit pressure loss: 2907 dropped events in the measured run;
- metadata-only privacy schema retained.

The broader vendored build surface is also evidence: clang, native build tooling, autoconf/automake/libtool, gettext/autopoint, flex/bison, and related build inputs must remain isolated from the normal ptrace-only installation path.

## Stack decision

Both stacks passed the common semantic hard gates. Aya was not rejected for semantic failure.

The accepted M8.3 implementation stack is **libbpf-rs / libbpf** because the evidence-backed integration path:

- preserves the product Rust 1.82 build target for the experimental userspace path;
- can remove libelf/zlib/zstd from the distributed runtime binary through isolated vendoring;
- aligns the next portability/parity work with the upstream Linux libbpf ecosystem;
- permits the heavier native build chain to be quarantined in eBPF-specific build/release infrastructure.

Aya remains a validated alternative and is not killed.

## Preserved negative evidence

The following failures remain part of M8.2 evidence:

1. `M8_2_NEGATIVE_001_LIBBPF_UNLOCKED_RESOLUTION.md`
2. `M8_2_NEGATIVE_002_LIBBPF_TRANSITIVE_MSRV_DRIFT.md`
3. `M8_2_NEGATIVE_003_AYA_BTF_FORK_SIGNATURE.md`
4. `M8_2_NEGATIVE_004_LIBBPF_CLASSIC_FORK_TRACEPOINT_POLICY.md`
5. `M8_2_NEGATIVE_005_LIBBPF_GPL_RESTRICTED_CORE_READ.md`
6. `M8_2_NEGATIVE_006_BTF_TASK_STRUCT_APACHE_BOUNDARY.md`
7. `M8_2_NEGATIVE_007_TRACEFS_AUDIT_PERMISSION.md`
8. `M8_2_NEGATIVE_008_LIBBPF_VENDORED_AUTOPOINT.md`
9. `M8_2_NEGATIVE_009_LIBBPF_VENDORED_RUSTFMT.md`
10. `M8_2_NEGATIVE_010_LIBBPF_FILE_EVENT_DECODER_GAP.md`

Each remains evidence about dependency resolution, host policy, licensing boundaries, build prerequisites, or harness defects. None is rewritten as if it never occurred.

## Closure labels

- M8.2 feasibility: **PROVED on reference environment**
- Aya feasibility: **PROVED on reference environment**
- libbpf-rs feasibility: **PROVED on reference environment**
- E9 file-related metadata: **PROVED for both candidates on reference environment**
- vendored libbpf packaging: **PROVED on reference environment**
- M8.3 selected stack: **libbpf-rs / libbpf — ACCEPTED**
- cross-backend semantic parity: **OPEN**
- multi-kernel portability: **OPEN**
- performance superiority: **OPEN / NO CLAIM**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace correctness reference: **RETAINED**
