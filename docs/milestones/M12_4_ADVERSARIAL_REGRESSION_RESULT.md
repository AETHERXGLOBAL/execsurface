# M12.4 — Adversarial Regression — Result

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`
Tested candidate SHA: `d4f11c6e6ee73baba2357741513792dc3d57b320`
Status: **CLOSED**
Classification: **`M12_4_ADVERSARIAL_REGRESSION_PASS_BOUNDED`**

## Team

Fixed roles retained:

- **Innovative Systems Architect** — designed a bounded adversarial harness that replays the known ptrace counterexamples against the clean release candidate without importing the M10 research branch wholesale.
- **Anti-Deviation / Skeptical Reviewer** — required actual counterexample reproduction, independent target truth, fail-closed behavior, cross-kernel execution, preservation of public claim boundaries, and retention of failed harness attempts.

Dynamic specialists for this gate:

- Linux ptrace/process/thread semantics;
- fd-table lifecycle and concurrent close/open attribution;
- TOCTOU/race adversarial testing;
- evidence semantics and success-vs-attempt separation;
- CI/kernel-matrix and harness-integrity review;
- loss/completeness and session-isolation testing.

## Gate objective

Attempt to invalidate the clean portable candidate by replaying the architectural counterexamples that motivated M10/M11 and by attacking adjacent fail-closed invariants.

The gate was not allowed to pass merely because the old tests remained green. It required fresh counterexample reproduction on the clean M12 candidate.

## Final workflow evidence

Workflow: `M12 adversarial regression`
Run: `36460904199`
Candidate SHA: `d4f11c6e6ee73baba2357741513792dc3d57b320`
Conclusion: **success**

Both matrix jobs completed successfully:

- Ubuntu 22.04.5, kernel `6.8.0-1064-azure`;
- Ubuntu 24.04.5, kernel `6.17.0-1022-azure`.

Both jobs passed:

- bounded C fixture compilation;
- public claim-boundary sentinels;
- rustfmt;
- observer Clippy with `-D warnings`;
- dedicated six-case M12.4 adversarial suite;
- existing ptrace Linux regression suite;
- full-workspace tests;
- Cargo.lock integrity;
- evidence-artifact upload.

## PATH-TOCTOU replay

The fixture mutates a shared pathname buffer between `A` and `B`. The target's file byte is used as independent truth for the file actually opened. The observer's `FilePathAccess` pathname is compared against that truth.

### Ubuntu 22.04

- requested runs: **300**
- valid truth runs: **300**
- observer errors: **0**
- ptrace pathname-intent vs kernel-consumed-object mismatches: **153**
- mismatches that were non-PASS-eligible / fail-closed: **153 / 153**
- classification: `PATH_TOCTOU_REPRODUCED_AND_FAIL_CLOSED_FOR_FIXTURE`

### Ubuntu 24.04

- requested runs: **300**
- valid truth runs: **300**
- observer errors: **0**
- ptrace pathname-intent vs kernel-consumed-object mismatches: **157**
- mismatches that were non-PASS-eligible / fail-closed: **157 / 157**
- classification: `PATH_TOCTOU_REPRODUCED_AND_FAIL_CLOSED_FOR_FIXTURE`

### Interpretation

The test again proves that ptrace syscall-entry pathname metadata is not kernel-object identity.

M12 does **not** erase that criticism. The clean candidate preserves the narrower public claim: pathname access-attempt metadata under the recorded observer. In this controlled threaded fixture, clone-based ambiguity causes the observation to be incomplete, so every reproduced mismatch is non-PASS-eligible.

This is not proof that every conceivable pathname TOCTOU race is detected. The current protection in this candidate is conservative and tied to observed clone-based concurrency.

## Shared-FD race replay

The target independently logs successful `pread` truth while another thread repeatedly closes/reopens a shared fd between files `A` and `B`. The observer's attributed fd-read stream is compared to target truth.

### Ubuntu 22.04

- attempts: **3**
- attempts with target/observer divergence: **3 / 3**
- attempts that failed closed: **3 / 3**
- target-proven successful reads: **8699**
- observer-emitted attributed reads: **6557**
- classification: `SHARED_FD_COUNTEREXAMPLE_REPRODUCED_AND_FAIL_CLOSED`

### Ubuntu 24.04

- attempts: **3**
- attempts with target/observer divergence: **3 / 3**
- attempts that failed closed: **3 / 3**
- target-proven successful reads: **7627**
- observer-emitted attributed reads: **6301**
- classification: `SHARED_FD_COUNTEREXAMPLE_REPRODUCED_AND_FAIL_CLOSED`

### Interpretation

The underlying ptrace attribution limitation remains real. The M12 candidate does not claim to repair exact shared-fd attribution. It prevents the known clone-based ambiguity from remaining `complete=true` / PASS-eligible.

The guard is intentionally conservative because raw v2 does not retain exact `CLONE_FILES` flags; it may classify some clone concurrency as incomplete even when a stronger future implementation could prove the fd tables are independent.

## Additional adversarial cases

The dedicated suite also passed all of the following on both matrix systems:

1. **Loss / event budget** — a deliberately small event limit produced `event_limit_exceeded`, `complete=false`, and no silent PASS authority.
2. **Failed operation non-promotion** — a failed file open was observed as an open attempt but was not promoted into a successful fd effect for the missing file.
3. **Session isolation** — a file identity from one observation session did not leak into a later independent no-op observation.
4. **Unsupported experimental capability** — the experimental eBPF descriptor remained distinct from the default ptrace descriptor, with unsupported capabilities explicitly represented rather than silently treated as available.
5. **Claim-boundary sentinels** — README continued to state `pathname access attempts`, `observed execution-surface drift under its recorded observer and policy`, and `Incomplete evidence cannot silently become PASS.`

## Independent regression layers on the same candidate SHA

### Clean integration CI

Run `36460904247`: **success**.

It passed full-workspace Clippy, shared-FD fail-closed regression, ptrace regressions, frozen baseline serialization, legacy schema rejection, CLI compatibility/exit codes/self-service, full-workspace tests, and lockfile integrity.

### Public / Marketplace compatibility

Run `36460904105`: **success**.

It passed current published release health, candidate packaging/path install and self-service, and stable Marketplace `@v0.1` PASS/controlled-REVIEW behavior.

## Evidence artifacts

Run `36460904199` produced two evidence artifacts:

- Ubuntu 22.04: artifact `10987626484`, digest `sha256:fb6c9574831cef4c6aaefdd7bbd931e0e140da9a4a5862006390f50329904122`;
- Ubuntu 24.04: artifact `10987326477`, digest `sha256:785b419453aa25ffe8ad5452addabc3e7cda15e57b05fb34a1f777b4715b4159`.

Each artifact contains the JSON PATH-TOCTOU and shared-FD result records for its matrix environment.

## Failure-first harness record

Two earlier attempts are retained and are **not** rewritten as passes:

1. Run `36459858354` failed at `rustfmt --check` on both matrix jobs before semantic tests ran. Fixture compilation and claim-boundary sentinels had passed. This was a harness-format failure, not semantic evidence.
2. Run `36460586755` passed the dedicated adversarial suite and ptrace regressions on both systems but later failed the generic full-workspace step because the two external-fixture tests were invoked a second time without `M12_PATH_TOCTOU_BIN` / `M12_FD_SHARE_BIN`. This was a harness-integration failure.

The repair did **not** weaken the dedicated adversarial assertions. External-fixture cases now skip only when invoked outside the dedicated gate; the dedicated workflow sets the fixture variables explicitly, verifies the compiled binaries exist, requires the counterexamples to reproduce, and fails if the expected JSON evidence is not produced.

## What this gate proves

Within the tested Linux x86_64 hosted-CI matrix and bounded fixtures:

- the known PATH-TOCTOU counterexample still exists and is not silently converted into complete/PASS-eligible evidence in the controlled threaded fixture;
- the known shared-FD attribution counterexample still exists and is fail-closed in every tested attempt;
- forced event-budget loss remains fail-closed;
- a failed open is not promoted into a successful fd effect;
- observation sessions remain isolated in the tested case;
- experimental backend capability gaps remain explicit;
- the clean candidate survives ptrace, workspace, packaging, and current Marketplace compatibility regression on the same SHA.

## Explicit non-claims / limitations

This gate does **not** prove:

- universal detection of all TOCTOU races;
- exact or repaired shared-FD attribution;
- that every clone implies `CLONE_FILES` in the kernel;
- universal completeness across all syscalls, namespaces, containers, kernels, or Linux distributions;
- kernel-object authority for ptrace pathname metadata;
- hybrid/BPF-LSM product readiness;
- ptrace↔hybrid baseline equivalence;
- production readiness.

The current clone guard intentionally trades possible false incompleteness for preventing a known false-completeness class.

## M12.4 decision

**`M12_4_ADVERSARIAL_REGRESSION_PASS_BOUNDED`**

M12.4 is closed.

Next authorized gate: **M12.5 — Performance / resource regression**.
