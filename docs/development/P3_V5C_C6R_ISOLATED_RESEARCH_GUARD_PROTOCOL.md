# ExecSurface — P3 V5-C C6R Isolated Research Guard Protocol

Date: 2026-09-29
Parent: C6 direct result / #104 / #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — RESEARCH-ONLY**

## Rationale

C6 direct integration was rejected as `C6_COMPATIBILITY_REGRESSION`: changing the default ptrace handoff would silently change the accepted alpha.4/raw-v2 contract encoded by preserved M11 tests.

C6R tests the same certificate-aware completeness hypothesis behind an explicitly isolated research-only call path while leaving the default/public observer unchanged.

This is not a relaxation or relabeling of C6. The C6 failure remains closed and retained.

## Team

Fixed roles:
1. Innovation Scientist / Systems Architect
2. Anti-Drift / Scientific Integrity Reviewer
3. Independent Falsifier / Red Team
4. Independent Critical-Milestone Reviewer

Dynamic specialists:
- Linux ptrace clone/fork/vfork routing
- fd-table lifecycle and exec semantics
- Rust API-boundary/state-machine design
- compatibility/regression engineering
- Go build concurrency / reproducibility

## Architecture hypothesis

Keep the current default path immutable:

`observe_command -> PtraceBackend -> legacy conservative shared-fd guard`

Add only an internal research finalization mode:

`linux_ptrace::observe -> internal certificate -> certificate-aware research finalizer`

The research finalizer may omit only the synthetic `shared_fd_table_ambiguity` warning when `CloneFdCertification::fully_certified()` is true. It may never clear any pre-existing warning or change an already-incomplete observation to complete.

No public API, CLI flag, raw schema, baseline schema, normalization profile, stable Action, or release behavior changes in C6R.

## Required implementation boundary

- define a private guard-policy/finalizer abstraction;
- default `PtraceBackend::observe` must explicitly select `LegacyConservative`;
- research-only tests/workflows may select `CertificateAwareResearch`;
- raw-v2 spawn mechanism remains event-label based;
- non-ptrace backends remain unchanged.

## Mandatory gates

R1. Existing M11/M12 and all workspace tests remain green **unchanged**.

R2. Uncertified clone through research mode still fails closed with `shared_fd_table_ambiguity`.

R3. Existing independent warnings (`event_limit_exceeded`, lifecycle/pairing loss, clone flags unavailable, clone3 unreadable, successful-open identity failures) are never cleared.

R4. Live private/shared/FORK-routed/VFORK-routed clone certificate falsifiers remain green.

R5. Research mode does not alter serialized raw-v2 event representation.

R6. Clone-free healthy observation remains unchanged.

R7. Pinned FZF diagnostic-only run after exactly three direct priming runs must succeed through the research finalizer with `complete=true` and zero warnings; this is not a V5 learning sample.

R8. Independent critical reviewer must verify that no default/public code path selects research mode.

## Allowed outcomes

- `C6R_ISOLATED_RESEARCH_GUARD_PASS_BOUNDED`
- `C6R_FALSE_COMPLETENESS_FOUND`
- `C6R_PUBLIC_PATH_LEAKAGE`
- `C6R_REAL_WORKLOAD_STILL_INCOMPLETE`
- `C6R_INCOMPLETE_EVIDENCE`

Only bounded PASS can authorize a newly preregistered V5-R2 research campaign. No public integration follows automatically.
