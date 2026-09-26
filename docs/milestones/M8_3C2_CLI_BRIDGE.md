# M8.3c2 — Explicit Experimental CLI Bridge

Date: 2026-09-26
Status: **OPEN — CI / INTEGRATION EVIDENCE REQUIRED**
Tracking: #37
Parent: `docs/milestones/M8_3C_COLLECTOR.md`

## Objective

Expose the already-proved isolated M8.3c1 libbpf companion collector through an explicit observation-only CLI route without linking libbpf into the ordinary ExecSurface binary and without changing `learn` or `check` semantics.

Target route:

```text
execsurface observe --backend experimental-libbpf --collector PATH -- COMMAND [ARGS...]
```

The existing route remains authoritative and unchanged:

```text
execsurface observe -- COMMAND [ARGS...]
```

## Architectural shape

The normal `execsurface` package keeps the existing ptrace CLI implementation intact. A small entry wrapper intercepts only an explicit `observe --backend ...` form. Every invocation outside that form is delegated to the existing CLI implementation.

The experimental libbpf observer remains a separate companion binary. The main ExecSurface package does not add libbpf, libelf, zlib, clang, BPF skeleton generation, or other native eBPF build dependencies.

## Non-negotiable invariants

1. Default `observe`, `learn`, and `check` continue to use ptrace under their existing semantics.
2. No automatic eBPF discovery or selection.
3. No silent eBPF-to-ptrace fallback.
4. ExecSurface never invokes `sudo`, changes sysctls, or weakens host security policy.
5. The user supplies the companion collector path explicitly.
6. The companion writes into a per-invocation private temporary directory created atomically by ExecSurface.
7. The CLI validates protocol version, backend identity, platform, architecture, privacy profile, and the exact supported/unsupported capability partition before displaying the report.
8. Duplicate or drifted capability declarations are rejected.
9. `observation_complete=true`, `completeness=complete`, and unknown completeness states are rejected during M8.3c2.
10. Reported producer drops cannot coexist with a non-loss completeness state.
11. Collector launch/load failure is an explicit ERROR and must state that no ptrace fallback was attempted.
12. eBPF-derived PASS remains NOT AUTHORIZED.

## Temporary evidence handling

The wrapper creates a unique directory below the platform temporary root using an atomic `create_dir` operation and, on Unix, restricts it to mode `0700` before passing the report pathname to the companion. The report file and directory are removed on wrapper teardown.

This is an evidence-transfer mechanism, not a new persistent evidence format.

## Central contract binding

The CLI imports the central experimental `BackendDescriptor` from `execsurface-observe` and maps every typed `ObservationCapability` exhaustively to the M8 experimental wire name.

This has an intentional anti-drift property: adding a new capability enum variant requires the CLI bridge to be updated before compilation succeeds.

## Acceptance gates

M8.3c2 closes only if all of the following are proved on the reference CI environment:

- G1: normal workspace Format / Clippy / Tests / lockfile integrity remain green;
- G2: unchanged default `execsurface observe -- COMMAND` still identifies the ptrace backend;
- G3: explicit `--backend ptrace` works without accepting `--collector`;
- G4: real privileged experimental observation succeeds when the user externally supplies the required privilege;
- G5: the CLI does not invoke privilege elevation itself;
- G6: unprivileged companion failure produces an explicit error with no target execution and no ptrace fallback;
- G7: missing collector / invalid collector path / non-zero collector exit fail closed;
- G8: fake reports with wrong backend identity, capability drift, `complete`, or `observation_complete=true` are rejected;
- G9: `learn` and `check` do not accept the experimental backend route;
- G10: normal Distribution Probe and GitHub Action Smoke remain green on the PR.

## Non-claims

- no ptrace/eBPF parity;
- no eBPF PASS authority;
- no automatic backend selection;
- no stable public eBPF distribution claim;
- no universal kernel support;
- no performance-superiority claim.

## Next gate

After M8.3c2 closes, M8.4 remains the formal fail-closed loss/lifecycle gate before M8.5 cross-backend parity.
