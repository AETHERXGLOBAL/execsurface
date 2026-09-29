# ExecSurface — P3 V5-C C6 Direct Integration Result

Date: 2026-09-29
Parent: #104 / #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — C6_COMPATIBILITY_REGRESSION**

## Scope

This result evaluates one specific integration hypothesis: make the default ptrace backend skip the synthetic post-collection `shared_fd_table_ambiguity` guard whenever the new internal clone/fd certificate is positively complete.

The hypothesis is rejected for the current public-v2/default-backend contract.

No C6 implementation commit was accepted. Public `v0.1.0-alpha.4`, `main`, stable `@v0.1`, raw/canonical/baseline v2 semantics, and the frozen failed V5 Stage L evidence remain unchanged.

## Retained execution history

### Run `36564238281`
Tooling-only failure before semantic testing: patch driver expected stale ptrace-handoff text.

### Run `36564356709`
Tooling-only failure before semantic testing: negative-test insertion anchor was stale.

### Run `36564453022`
Patch applied and formatting passed, but the temporary negative fixture used a nonexistent `RawEvent.executable` field. Clippy/build rejected the fixture before scientific testing.

### Run `36564614880` — semantic attempt
Passed before regression gate:
- rustfmt PASS;
- clippy `-D warnings` PASS;
- negative uncertified-clone guard falsifier PASS;
- live clone-origin→FORK falsifier PASS;
- live clone-origin→VFORK falsifier PASS;
- live ordinary private/shared clone harness PASS.

Full `execsurface-observe` regression then failed on four preserved M11 tests:
1. `clone_based_threading_fails_closed_for_fd_lifecycle_completeness`;
2. `known_clone_without_clone_files_is_live_but_v2_still_fails_closed`;
3. `shared_fd_reuse_tracks_replacement_path_but_stays_fail_closed`;
4. `exec_from_shared_fd_table_preserves_inherited_fd_identity_but_stays_fail_closed`.

The no-clone control and resource-truncation fail-closed test remained green.

## Interpretation

The new internal certificate has bounded evidence that it can distinguish clone/fd relations, but applying it directly to the current default public-v2 observer changes the historical alpha.4 contract: sessions that raw v2 deliberately treats as incomplete would become complete without changing the public evidence schema.

That is a compatibility/semantic-boundary regression, even though the internal collector evidence is stronger.

The correct response is **not** to rewrite or weaken the M11 tests. Those tests encode the accepted alpha.4/v2 contract and remain authoritative for the public/default path.

Formal decision:

`C6_COMPATIBILITY_REGRESSION`

## Authorized next step

A separate research-only path may test certificate-aware completion while the default/public observer continues to use the legacy conservative guard unchanged.

This successor must:
- use the same internal certificate and falsification criteria;
- leave `observe_command` / default `PtraceBackend` behavior unchanged;
- leave all M11/M12 public-contract tests unchanged and green;
- expose no silent public-v2 reinterpretation;
- use an explicitly research-scoped call path/workflow for any future V5 requalification;
- retain all C6 failures as evidence.

No public integration or release is authorized.
