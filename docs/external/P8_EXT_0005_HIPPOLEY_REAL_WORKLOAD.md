# P8-EXT-0005 — hippoley real-workload evidence and bounded O_TMPFILE review

Status: **EXTERNAL A4 INTAKE PROPOSED / SCIENTIFIC CLASSIFICATION INCONCLUSIVE**  
Review date: 2026-10-16  
Review base: `main@199fcbf910ea10f38b7ff3019a7c1575a4747239`  
Authority: [P8 #114](https://github.com/AETHERXGLOBAL/execsurface/issues/114), [external workload #140](https://github.com/AETHERXGLOBAL/execsurface/issues/140), [P8-A4 protocol](../development/P8_A4_EXTERNAL_REAL_WORKLOAD_PROTOCOL.md).

## External actor, scope and independent origin

- External evaluator: [@hippoley](https://github.com/hippoley), reporting their own [Residual-Necessity](https://github.com/hippoley/Residual-Necessity) workload.
- Relationship: `USER_OR_EVALUATOR`; no AETHER X authorship of the workload was reported. Precise affiliation and assistance details remain subject to evidence qualification; no zero-assistance A1 claim.
- Evidence class: `EXTERNAL_REAL_WORKLOAD_REPORT` (A4 candidate), not adoption, endorsement, standards acceptance, or v1.0.0 validation.
- Product used: immutable public `v0.1.0-alpha.6`, checksum verified according to the evaluator. Linux x86_64, GitHub-hosted Ubuntu, Python 3.11.
- Command: `python -m pytest -q`. Original external source commit: `ca7d14f937ce9069dc2eb742d20b7a03751ee073`.
- Original retained [run 37745730944](https://github.com/hippoley/Residual-Necessity/actions/runs/37745730944); [original report](https://github.com/AETHERXGLOBAL/execsurface/issues/140#issuecomment-6056218704): target **15 tests passed**, `doctor PASS`, `learn exit 2`, incomplete observation, **no accepted baseline** or downstream check verdict. The original result remains frozen.

## Follow-up evidence — never overwrite the original

- [Diagnostic run 37764233781](https://github.com/hippoley/Residual-Necessity/actions/runs/37764233781) and [report](https://github.com/AETHERXGLOBAL/execsurface/issues/140#issuecomment-6072350289): `collection_health.state=incomplete_capability`, `warning_codes=[side_effectful_open_identity_divergence]`, `raw_observation.complete=false`, 42 typed effects reported. Its evidence was uploaded as GitHub Actions artifact `11559680172`. **The artifact's bytes have not been independently inspected in this review.**
- Warning detail as reported: successful side-effectful open used the lexical path `/tmp` while `/proc/<tid>/fd/<fd>` identified a deleted/unlinked object such as `/tmp/#94573 (deleted)`.
- The diagnostic run's later `learn` also hit an unrelated `jsonschema` dependency error. Do not count that command's failure as clean observer reproduction.
- The evaluator's [own evidence ledger](https://github.com/hippoley/Residual-Necessity/blob/main/docs/EXTERNAL_EVIDENCE.md) separately reports a later corrected diagnostic run `37869646669`: 39 tests passed, 41 typed effects, `learn exit 2`, `FAIL_CLOSED_ON_INCOMPLETE_EVIDENCE`. Differences in run/fixture and effect counts must not be conflated.
- [Workflow run 37922133708](https://github.com/hippoley/Residual-Necessity/actions/runs/37922133708) ended `SUCCESS` at the **diagnostic-workflow** level, with downstream `Check same external workload` skipped. This is **not** baseline success or ExecSurface `PASS`.

## Source-level technical determination

The immutable Alpha.6 and v1.0.0 tags reference the **same** Rust blob for `crates/execsurface-observe/src/linux_ptrace.rs` (`6ec1076673b93577e2125e5e0e8abafa9445c2fb`).

At a successful `open` syscall exit the observer binds the returned FD to `/proc/<tid>/fd/<fd>`; for a side-effectful open (`O_TRUNC`, `O_CREAT`, `O_TMPFILE`), a lexical/kernel-resolved pathname mismatch emits `side_effectful_open_identity_divergence`. `Collector.warning` explicitly sets `observation.complete=false`. Raw observation schema v2 does not serialize a separate successful-open object identity suitable for promoting this path observation to complete semantic authority.

A small *independent Linux filesystem mechanism check* confirmed that `O_TMPFILE` can yield an FD path of the `#<inode> (deleted)` form; that check does **not** reproduce ExecSurface end-to-end or establish the exact flags used in hippoley's original run.

**Provisional finding:** `EXPECTED_FAIL_CLOSED / KNOWN_RAW_V2_CAPABILITY_BOUNDARY` for the reported path/FD divergence, with scientific result `INCONCLUSIVE` on whether an avoidable false-incompleteness improvement exists. No defect, cure, performance improvement or production adoption is claimed.

## Bounded regression acceptance — implementation is review-only

The attached Rust test additions run the actual ptrace observer and fixture (not merely compare strings):

1. `r3_otmpfile_side_effect_fails_closed_with_unrepresentable_object_identity`: create an anonymous `O_TMPFILE` object, retain lexical open intent, require `side_effectful_open_identity_divergence`, require a deleted FD path diagnostic, require `complete=false`.
2. `r3_regular_create_remains_complete_with_matching_open_identity`: ordinary `O_CREAT|O_EXCL` with a matching resolved path must remain complete without divergence warning.
3. Preserve existing symlink-truncation, ordinary truncate, and alias/FD identity regression cases unchanged.
4. Unsupported `O_TMPFILE` filesystem must be surfaced as an unsupported/failed test **rather than counted as a passing product behavior**.

Acceptance requires CI/real Linux x86_64 ptrace execution on the exact PR SHA, test result retention, and an independent engineering review. These tests are **not yet reported as passing**. No product algorithm, raw schema, stable baseline profile, verdict semantics, or completeness threshold is modified here.

## Remaining evidence gates and dispositions

- Compare original and corrected run conditions; independently inspect redacted raw artifact if accessible; capture the real `open/openat/openat2` flags and kernel dev/inode/FD information before claiming a precise workload-specific root cause.
- Retain UNKNOWN for proposition-level decisions when evidence is incomplete; no workaround may turn uncertain evidence into `PASS` or authorize a baseline.
- Preserve P8-EXT-0005 as a **proposed qualifying A4 external use record** for independent acceptance. Review actor independence/assistance and raw evidence fidelity before A5 disposition. Do **not** close #114 or #140 solely because this entry was drafted.
- Current supported public product remains stable `v1.0.0`. These Alpha.6 observations do not imply that stable v1 fixes the event; the analyzed warning path is still present there.
- No new release, compatibility promotion, security defect label, or product fix is authorized by this record.
