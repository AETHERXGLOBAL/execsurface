# M11.3 — Shared-FD False-Completeness Hardening Result

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **CLOSED — PASS**

## Accepted evidence

Accepted implementation HEAD before closeout note: `f62bcc7d6acf5834c123558e1c84f4d3594cb3bb`.

Accepted GitHub Actions run: `36447403629` — **SUCCESS**.

The accepted gate ran and passed:

- M11 formatting checks;
- full-workspace Clippy with warnings denied;
- authority-model tests;
- ptrace pathname-authority bridge tests;
- shared-FD ambiguity runtime tests;
- frozen baseline-v2 digest compatibility test;
- full locked workspace test suite;
- Cargo.lock integrity.

## Closed invariant

The M10.2 false-completeness state is no longer silently accepted on this research branch.

When clone-based concurrency is observed under the current raw-v2 contract, ExecSurface conservatively fails closed because raw v2 does not retain the `CLONE_FILES` flags required to certify that fd lifecycle attribution was unambiguous.

The observer now emits:

`shared_fd_table_ambiguity`

and sets:

`complete=false`

The internal handoff classifies this as:

`IncompleteAmbiguity`

which is not PASS-eligible.

## Runtime regression evidence

A controlled threaded fixture proves that clone-based concurrency now becomes incomplete with exactly one ambiguity warning.

A no-clone control proves the warning is not invented for a simple single-process run.

The pre-existing ptrace clone/fd regression still requires the expected fd-read events to be observed, but no longer asserts the scientifically invalid `complete=true` claim killed by M10.2.

## Conservative boundary

M11.3 does **not** prove that all clone events use `CLONE_FILES`.

Because the existing raw-v2 observation schema does not preserve clone flags, this gate deliberately treats observed clone-based concurrency as an ambiguity trigger until a stronger explicit clone-flag contract is implemented and proved.

This can reduce PASS eligibility for some concurrent workloads. That effect must be measured before release integration.

## Preserved boundaries

No change was made to:

- `main`;
- `v0.1.0-alpha.3`;
- Marketplace behavior;
- baseline-v2 serialization or digest;
- release tags or stable Action channel.

## Negative evidence retained

The first full-workspace M11.3 run exposed one historical test that still asserted `complete=true` for clone/shared-fd execution. The test was corrected to preserve event-capture checks while adopting the M10.2 fail-closed semantics. No acceptance threshold was weakened.

## Decision

**M11.3 CLOSED — PASS.**
