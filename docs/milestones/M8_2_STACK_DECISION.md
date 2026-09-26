# M8.2 — eBPF Stack Decision

Date: 2026-09-26
Status: **CLOSED / ACCEPTED FOR EXPERIMENTAL M8.3 IMPLEMENTATION**
Tracking: #34

## Decision

Select **libbpf-rs / libbpf** as the implementation stack for the M8.3 experimental eBPF observation backend.

This decision does **not** replace native ptrace, does **not** authorize eBPF to produce evidence-equivalent PASS, and does **not** change the existing `cargo install execsurface --locked` or `AETHERXGLOBAL/execsurface@v0.1` user paths.

The selected eBPF backend must remain isolated/optional while M8.3–M8.6 establish semantic coverage, fail-closed loss handling, ptrace parity, performance, and kernel compatibility.

Aya remains a viable, preserved alternative. It is **not killed** by this decision.

## Why libbpf-rs was selected

Both Aya and libbpf-rs passed the same feasibility hard gates on Ubuntu 24.04:

- load / attach / detach;
- explicit unprivileged denial;
- process-exec metadata transport;
- parent → child lineage;
- producer-side event-loss accounting under deliberate ring-buffer pressure;
- metadata-only persisted event schema;
- privacy sentinels absent from persisted output;
- preservation of the existing ptrace product path.

The selection is therefore not based on a semantic failure by Aya.

libbpf-rs is selected because the combined evidence gives it the stronger integration path for ExecSurface:

1. **Current product MSRV is preserved.** The libbpf-rs probe was actually built with Rust 1.82. Aya's evaluated line required Rust 1.87 userspace plus a pinned nightly + `rust-src` + `bpf-linker` for the BPF object.
2. **Runtime dependency cost can be isolated.** The default libbpf-rs probe linked system `libelf`, `zlib`, and `zstd`, but the isolated vendored packaging experiment proved these dependencies can be removed from the final runtime binary.
3. **Vendored runtime proof succeeded without changing the existing product install path.** The final vendored binary linked only the ordinary loader, `libgcc_s`, and `libc` on the reference runner.
4. **Upstream Linux/libbpf alignment is useful for the next portability/parity gates.** This does not itself prove portability, but it provides the selected implementation path for M8.3–M8.6.
5. **The heavier native build chain can be quarantined.** Build complexity is accepted only in dedicated CI/release tooling or optional backend packaging; it must not become a requirement for the ptrace-only installation path.

## Clean feasibility evidence

Reference clean feasibility head:

`2109db1cc5ca16cb0648f7e3acae72efd5c8667f`

Workflow run:

- `M8 eBPF Feasibility` — run `36242212463` — **SUCCESS**
- normal repository `CI` on the same head — run `36242212478` — **SUCCESS**

The feasibility run closed all three jobs successfully:

- Host / BTF / privilege audit;
- Aya / build + lifecycle probe;
- libbpf-rs / build + lifecycle probe.

The reference host was Ubuntu 24.04.5, Linux `6.17.0-1022-azure`, x86_64, with readable kernel BTF and `kernel.unprivileged_bpf_disabled=2`.

## Common semantic feasibility contract proved by both candidates

The final probes intentionally use an Apache-compatible lineage strategy based on successful `clone`, `clone3`, `fork`, and `vfork` syscall exits instead of `task_struct` reads.

The persisted feasibility event is 16 bytes and contains only:

- event kind;
- process id;
- a metadata value whose meaning is event-kind-specific (for example child process id for lineage or returned file descriptor for the E9 file event);
- reserved field.

Both candidates demonstrated:

- exec observation;
- launched-tree lineage evidence;
- explicit permission denial without privilege escalation;
- a 4 KiB ring-buffer pressure experiment;
- a producer-side per-CPU drop counter that increased when the ring was deliberately not consumed;
- no persisted argv/environment sentinel content.

## Post-selection E9 hardening

After the stack decision, the feasibility harness was strengthened so E9 also requires one file-related metadata semantic from both candidates.

The chosen probe observes successful `openat` syscall exits and persists only process identity plus the returned file descriptor. It does not persist filename, file content, argv, environment, stdin, or user-memory content.

Corrected common-gate evidence:

- head `6e4ba2dc0fde94c77ca63abbdf7e4eac9a9162f1`;
- `M8 eBPF Feasibility` — run `36242866289` — **SUCCESS**;
- Host / BTF / privilege audit — **SUCCESS**;
- Aya / build + lifecycle probe — **SUCCESS**;
- libbpf-rs / build + lifecycle probe — **SUCCESS**.

