# M6.5 — Observation Backend Decision

Date: 2026-09-26

Status: **ACCEPTED**

## Decision

Keep the native Linux ptrace implementation as the **reference correctness backend for M7**.

Do not replace it with eBPF before external-workload evidence.

Do not treat a future eBPF backend as evidence-equivalent merely because it emits similar event names.

## Evidence

Final performance run:

https://github.com/AETHERXGLOBAL/execsurface/actions/runs/36188370027

Final controlled 128-file burst:

- direct median: 18.944 ms;
- observed median: 70.513 ms;
- median absolute ptrace overhead: 51.569 ms;
- median slowdown: 3.722×;
- median observed events: 524;
- canonicalization median: 0.571 ms;
- observation remained complete.

The relative slowdown is material on a short synthetic command, but the absolute cost did not cross the predeclared 500 ms pre-M7 trigger.

## Why ptrace remains the reference backend

M6.5 strengthened ptrace semantics to include:

- syscall entry/exit pairing;
- successful open -> fd identity;
- actual fd-attributed read/write;
- fd duplication/close lifecycle;
- fork inheritance;
- CLONE_FILES sharing;
- close-on-exec semantics;
- openat/openat2 trace-time path handling;
- explicit event-budget incompleteness;
- fault-injection fail-closed behavior.

Those semantics are now part of the evidence contract.

An alternative backend must prove comparable completeness/privacy semantics rather than only lower latency.

## eBPF authorization gate

A production eBPF fast-path becomes justified when an external workload reproduces either:

1. median slowdown > 2× on a command whose direct median is at least 100 ms; or
2. median absolute observer overhead > 500 ms.

Before an eBPF backend may produce PASS, it must also provide:

- explicit lost-event accounting;
- fail-closed handling for event loss;
- equivalent metadata-only privacy boundaries;
- versioned backend capability metadata;
- M4 comparability separation from ptrace;
- regression evidence against reference ptrace on controlled workloads.

## Non-claim

This decision does not claim ptrace is low-overhead or production-optimal.

It establishes ptrace as the current correctness reference and defers optimization until a real workload demonstrates that the additional backend complexity is justified.
