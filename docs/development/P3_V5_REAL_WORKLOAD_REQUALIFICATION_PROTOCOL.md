# ExecSurface — P3 V5 Real-Workload Requalification Protocol

Date: 2026-09-29
Tracking: #103
Parent evidence: #62
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **V5 PREREGISTERED — NO PUBLIC INTEGRATION AUTHORIZED**

## Objective

Re-run the exact pinned real workload that exposed Issue #62 and test whether the V1/V2/V3 research stack removes only the bounded GCC ephemeral filename variance while preserving genuine non-target variability.

This is a value/requalification gate, not a release gate. Public `v0.1.0-alpha.4`, canonical schema v2, normalization profile v3 and single-run baseline/check semantics remain unchanged.

## Team

Fixed roles:
- Innovation Scientist / Systems Architect;
- Anti-Drift / Scientific Integrity Reviewer;
- Independent Falsifier / Red Team.

Dynamic specialists:
- Go build/test and GCC producer semantics;
- Rust research integration;
- multi-run reproducibility and set-stability;
- provenance / content-addressed evidence;
- canonicalization and exact-effect identity;
- CI real-workload harness design.

## Frozen historical evidence

Issue #62 was produced by workflow:
`.github/workflows/m9-fzf-stability-discriminator.yml`

Historical discriminator run:
`36279728908`

Historical source:
`bf2835013ffcc1d2d5f8af26d237846bcf44f04c`

Pinned upstream workload:
`junegunn/fzf@b1be3a8be1b833ce5b92fbbac11637643d60a046`

Historical artifact:
`10918895441`

Historical artifact digest:
`sha256:3247cfc896efb16899c991337d374b6783d09e8ff66f2d6d489cfc5e46bb4469`

The exact historical workload command was:
`cd <pinned-fzf-worktree> && go test ./... >/dev/null 2>&1`

The historical harness ran three direct priming executions before learning. Six unchanged single-baseline checks then produced finding counts `11,14,15,13,16,19`. Four GCC ephemeral assembly-file findings were stable residual noise in each check, while Go cache/module/stdlib read sets varied genuinely.

## V5 execution design

1. Checkout the current development branch and the exact pinned fzf revision.
2. Verify the fzf checkout is at the frozen commit and tracked source is clean.
3. Build current development ExecSurface without changing public behavior.
4. Run the exact `go test ./...` workload directly three times as priming, matching the historical discriminator setup.
5. Collect **three independent `execsurface learn` runs** of the unchanged command. Each lockfile is separately preserved and content-digested.
6. Verify all three lockfiles are complete/valid and profile-compatible under public normalization profile v3.
7. Feed the three raw locks into the V1 variance analyzer using their original evidence digests.
8. For each raw surface, identify V2-eligible GCC ephemeral identities under the accepted producer/role contract.
9. Apply the V2 research projection only to eligible GCC identities, rebuild research-only profile-v4 locks, and feed those derived locks into V1 while retaining the original run evidence digests as provenance.
10. Build a V3 contract from the projected report with an **empty explicit acceptance selection**.

## Required evidence classes

### Targeted GCC variance

V5 must observe at least one raw variable candidate whose exact target path belongs to a V2-eligible GCC ephemeral identity from one of the real learning runs.

After V2 projection:
- those random identities must no longer survive as distinct random-path variable candidates;
- canonical `$TMP/cc<gcc-ephemeral>.s` effects may become invariant only if the real effect role is present in all learning runs.

### Non-target variability

V5 must observe at least one raw variable candidate that is **not** a V2-eligible GCC identity.

Every exact non-target variable candidate must retain the same support count and the same original source-evidence provenance after the GCC-only projection. V2 is not allowed to collapse Go cache, module-source or standard-library effects.

### Authorization boundary

The projected V3 contract must contain **zero accepted variable effects** when the explicit selection is empty, regardless of recurrence frequency.

## Fail-closed requirements

V5 is not a positive result if any of these occur:
- fewer than three valid real learning runs;
- fzf revision mismatch or tracked source mutation;
- incomplete observation/baseline generation;
- incompatible tool/command/platform/observer/schema/normalization profile across raw runs;
- no real targeted GCC variance reproduced;
- no real non-target variability reproduced;
- any exact non-target variable effect disappears, changes support, or loses original evidence provenance solely because of V2 projection;
- any raw random GCC identity remains as a projected random-path candidate;
- empty V3 selection yields any accepted-variable effect.

No thresholds may be relaxed after results are observed. No Go cache/module/stdlib wildcard normalization may be introduced in V5.

## Required artifacts

The CI artifact must retain at minimum:
- environment and frozen revisions;
- direct-prime stdout/stderr/status;
- three raw baseline lockfiles;
- SHA-256 digest for each raw baseline evidence file;
- machine-readable V5 analysis report;
- human-readable summary;
- sealed `SHA256SUMS` for all evidence files.

## Allowed decisions

- `P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED`
- `P3_V5_TARGETED_VARIANCE_NOT_REPRODUCED`
- `P3_V5_NON_TARGET_VARIANCE_COLLAPSED`
- `P3_V5_REAL_WORKLOAD_EVIDENCE_INCOMPLETE`

Even `P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED` does not authorize public integration. A separate integration/release-compatibility gate would still be required.
