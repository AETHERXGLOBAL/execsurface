# M10.3C — Scientific Integrity Review

Date: 2026-09-28
Tracking: #88
Status: **CLOSED — NON-INTERPRETABLE FOR ORIGINAL M10.3 PROPOSITION**

## Decision

Workflow run `36435456534` is retained as diagnostic infrastructure evidence only. It MUST NOT close or satisfy the original M10.3 BPF-LSM file-object proposition.

## Reason 1 — proposition lineage drift

The authoritative M10.3 protocol froze a narrow proposition: one session-registered target, one controlled open after a barrier, and target-side `fstat(2)` ground truth on the returned fd.

M10.3C incorrectly described the older two-file / 100-iteration prototype as the "unchanged" proposition. That prototype establishes parent-side object identities before the target run and is therefore a different fixture.

Anti-Deviation review rejects treating those criteria as equivalent.

## Reason 2 — hook ABI mismatch discovered before semantic execution

Run `36435456534` booted a BPF-LSM-capable guest successfully, but the verifier rejected the research program before attachment:

`func 'bpf_lsm_file_open' doesn't have 3-th argument`

The prototype declared `file_open` with an extra `mask` argument. Linux `file_open` has one native hook argument (`struct file *file`); a BPF LSM program additionally receives the chained return value. The prototype therefore did not reach a semantic test.

Frozen run classification emitted `BPF_LSM_FILE_INFRA_FAILURE`.

Artifact: `10975880099`
Artifact digest: `sha256:8cb5ef3e8b70bbd3f8c4184d158d6f94deceb1910f569c9a11f01f1ee8295ece`

## Consequence

A new gate, M10.3D, is required. It will:

1. restore the original one-open / target-`fstat` proposition;
2. correct only the hook ABI necessary to execute that proposition;
3. keep BPF LSM audit-only;
4. keep explicit session membership and drop accounting;
5. record the first scientifically interpretable result before any widening.

No public runtime semantics, `main`, Marketplace behavior, ptrace authority, baseline compatibility, or BPF product authority changes are authorized.