The pre-correction libbpf userspace decoder failure is preserved as `M8_2_NEGATIVE_010_LIBBPF_FILE_EVENT_DECODER_GAP.md` rather than rewritten.

## libbpf-rs packaging evidence

Reference measured packaging head:

`e297e6713949d84887952b2fd3e4504423011e25`

Workflow:

- `M8 eBPF Packaging Probe` — run `36242514406` — **SUCCESS**
- normal repository `CI` — run `36242514405` — **SUCCESS**

The vendored packaging job proved:

- actual build toolchain: Rust `1.82.0`;
- resolver toolchain: Cargo `1.87.0` only to create a Rust-1.82-compatible lock;
- vendored libbpf/libelf/zlib build completed;
- build time for the measured release build: **54 seconds** on the reference hosted runner;
- binary size: **1,038,864 bytes**;
- binary SHA-256:
  `24b65d4df2c837d9772485338d1998d35102039698d1f501a84be820395554e8`;
- runtime `ldd` contained `libgcc_s` and `libc`, and did **not** contain `libelf.so`, `libz.so`, or `libzstd.so`;
- `M8_LIBBPF_VENDORED_RUNTIME_DEPENDENCY_PASS`;
- `M8_LIBBPF_VENDORED_SEMANTICS_PASS`;
- event transport: 176 events in the controlled run;
- lineage: at least one edge rooted at the launched PID;
- explicit pressure loss: 1366 dropped events;
- metadata-only privacy schema retained.

A later packaging regression gate retained the E9 file-metadata semantic:

- head `81b273dc667ffaae8b3b08ae9acd3ed4794fa61c`;
- `M8 eBPF Packaging Probe` — run `36242882478` — **SUCCESS**;
- vendored runtime packaging — **SUCCESS** including retained feasibility semantics.

## Preserved negative evidence

M8.2 intentionally retains failed approaches and harness discoveries:

1. unlocked libbpf dependency resolution;
2. transitive MSRV drift;
3. Aya BTF fork-signature/tooling path;
4. classic `sched_process_fork` hosted-runner policy rejection;
5. GPL-restricted kernel-struct read path;
6. Apache/task_struct boundary;
7. tracefs audit permission requirement;
8. vendored libbpf `autopoint` requirement;
9. vendored libbpf Rust 1.82 `rustfmt` requirement;
10. libbpf E9 file-event userspace decoder/schema drift.

These failures constrain M8.3. They are not rewritten as successes.

## Anti-drift review

### Scope

The selected path remains runtime execution-surface observation for baseline/candidate drift. It is not an EDR, antivirus, sandbox, threat-intelligence engine, or general kernel telemetry platform.

### Licensing

The experiment encountered GPL-restricted helper/kernel-struct access. The solution was to change the observation mechanism, **not** to change ExecSurface's Apache-2.0 license merely to satisfy the verifier.

### Installability

Native build dependencies required for vendored libbpf are accepted only for dedicated eBPF build/release infrastructure. They must not become prerequisites for normal ExecSurface installation.

### Evidence authority

M8.2 proves stack feasibility only. It does not prove full semantic parity with ptrace and does not authorize eBPF-derived PASS.

## M8.3 entry conditions

M8.3 may now implement the selected libbpf-rs backend, subject to all of the following:

- ptrace remains the default/reference backend;
- eBPF remains opt-in/experimental;
- backend identity and capabilities are explicit;
- unsupported event classes remain explicit;
- any loss, truncation, decode failure, attach failure, or observer uncertainty is fail-closed;
- current baseline/diff/policy/verdict/report semantics are not silently changed;
- normal ptrace installation remains unaffected;
- the implementation must later enter M8.5 cross-backend parity before any evidence-equivalence decision.

## Labels

- Aya feasibility: **PROVED on reference M8.2 environment**
- libbpf-rs feasibility: **PROVED on reference M8.2 environment**
- E9 file-related metadata semantic: **PROVED for both candidates on reference M8.2 environment**
- vendored libbpf runtime packaging: **PROVED on reference M8.2 environment**
- selected M8.3 stack: **libbpf-rs / libbpf — ACCEPTED**
- eBPF PASS authority: **NOT AUTHORIZED**
- ptrace correctness reference: **RETAINED**
