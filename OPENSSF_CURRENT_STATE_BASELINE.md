# OpenSSF Current State Baseline

Date: 2026-09-28
Purpose: state the public ExecSurface facts that govern OpenSSF / Linux Foundation technical engagement. Historical outreach that referenced `v0.1.0-alpha.3` is not a current-state description.

## Source of truth

- Repository: `AETHERXGLOBAL/execsurface`
- Current `main` HEAD after OpenSSF-pack merge: `a90c4c6e94129da5445aeed40863e097df045c28`
- Latest published GitHub Release: `v0.1.0-alpha.4`
- Immutable `v0.1.0-alpha.4` source commit: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Stable Action channel `v0.1`: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Public maturity: Public Alpha
- Public platform scope: Linux x86_64

The commits after the immutable alpha.4 source are documentation/evaluation/community-engagement changes only. They do not alter the released alpha.4 runtime semantics and do not silently move the stable Action channel.

At the initial OpenSSF baseline freeze, the release source/current main was 143 commits ahead of `v0.1.0-alpha.3`. Subsequent documentation-only engagement commits moved `main` further without changing the immutable alpha.4 source.

## Current product model

ExecSurface is a runtime-evidence and drift-evaluation system:

`Observe -> Normalize -> Accepted Baseline -> Diff -> Explicit Policy -> PASS / REVIEW / BLOCK / ERROR`

The baseline records an accepted canonical observed execution surface. It is not policy and is not proof that the accepted behavior is safe.

The primary public value proposition is bounded and specific:

> ExecSurface learns an accepted runtime execution surface for a command and later reports observed execution behavior that appeared, disappeared, or changed under the recorded observer and explicit policy.

It is primarily an **observer-backed evidence generator + baseline/diff/policy evaluation system**. It is not a general Linux mandatory-access-control system and it is not currently a kernel enforcement product.

## Current public architecture

### Public default/reference observer

Native Linux `ptrace` remains the public default/reference observer for the portable alpha.4 line.

The current reference path provides bounded evidence for selected descendant process/exec, pathname access-attempt metadata, successful-open/fd identity handling, covered fd-attributed read/write effects, covered rename/delete effects, network connect destinations, causal executable chains, and explicit observer incompleteness.

### Important alpha.4 hardening

The public portable path treats the known clone/shared-FD ambiguity conservatively:

- the known false-completeness class is no longer allowed to remain `complete=true` / PASS-eligible;
- ambiguity becomes explicit incomplete evidence;
- the implementation does **not** claim exact shared-FD attribution repair;
- this may create conservative false incompleteness for some clone/thread concurrency where exact fd-table sharing cannot be proved from raw observation v2.

### Evidence-authority boundary

A pathname copied from userspace at ptrace syscall entry is **pathname access-attempt metadata**, not kernel-object identity.

The PATH-TOCTOU criticism is preserved rather than explained away. Current public documentation does not treat ptrace userspace pointer observations as universally kernel-authoritative.

### Hybrid / BPF-LSM status

The architecture review triggered by Greg Kroah-Hartman's criticism concluded `HYBRID_ARCHITECTURE_RECOMMENDED` for stronger authority-sensitive propositions.

Subsequent research validated bounded kernel-hook/hybrid evidence concepts, but alpha.4 deliberately does **not** promote them to the public default.

Current public boundary:

- BPF-LSM / kernel-hook work: managed/research-only;
- automatic hybrid selection: not authorized;
- hybrid `learn` / `check` public authority: not authorized;
- ptrace <-> hybrid baseline interchangeability: not authorized;
- new privilege requirement for the portable alpha.4 path: not introduced;
- end-to-end hybrid production-readiness claim: not made.

## What ExecSurface can currently establish

Within its recorded observer, capability and completeness boundary, ExecSurface can establish that:

1. an accepted canonical observed execution surface was learned for a declared command;
2. a later observed execution produced no review/block finding, or produced specified added/removed/changed observed effects;
3. the selected policy mapped those findings to PASS / REVIEW / BLOCK;
4. operational, comparability or observer-health failure maps to ERROR/incomplete rather than silently becoming PASS;
5. machine-readable evidence can be retained for reproduction and inspection.

## What ExecSurface does not establish

ExecSurface does not establish that:

- a program is safe;
- drift is malicious;
- no unobserved behavior exists;
- no network access is possible because none was observed;
- ptrace sees every Linux effect or every kernel object authoritatively;
- mmap/io_uring and every possible I/O path are fully represented by the current fd model;
- all containers, namespaces, kernels or Linux security configurations support the public observer;
- the BPF-LSM research path is a production-ready public backend;
- self-evaluation is independent adoption or external validation.

## Supported public environment and installation

Current public support scope: Linux x86_64.

Documented public paths:

