# ExecSurface — P3 V5-C C6 Certificate-Aware Guard Integration Protocol

Date: 2026-09-29
Parent: #104 / #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO PUBLIC RELEASE AUTHORIZATION**

## Objective

Test whether the legacy post-collection `shared_fd_table_ambiguity` warning can be suppressed **only** when the internal ptrace clone/fd certificate proves every relevant clone-origin transition is causally correlated and fd-table relation is known, while preserving all existing fail-closed behavior for absent/incomplete/contradictory certification and every independent observer-health failure.

This is a development-branch integration experiment. Public `v0.1.0-alpha.4`, `main`, raw/canonical/baseline v2 bytes, and stable `@v0.1` remain unchanged.

## Team

Fixed roles:
1. Innovation Scientist / Systems Architect — design the narrowest proof-gated integration, not a generic guard deletion.
2. Anti-Drift / Scientific Integrity Reviewer — preserve v2 meaning, frozen V5 evidence, and all independent incompleteness paths.
3. Independent Falsifier / Red Team — own false-completeness, forged/partial certificate, loss, lifecycle, and compatibility attacks.

Dynamic C6 specialists:
- Linux ptrace / clone / clone3 / fork/vfork routing
- fd-table lifecycle / exec / CLOEXEC / dup / close / reuse
- Rust state-machine and API-boundary design
- observer completeness / proof-carrying evidence
- regression / reproducibility engineering

## Integration rule

For ptrace only:

1. collect `PtraceObservation { observation, clone_fd_certification }`;
2. if `clone_fd_certification.fully_certified()` is **true**, do not add the synthetic post-collection `shared_fd_table_ambiguity` warning merely because raw v2 contains `SpawnMechanism::Clone`;
3. if the certificate is false/ambiguous/absent, preserve the legacy guard exactly;
4. existing raw warnings remain authoritative and are never cleared by the certificate;
5. an observation already marked incomplete remains incomplete;
6. no raw event, path, identity, or effect may be invented or rewritten to obtain completeness;
7. public raw-v2 `ProcessSpawn.mechanism` remains event-label based;
8. non-ptrace backends remain unchanged.

The certificate is a prerequisite for suppressing one conservative synthetic warning; it is not a universal completeness certificate.

## Mandatory falsification cases

### G1 — clone-free healthy observation
No regression; remains complete when otherwise healthy.

### G2 — certified clone origin without `CLONE_FILES`
May avoid the generic ambiguity warning while retaining raw-v2 spawn representation.

### G3 — certified `CLONE_FILES`
May avoid the generic ambiguity warning only after shared relation is positively certified.

### G4 — clone origin routed as FORK
Internal certificate follows syscall origin; raw-v2 mechanism remains `Fork`; no authority laundering.

### G5 — clone origin routed as VFORK
Same boundary as G4.

### G6 — missing pending clone evidence
Certification false; legacy ambiguity/incompleteness remains fail-closed.

### G7 — clone3 flags unreadable
Certification false; existing warning remains and observation remains incomplete.

### G8 — resource/event truncation
Certificate cannot clear or override event-limit/resource incompleteness.

### G9 — syscall pairing/lifecycle loss
Certificate cannot clear or override lifecycle warnings.

### G10 — successful-open identity failure
Certificate cannot clear unrelated file-identity incompleteness.

### G11 — shared fd close/dup/reuse/exec semantics
Existing state-machine adversarial fixtures must remain green.

### G12 — raw-v2 compatibility
Serialized public raw observation schema and historical `ProcessSpawn.mechanism` interpretation remain unchanged.

### G13 — legacy M11/M12 false-completeness regression
Existing shared-FD fail-closed fixtures must not silently become complete when no positive internal certificate is available.

### G14 — pinned FZF diagnostic
After G1–G13 pass, exactly one diagnostic-only public-path observation may be run against the pinned FZF workload after exactly three direct priming runs. It must be complete with zero warnings. It does not count as a V5 learning sample.

## Static / regression gates

Required before a bounded PASS:
- `cargo fmt --all -- --check`;
- clippy `-D warnings` for affected crates;
- all `execsurface-observe` unit/integration tests;
- full workspace test suite;
- dedicated live ptrace FORK/VFORK/private/shared harnesses;
- explicit negative tests proving an incomplete certificate still invokes the legacy guard;
- diff check confirming no public schema/version change;
- no force push/history rewrite.

## Allowed outcomes

- `C6_CERTIFICATE_AWARE_GUARD_PASS_BOUNDED`
- `C6_FALSE_COMPLETENESS_FOUND`
- `C6_COMPATIBILITY_REGRESSION`
- `C6_REAL_WORKLOAD_STILL_INCOMPLETE`
- `C6_INCOMPLETE_EVIDENCE`

Only `C6_CERTIFICATE_AWARE_GUARD_PASS_BOUNDED` may authorize a newly preregistered V5-R2 campaign. It does not authorize public release integration by itself.
