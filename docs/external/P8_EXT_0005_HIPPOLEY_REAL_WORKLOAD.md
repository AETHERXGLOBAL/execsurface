# P8-EXT-0005 — hippoley real-workload evidence and bounded O_TMPFILE review

Status: **INTAKE PRESERVED — `INSUFFICIENT_DETAIL` FOR FORMAL A4 / `REQUEST_MORE_EVIDENCE` / A5 OPEN**  
Review date: 2026-10-16  
Review base: `main@199fcbf910ea10f38b7ff3019a7c1575a4747239`
Scientific review of PR head: `d82cd18298994ab1bdaf2758c6b11531a2aa631e`  
Authority: [P8 #114](https://github.com/AETHERXGLOBAL/execsurface/issues/114), [external workload #140](https://github.com/AETHERXGLOBAL/execsurface/issues/140), [P8-A4 protocol](../development/P8_A4_EXTERNAL_REAL_WORKLOAD_PROTOCOL.md).

## External actor, scope and independent origin

- External evaluator: [@hippoley](https://github.com/hippoley), reporting their own [Residual-Necessity](https://github.com/hippoley/Residual-Necessity) workload.
- Relationship: `USER_OR_EVALUATOR`; no AETHER X authorship of the workload was reported. Precise affiliation and assistance details remain subject to evidence qualification; no zero-assistance A1 claim.
- Evidence class: `EXTERNAL_REAL_WORKLOAD_REPORT` (A4 intake, evidence origin retained). **Current qualification: `INSUFFICIENT_DETAIL` for A4/A5 acceptance**, pending a distinct scientific decision under #114; not adoption, endorsement, standards acceptance, or v1.0.0 validation.
- Product used: immutable public `v0.1.0-alpha.6`, checksum verified according to the evaluator. Linux x86_64, GitHub-hosted Ubuntu, Python 3.11.
- Command: `python -m pytest -q`. Original run-head source commit: `ca7d14f937ce9069dc2eb742d20b7a03751ee073`; **PR merge-ref checkout SHA** recorded in the original workflow: `1283968a7d1797c45a47de5ee3c3aaa2cc399dbf`. Both source identities must be retained; these are not interchangeable.
- Original retained [run 37745730944](https://github.com/hippoley/Residual-Necessity/actions/runs/37745730944); [original report](https://github.com/AETHERXGLOBAL/execsurface/issues/140#issuecomment-6056218704): target **15 tests passed**, `doctor PASS`, `learn exit 2`, incomplete observation, **no accepted baseline** or downstream check verdict. The original result remains frozen.

## Follow-up evidence — never overwrite the original

- [Diagnostic run 37764233781](https://github.com/hippoley/Residual-Necessity/actions/runs/37764233781) and [report](https://github.com/AETHERXGLOBAL/execsurface/issues/140#issuecomment-6072350289): the external [AETHER ASSURANCE audit at PR #190](https://github.com/AETHERXGLOBAL/execsurface/pull/190#issuecomment-6220746180) reports **direct inspection** of Actions artifact `11559680172`. The captured JSON has `collection_health.state=incomplete_capability`, `raw_observation.complete=false`, **921 raw events**, **42 typed effects**, and **two** `side_effectful_open_identity_divergence` warnings for TID 2338. The two associated `file_path_access` OPEN events are sequences **722 and 723**, pathname `/tmp`, flags **4915330 = `0x4b0082`**, including Linux `O_TMPFILE`. Reported FD paths were `/tmp/#94573 (deleted)` and `/tmp/#94574 (deleted)`. The captured target outcome was **exit code 2**; the run separately lacked the `jsonschema` dependency. These facts apply to **this diagnostic run only**, not the original run.
- Warning detail as reported: successful side-effectful open used the lexical path `/tmp` while `/proc/<tid>/fd/<fd>` identified a deleted/unlinked object such as `/tmp/#94573 (deleted)`.
- The diagnostic run's later `learn` also hit an unrelated `jsonschema` dependency error. Do not count that command's failure as clean observer reproduction.
- The evaluator's [own evidence ledger](https://github.com/hippoley/Residual-Necessity/blob/main/docs/EXTERNAL_EVIDENCE.md) separately reports a later corrected diagnostic run `37869646669`: 39 tests passed, 41 typed effects, `learn exit 2`, `FAIL_CLOSED_ON_INCOMPLETE_EVIDENCE`. Differences in run/fixture and effect counts must not be conflated.
- [Workflow run 37922133708](https://github.com/hippoley/Residual-Necessity/actions/runs/37922133708) ended `SUCCESS` at the **diagnostic-workflow** level. The [AETHER ASSURANCE audit](https://github.com/AETHERXGLOBAL/execsurface/pull/190#issuecomment-6220746180) reports direct inspection of its distinct Actions artifact **11611559122**: target **exit code 0**, `raw_observation.complete=false`, **two** matching divergence warnings, **two** `/tmp` OPEN events bearing `O_TMPFILE` flags, and **142 typed effects in this artifact**. `Check same external workload` was still skipped. A green workflow is **not** baseline creation or ExecSurface `PASS`.

## AETHER RESEARCH scientific decision (2026-10-16)

**Source:** [AETHER RESEARCH scientific assessment, Issue #114 comment 6220943075](https://github.com/AETHERXGLOBAL/execsurface/issues/114#issuecomment-6220943075). Its reviewed PR HEAD was `d82cd18298994ab1bdaf2758c6b11531a2aa631e`.

- **A4 outcome:** `REQUEST_MORE_EVIDENCE`; formal qualification `INSUFFICIENT_DETAIL`; no upgrade to `QUALIFYING_EXTERNAL_EVIDENCE`.
- **A5:** `OPEN`. The preceding bounded engineering tests and A3 contribution are separate; a positive A5 closeout is not implied.
- **Accepted evidence:** genuine externally selected workload with an immutable negative/partial original result (15 target tests passed but learn exit 2, no baseline), and distinct diagnostic evidence proving `O_TMPFILE` in the later runs. Exact `O_TMPFILE` flags for the **original** are unproven and **not required** merely to qualify a bounded negative A4 report.
- **Scientific gaps needing evaluator-authored declaration:** (1) A0 relationship, affiliation and any ExecSurface contribution; (2) exact prior-help tier `ZERO_ASSISTANCE`, `DOCS_ONLY_CLARIFICATION`, `ASSISTED` or `COLLABORATIVE` and any prior guidance; (3) practical setup/privileges/docs friction, and captured-data/privacy/redaction accounting with unassessed fields explicitly marked; (4) no baseline was created/approved and check/drift were not attempted; (5) acknowledge the original run-head versus checkout merge-ref identity.
- **Non-requirement:** do not demand a new successful run, retroactive reconstruction of the original syscall flags, or replacement of the retained initial failure to consider a bounded negative result.
- **Disposition:** retain `P8-EXT-0005` as intake without formal A4 acceptance. The distinct scientific gatekeeper must reconsider only after the declaration is received. Preserve both #114 and #140 OPEN; leave PR #190 Draft/unmerged until scoped assurance re-review and all applicable acceptance gates.

## Protocol-field reconciliation (pending scientific determination)

| P8-A4 intake field | Current verified or reported evidence | Gate standing |
|---|---|---|
| Evaluator/workload | External handle `hippoley`; independently selected `hippoley/Residual-Necessity` repository and PR #2 | Origin reported; affiliation/assistance declaration not fully qualified |
| Pinned source / command | Original source `ca7d14f937ce9069dc2eb742d20b7a03751ee073`; `python -m pytest -q` | Source and command identified |
| Product / host | Checksum-verified Alpha.6 binary, GitHub-hosted Ubuntu Linux x86_64, Python 3.11 | Reported; original log confirms download/doctor stages |
| Initial baseline approval | `learn` exit 2, **no baseline created or approved** | Not applicable: no baseline; do not invent approval |
| Unchanged check / controlled drift | Not attempted after failed learning; downstream check skipped | Not applicable, not PASS |
| Completeness | Original incomplete error; diagnostics expose `incomplete_capability`, raw incomplete and O_TMPFILE open-path divergence | Confirmed diagnostic mechanism; original flags unknown |
| False PASS/REVIEW claim | Neither issued; no baseline/check verdict | No such claim evidenced |
| Setup/operational friction | Successful doctor; successful target tests; observer inability to learn; separate diagnostic `jsonschema` dependency error | Partial evidence, general fit/performance unresolved |
| Privacy / redaction | Metadata-only observer; typed JSON artifact available, with numeric TID, paths/flags; formal privacy/sanitization accounting not yet reviewed | Pending |
| Assistance/affiliation | External evaluator-controlled original run before AETHER X troubleshooting; complete A0 assistance tier and independence declaration absent | **Block formal qualification** |
| Adoption / endorsement | No adoption, endorsement or independent v1 certification demonstrated | Prohibited to claim |

## Source-level technical determination

The immutable Alpha.6 and v1.0.0 tags reference the **same** Rust blob for `crates/execsurface-observe/src/linux_ptrace.rs` (`6ec1076673b93577e2125e5e0e8abafa9445c2fb`).

At a successful `open` syscall exit the observer binds the returned FD to `/proc/<tid>/fd/<fd>`; for a side-effectful open (`O_TRUNC`, `O_CREAT`, `O_TMPFILE`), a lexical/kernel-resolved pathname mismatch emits `side_effectful_open_identity_divergence`. `Collector.warning` explicitly sets `observation.complete=false`. Raw observation schema v2 does not serialize a separate successful-open object identity suitable for promoting this path observation to complete semantic authority.

An independent Linux filesystem mechanism check confirmed that `O_TMPFILE` can yield an FD path of the `#<inode> (deleted)` form. Together with **actual `O_TMPFILE` flags read from the two later diagnostic artifacts**, the observed mechanism is better grounded than solely the reporter's prose. Neither mechanism evidence nor the diagnostic artifacts independently establish the **original frozen run's** open flags: the original run `37745730944` has no preserved downloadable Actions artifact identified in this review.

**Engineering finding, bounded to the diagnostic O_TMPFILE mechanism:** `EXPECTED_FAIL_CLOSED / KNOWN_RAW_V2_CAPABILITY_BOUNDARY`. **Scientific finding:** `INCONCLUSIVE` for precise original-run causation, avoidable false-incompleteness alternatives and formal A4 qualification. AETHER ASSURANCE issued `REVISE` for [PR #190](https://github.com/AETHERXGLOBAL/execsurface/pull/190#issuecomment-6220746180), not merge approval. No universal no-defect claim, cure, performance improvement or production adoption is inferred.

## Bounded regression acceptance — implementation is review-only

The attached Rust test additions run the actual ptrace observer and fixture (not merely compare strings):

1. `r3_otmpfile_side_effect_fails_closed_with_unrepresentable_object_identity`: create an anonymous `O_TMPFILE` object, retain lexical open intent, require `side_effectful_open_identity_divergence`, require a deleted FD path diagnostic, require `complete=false`.
2. `r3_regular_create_remains_complete_with_matching_open_identity`: ordinary `O_CREAT|O_EXCL` with a matching resolved path must remain complete without divergence warning.
3. Preserve existing symlink-truncation, ordinary truncate, and alias/FD identity regression cases unchanged.
4. Unsupported `O_TMPFILE` filesystem must be surfaced as an unsupported/failed test **rather than counted as a passing product behavior**.

**Hosted engineering verification at audited SHA `bfa6f16091db87be0dc175905d9b3a610a9fd493`**: [CI `38935601086`](https://github.com/AETHERXGLOBAL/execsurface/actions/runs/38935601086) recorded both new tests as `ok` and all **9/9** `r3_object_identity` tests passing. Stage-2 Final Gate `38935601048`, Adversarial Regression `38935601335`, P9.3 Compatibility `38935600987` and V1 RC1 Qualification `38935601082` also reported success on that SHA. The AETHER ASSURANCE independent artifact audit returned **`REVISE`** pending qualifications. Any subsequent documentation-only HEAD requires fresh source/CI verification before merge. No product algorithm, raw schema, stable baseline profile, verdict semantics, or completeness threshold has been modified.

## Remaining evidence gates and dispositions

- Raw flags and FD path details **were established for the diagnostic runs** by the AETHER ASSURANCE artifact audit; the original run's exact syscall/flags remain unverified. Do not request re-extraction of the same diagnostic evidence as though it had never been inspected. Additional artifact/metadata checks should be confined to unresolved original-run attribution, actor/assistance boundaries, and alternatives to false incompleteness.
- Retain UNKNOWN for proposition-level decisions when evidence is incomplete; no workaround may turn uncertain evidence into `PASS` or authorize a baseline.
- Preserve P8-EXT-0005 as **recorded external A4 intake, with `INSUFFICIENT_DETAIL` for formal qualification**. AETHER RESEARCH has now issued `REQUEST_MORE_EVIDENCE` at [Issue #114](https://github.com/AETHERXGLOBAL/execsurface/issues/114#issuecomment-6220943075); acquire the minimum evaluator-authored declaration above and request a new scientific gate decision before considering A5. Do **not** close #114 or #140 solely because this entry was drafted.
- Current supported public product remains stable `v1.0.0`. These Alpha.6 observations do not imply that stable v1 fixes the event; the analyzed warning path is still present there.
- No new release, compatibility promotion, security defect label, or product fix is authorized by this record.