1. checksum-verified GitHub Release binary for `v0.1.0-alpha.4`;
2. Rust-native `cargo install execsurface --locked` after registry publication;
3. GitHub Action stable channel `AETHERXGLOBAL/execsurface@v0.1`;
4. immutable Action pin `AETHERXGLOBAL/execsurface@v0.1.0-alpha.4`.

`execsurface doctor` is diagnostic. It does not elevate privilege, modify ptrace policy or weaken host security configuration.

## Main changes since alpha.3

### Changed / strengthened

- release moved from alpha.3 to alpha.4;
- known clone/shared-FD false-completeness behavior hardened to fail closed;
- pathname evidence authority is explicitly bounded as userspace syscall-entry/access-attempt metadata rather than kernel-object identity;
- adversarial PATH-TOCTOU and shared-FD counterexamples are retained in release-review evidence;
- release review added explicit compatibility, adversarial, performance/resource, provenance and internal red-team gates;
- alpha.4 public Release binary and stable Action channel were promoted only after those gates;
- public documentation states the conservative false-incompleteness tradeoff;
- OpenSSF-facing evaluation material was refreshed from alpha.3 to exact alpha.4 public artifacts.

### Intentionally unchanged

- Linux x86_64 public support boundary;
- native ptrace as public default/reference observer;
- baseline-v2 serialization/digest semantics;
- PASS / REVIEW / BLOCK / ERROR meaning and exit-code contract;
- metadata-oriented privacy boundary;
- no automatic BPF-LSM/hybrid selection;
- no ptrace/hybrid baseline equivalence.

## Greg Kroah-Hartman criticism: current disposition

The criticism remains **technically valid and incorporated**, not dismissed.

Repository architecture review: `docs/architecture/PTRACE_VS_LSM_ARCHITECTURE_REVIEW.md`.

The review established that SELinux/AppArmor are not drop-in replacements for ExecSurface's per-command evidence-generation contract, while also establishing that ptrace is not the strongest evidence authority for all object-identity propositions. The resulting architectural direction is additive/hybrid research, with public promotion deferred until proposition semantics, loss, privilege, portability and compatibility are proved.

## Documentation drift discovered and resolved

Initial OpenSSF baseline inspection found:

1. `docs/TECHNICAL_EVALUATION.md` still pinned alpha.3;
2. `docs/INDEPENDENT_EVALUATION.md` still named alpha.3 current;
3. historical M9.3 roadmap wording could be mistaken for current release status;
4. the architecture review correctly referenced alpha.3 as its historical freeze point.

Resolution:

- both evaluation docs now use exact alpha.4 public artifacts and current limitation/authority wording;
- `docs/STATUS.md` now states the live current state and explicitly treats older roadmap entries as historical milestone records;
- README now calls alpha.4 the current published release;
- historical architecture/milestone records are retained rather than rewritten.

## Zero-assistance rehearsal

A clean GitHub-hosted Ubuntu 24.04 evaluation using only the published alpha.4 binary/checksum completed successfully:

- exact alpha.4 download/checksum: PASS;
- `execsurface --version`: alpha.4;
- `doctor`: PASS;
- baseline learn: PASS;
- unchanged check: PASS / exit `0`;
- controlled drift: REVIEW / exit `10`;
- evidence artifact retained.

Canonical rehearsal run: `36478741625`.
Final pre-merge repeat: `36479236174`.

This is internal readiness evidence only, not independent external validation.

## Historical OpenSSF outreach content now obsolete

The following must not be reused as current facts:

- `ExecSurface v0.1.0-alpha.3` as latest release;
- any wording implying the pre-M10 ptrace authority model is unchanged;
- any evaluation command that pins alpha.3;
- any statement that omits the shared-FD fail-closed hardening / conservative false-incompleteness tradeoff;
- any statement that ignores the ptrace-vs-LSM architecture review and kernel-hook research direction;
- any claim that outreach or internal rehearsal constitutes external validation.

## OpenSSF path and current engagement state

`PRIMARY_OPENSSF_PATH = ORBIT Working Group`

`SECONDARY_OPENSSF_PATH = Supply Chain Integrity Working Group` only for a concrete provenance/attestation question.

The public pack has been merged to `main`, the original OpenSSF/Linux Foundation email thread has been answered with alpha.4 material, and direct ORBIT GitHub issue creation was attempted.

That external GitHub write returned `403 Resource not accessible by integration`; this is recorded as a connector permission limitation, **not** an ORBIT rejection.

## Engagement readiness state

`READY_FOR_EXTERNAL_TECHNICAL_REVIEW — PUBLIC_PACK_MERGED / ZERO_ASSISTANCE_INTERNAL_REHEARSAL_PASS`

Remaining external step: obtain real community/evaluator interaction through ORBIT/OpenSSF channels. Any external failure or criticism must be preserved before assistance is offered.
