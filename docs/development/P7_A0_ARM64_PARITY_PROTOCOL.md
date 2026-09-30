# ExecSurface — P7-A0 Linux arm64 Research Parity Protocol

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Predecessor: `P6_FACTUAL_COMPARISON_COMPLETE_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO A0 RESULT YET**

## 1. Question

Can the exact public alpha.4 source build and execute natively on pinned GitHub-hosted Linux x86_64 and arm64 runners while preserving the same fail-closed behavioral classes for the frozen S0-S3 workload, without changing public semantics or architecture-specific acceptance thresholds?

This gate tests research portability/evidence parity. It does not expand public support.

## 2. Hypothesis

If the alpha.4 ptrace/evidence implementation is architecture-portable at the semantic level, then native builds from the exact same source should:

- produce a valid S0 baseline independently on each architecture;
- native-PASS S0 on each architecture;
- never native-PASS S1 process expansion;
- never native-PASS S2 when the observer reports incomplete/lost evidence;
- never native-PASS S3 file-write expansion;
- retain enough native evidence to explain each non-PASS outcome;
- require no architecture-specific whitelist, threshold or semantic reinterpretation.

Baseline digests and raw event counts are **not** required to be equal across architectures.

## 3. Fixed roles

1. **Innovation Scientist / Systems Architect** — test whether the evidence contract is portable rather than merely whether Rust compiles.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks public-support inflation, architecture-specific exemptions, byte-equality theater and post-result criterion changes.
3. **Independent Falsifier / Red Team** — attacks syscall/ABI handling, observer completeness, architecture-specific false PASS and unsupported-state laundering.
4. **Independent Critical-Milestone Reviewer** — verifies exact source/runner/workload identity, retained raw evidence and decision wording.

Dynamic specialists:
- Linux arm64 ABI / ptrace;
- Rust portability;
- runtime evidence semantics / PL;
- GitHub Actions runner/reproducibility;
- release/distribution.

## 4. Immutable boundaries

P7-A0 MUST NOT:

- modify public `v0.1.0-alpha.4`, stable `@v0.1`, `main`, public v2 semantics or public default observer;
- upload an arm64 asset to the alpha.4 release;
- call arm64 publicly supported from this research gate;
- claim cross-architecture baseline interchangeability;
- require baseline digest equality;
- add an arm64-only whitelist or loosen an acceptance threshold;
- treat unsupported, ambiguous, lost or incomplete as PASS;
- delete failed runs or negative evidence.

## 5. Frozen inputs

Public alpha.4 source:
`48e0b9a0707553349e75e97a9dfa096d13f9ab5d`

Public alpha.4 release scope at gate creation:
- Linux `x86_64` public support only;
- release contains x86_64 binary/checksum assets only.

Workload:
- path: `experiments/p6-competitive-falsification/workload.sh`;
- blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`.

Toolchain:
- Rust `1.90.0`;
- locked dependencies from alpha.4 `Cargo.lock`.

Native hosted runners:
- x86_64: `ubuntu-24.04`;
- arm64: `ubuntu-24.04-arm`.

The runner labels are pinned rather than using `ubuntu-latest`.

## 6. Build protocol

For each architecture independently:

1. checkout the development branch with full history;
2. verify `v0.1.0-alpha.4^{commit}` and `v0.1^{commit}` equal the frozen alpha.4 source;
3. verify the workload blob;
4. create a detached worktree at the exact alpha.4 source;
5. install/use Rust 1.90.0;
6. run `cargo +1.90.0 test --locked --workspace` from the alpha.4 worktree;
7. run `cargo +1.90.0 build --locked --release --bin execsurface`;
8. record `uname`, runner architecture, `rustc -vV`, target host, binary digest and version.

A compilation/test failure on arm64 is scientific portability evidence and must be retained. Do not patch alpha.4 inside this gate.

## 7. Runtime corpus

Each architecture learns its own S0 baseline exactly once with the natively built alpha.4 binary.

Then run unchanged:
- S0_CONTROL;
- S1_CHILD_PROCESS_EXPANSION;
- S2_NETWORK_DESTINATION_EXPANSION;
- S3_FILE_WRITE_EXPANSION.

Retain per scenario:
- native exit code;
- JSON report when produced;
- Markdown report when produced;
- stdout/stderr;
- baseline identity and effect count;
- raw reason for any ERROR/INCOMPLETE state;
- SHA256 manifest.

## 8. Frozen semantic acceptance

### S0 control
Required on both architectures:
- native exit `0`;
- result PASS;
- baseline exists and validates under the same alpha.4 binary.

### S1 process expansion
Required on both architectures:
- must not native-PASS;
- accepted outcomes are REVIEW/BLOCK or fail-closed ERROR/INCOMPLETE;
- if a trusted report is produced, the retained evidence must expose process-execution drift or a concrete reason why the observation is not trusted.

### S2 network expansion
Required on both architectures:
- must not native-PASS if evidence is incomplete/lost;
- REVIEW/BLOCK or fail-closed ERROR/INCOMPLETE are admissible factual outcomes;
- architecture-specific completeness differences must remain visible.

Exact S2 verdict equality is not required because public alpha.4 already has a bounded incompleteness path under the x86_64 P6 corpus.

### S3 file-write expansion
Required on both architectures:
- must not native-PASS;
- accepted outcomes are REVIEW/BLOCK or fail-closed ERROR/INCOMPLETE;
- trusted reports should expose file mutation/open/write drift, otherwise retain the explicit fail-closed reason.

## 9. Parity classification

A0 may close as `P7_A0_ARM64_PARITY_ESTABLISHED_BOUNDED` only if:

1. exact alpha.4 source builds/tests natively on both pinned architectures;
2. S0 PASSes on both;
3. S1-S3 produce zero false PASS on both;
4. any architecture-specific divergence remains fail-closed and explicitly evidenced;
5. no threshold/whitelist/public semantic change was required;
6. immutable public tags/release scope remain unchanged.

If arm64 builds/runs but proposition-level evidence meaning diverges in a way that prevents parity classification, close as:
`P7_A0_ARM64_SOURCE_PORTABLE_EVIDENCE_DIVERGENCE`.

If the exact alpha.4 source cannot build or execute natively on arm64 without source modification, close as:
`P7_A0_ARM64_NOT_PORTABLE`.

If runner/infrastructure evidence is insufficient, close as:
`P7_A0_INCOMPLETE`.

## 10. Promotion boundary

Even a positive A0 result is research-only. Public arm64 support would require a separate promotion/release gate including packaging, distribution, zero-assistance reproduction, and public compatibility evidence.

## 11. Next phase after A0

Only after A0 closes, P7 may consider a GitLab CI adapter that reuses the same behavioral/evidence semantics. Platform count alone is not a success criterion.
