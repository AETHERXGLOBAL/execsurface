# ExecSurface — Stage-2 R7 Hostile Cross-Gate Destruction Protocol

Date: 2026-10-08  
Parent remediation: #165 / PR #166  
Predecessors: R0-R6  
Status: **PREREGISTERED — HOSTILE DESTRUCTION / FAIL-FIRST**

## Objective

R7 does not add a feature. It attacks the composed guarantees established by R2-R6 and searches for cross-gate counterexamples that individual gate tests can miss.

The first mandatory attack surface is the boundary between:

- R4 baseline/policy custody;
- target execution;
- verdict-output materialization.

The governing question is:

> Can a caller select a verdict output path that aliases a trusted input artifact, another verdict output, or a workload-written object, causing ExecSurface itself to overwrite authoritative or workload-owned state while still returning a normal policy verdict?

If yes, the remediation remains RED even when custody verification itself was correct.

## Fixed review roles

1. **Execution / Runtime Correctness** — owns minimal implementation only after a counterexample is reproduced.
2. **Innovation / Architecture** — looks for composed-state failures rather than re-running isolated unit cases.
3. **Anti-Drift / Goal Alignment** — forbids turning R7 into generic hardening or expanding unsupported observer claims.
4. **Independent Destruction / Falsification** — owns aliasing, timing and state-mutation attacks.
5. **Independent Critical Reviewer** — requires retained RED evidence, unchanged assertions and exact-source GREEN reproof.

## Attack class A — trusted-artifact output collision

The following must be rejected before target execution and without modifying the protected artifact:

1. `--json-output` is the baseline path directly.
2. an output path is a symlink alias to the policy.
3. an output path is a hardlink alias to the baseline.

Custody pins being correct does not authorize ExecSurface to mutate the authorized artifact.

## Attack class B — output/output alias

JSON and Markdown outputs must not designate the same filesystem identity. A normal verdict must not be returned after one output silently overwrites the other.

The collision must be rejected before target execution when knowable at preflight.

## Attack class C — workload/output collision

If the target creates or writes the selected verdict-output object during the observed run, ExecSurface must not overwrite those workload-owned bytes after observation.

This collision can be known only after runtime evidence exists. Required behavior:

- target execution may already have occurred;
- report materialization must fail explicitly;
- workload-owned bytes must remain unchanged;
- the command must not return PASS/REVIEW/BLOCK as though report output succeeded.

## Identity rule

A path comparison that relies only on spelling is insufficient.

The destruction corpus must include:

- direct path equality;
- symlink alias equality;
- hardlink inode equality.

For paths that do not yet exist, lexical absolute identity with resolved existing parent is acceptable for the preflight boundary. Existing filesystem objects must additionally be compared by object identity on the supported Linux boundary.

## Frozen fail-first acceptance corpus

Before implementation, integration tests must encode all five attacks above and show failure against current source.

Assertions may not be weakened after implementation.

## Non-goals

R7 does not:

- reinterpret Alpha.5;
- expand observer syscall coverage;
- change policy v1/v2/v3 meaning;
- grant authority to signatures or backend names;
- change verdict precedence;
- authorize merge or release.

## Close condition

R7 may become GREEN only when:

- all fail-first counterexamples are retained;
- direct, symlink and hardlink protected-artifact collisions fail before target execution;
- JSON/Markdown alias collision fails before target execution;
- workload/output collision preserves workload bytes and returns ERROR;
- R0-R6 regression suites remain unchanged and green;
- CI, Adversarial Regression, P9.3 and Ptrace Lifecycle pass on the same qualified SHA;
- an independent destruction pass finds no equivalent alias bypass in the bounded output surfaces.
