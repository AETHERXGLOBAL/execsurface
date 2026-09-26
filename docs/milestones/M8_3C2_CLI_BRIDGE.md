# M8.3c2 — Explicit Experimental libbpf CLI Bridge

Date: 2026-09-26
Status: **CLOSED / PROVED ON REFERENCE CI**
Tracking: #37
Parent evidence: `docs/milestones/M8_3C_COLLECTOR.md`

## Objective

Expose the proved M8.3 real libbpf observer through an explicit observation-only CLI route without changing the default ptrace path, without adding libbpf to the normal workspace dependency graph, without automatic privilege elevation, and without allowing experimental eBPF evidence to become PASS-eligible.

Accepted interface:

```bash
execsurface observe --backend experimental-libbpf --collector PATH -- COMMAND [ARGS...]
```

The existing interface remains unchanged:

```bash
execsurface observe -- COMMAND [ARGS...]
```

and still selects the native ptrace reference backend.

`learn` and `check` remain ptrace-only.

## Implemented boundaries

The CLI bridge:

- requires `--backend experimental-libbpf` explicitly;
- requires an explicit `--collector PATH`;
- performs no collector discovery;
- never invokes `sudo` or changes host security policy;
- never silently falls back to ptrace;
- invokes the companion observer through its versioned JSON report protocol;
- validates `protocol_version == 1`;
- validates backend id `linux-libbpf-metadata-experimental-v1`;
- requires explicit supported and unsupported capability arrays;
- rejects `observation_complete=true`;
- rejects `completeness=complete` or any state outside the allowed incomplete states;
- removes its temporary report directory after the observation attempt;
- does not expose the experimental backend through `learn` or `check`.

The accepted incomplete states at this gate are:

- `incomplete_capability`;
- `incomplete_loss`;
- `incomplete_limit`.

No M8.3c2 path can authorize evidence-equivalent PASS.

## Accepted evidence

Final source head:

`54c0ba2725dc12f05fbf36bcfe2a809f52f234fa`

### Normal repository CI

- workflow: `CI`
- run: `36246643348`
- result: **SUCCESS**

Verified:

- format — PASS;
- clippy — PASS;
- tests — PASS;
- lockfile integrity — PASS.

### Real libbpf + CLI integration gate

- workflow: `M8.3 Real libbpf Observer`
- run: `36246643356`
- job: `isolated real libbpf observer`
- result: **SUCCESS**

The run proved, in sequence:

1. reference-host tracepoint/BTF contract — PASS;
2. declared native dependency setup — PASS;
3. Rust 1.82-compatible isolated dependency resolution — PASS;
4. observer formatting — PASS;
5. real observer + contract checker + deterministic fixture + ExecSurface CLI build — PASS;
6. unprivileged observer denial occurs before target launch — PASS;
7. real exec and successful-open evidence — PASS;
8. serialized observer report matches the central typed contract — PASS;
9. launched-tree lineage evidence — PASS;
10. event-budget overflow remains fail-closed — PASS;
11. default `execsurface observe -- ...` remains ptrace — PASS;
12. experimental CLI requires an explicit companion and performs no ptrace fallback — PASS;
13. privileged execution selected externally by the CI caller surfaces only incomplete experimental evidence — PASS;
14. a forged companion report claiming complete/PASS-eligible evidence is rejected — PASS;
15. `learn` and `check` reject the experimental backend option before the target can run — PASS.

Proof markers include:

- `M8_3C_UNPRIVILEGED_PRELAUNCH_DENIAL_PASS`;
- `M8_3C_REAL_EXEC_OPEN_REPORT_PASS`;
- `M8_3C_REPORT_PRIVACY_PASS`;
- `M8_3C_CENTRAL_CONTRACT_PASS`;
- `M8_3C_LINEAGE_REPORT_PASS`;
- `M8_3C_EVENT_LIMIT_FAIL_CLOSED_PASS`;
- `M8_3C2_DEFAULT_PTRACE_UNCHANGED_PASS`;
- `M8_3C2_EXPLICIT_COMPANION_NO_FALLBACK_PASS`;
- `M8_3C2_REAL_CLI_BRIDGE_PASS`;
- `M8_3C2_FORGED_COMPLETE_REJECTED_PASS`;
- `M8_3C2_LEARN_CHECK_PTRACE_ONLY_PASS`.

## Preserved negative evidence

Two pre-closure failures remain part of the record:

1. source `aa4de95d0293f92cc9a2aad3b0f3a2cc3f4d2f8f` — normal CI stopped at `cargo fmt --check`; the exact rustfmt diff was applied without semantic changes in `f311a978d14cfc5bf6622b9fbae0018cac706afe`.
2. workflow run `36246426763` at source `17cbba59b63e02e03c01e4bf3c83aae25533217b` — the dedicated gate failed during setup because it tried to build a nonexistent package id `execsurface-cli`. The actual package is `execsurface`. The gate was corrected to use the explicit manifest path `crates/execsurface-cli/Cargo.toml`, avoiding package-name inference. This was a workflow setup defect, not an observer, CLI, or eBPF semantic failure.

These failures are retained rather than rewritten as if they never occurred.

## M8.3 closure decision

With M8.3a, M8.3b, M8.3c1, and M8.3c2 accepted, **M8.3 is CLOSED / ACCEPTED for experimental observation integration**.

This means ExecSurface now has a real opt-in libbpf observation path reachable through an explicit CLI route while the native ptrace path remains the default correctness reference.

It does **not** authorize:

- ptrace/eBPF semantic parity;
- cross-backend baseline comparability;
- eBPF-derived PASS;
- automatic backend selection or fallback;
- performance-superiority claims;
- universal kernel/platform support;
- stable/public eBPF packaging.

## Next gate

**M8.4 — fail-closed loss/truncation gate.**

M8.4 must deepen controlled proofs for producer loss, userspace lag, teardown races, attachment/lifecycle failure and other event-loss channels before any later parity or PASS-authority discussion.

## Labels

- M8.3c2 CLI bridge: **PROVED ON REFERENCE CI**
- M8.3 overall implementation gate: **CLOSED / ACCEPTED**
- ptrace default/reference: **RETAINED**
- ordinary install dependency graph: **UNCHANGED**
- automatic backend selection: **NOT AUTHORIZED**
- eBPF PASS authority: **NOT AUTHORIZED**
- cross-backend parity: **OPEN / M8.5**
- next gate: **M8.4**
