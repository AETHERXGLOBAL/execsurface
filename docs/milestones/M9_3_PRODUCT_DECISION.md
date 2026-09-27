# M9.3 — Evidence Synthesis / Product Decision

Date: 2026-09-27
Status: **DECISION FROZEN — PREPARE `0.1.0-alpha.3` CORRECTNESS/HARDENING RELEASE**
Base main: `dd2235e3bac17f47f8101412c15edd408e104f43`

## Decision

Prepare ExecSurface `0.1.0-alpha.3` as a bounded **correctness / lifecycle-hardening / evaluation-readiness** public-alpha release.

This is not an adoption milestone and not an eBPF productization milestone.

## Why a new public version is required

The currently published public version is `0.1.0-alpha.2`, sourced from commit:

`c6cabf1b4d1399898b9643a7fce011664aac0918`

Since that release, the repository accepted a material ptrace correctness hardening that is not present in the published package. Public alpha.2 failed closed on the pinned `casey/just` workload with ptrace `ESRCH`. The bounded lifecycle-specific fix subsequently proved:

- 240/240 bounded diagnostic reproductions complete;
- 37 exact bounded `PTRACE_GET_SYSCALL_INFO` ESRCH recoveries observed in the diagnostic gate;
- 240/240 production-clean bounded stress executions complete;
- unchanged observer regression suite PASS;
- pinned `casey/just` 5/5 complete with exit 0;
- no blanket ESRCH suppression, sleeps, arbitrary retries, eBPF substitution, event-budget weakening or completeness relaxation.

A public install that still resolves to alpha.2 therefore does not include the accepted lifecycle hardening. Shipping a new prerelease is the correct way to make the proved fix available without rewriting the existing immutable release history.

## M9 synthesis

### What M9.1 established

Within the tested exact-host scope:

- post-hardening ptrace completed baseline -> unchanged-check workflows on pinned `ripgrep` and `fzf` runtime workloads with repeated PASS / 0 findings;
- controlled runtime expansion remained visible as REVIEW in the accepted user-like zero-contact cases;
- full build/test workloads can produce substantial legitimate temporary-path/runtime variation and must not be normalized away merely to force PASS;
- multiple workloads crossed the predeclared ptrace-cost trigger, permitting a bounded eBPF value experiment.

### What the eBPF value screen established

The triggered same-workload experiment did **not** establish an eBPF value claim. The current per-invocation libbpf path failed its warmup completeness/health gate (`incomplete_lifecycle`, and one `incomplete_capability` warmup) before accepted measured samples could begin.

Result retained as:

`KILLED FOR VALUE CLAIM / PARTIAL_NO_VALUE_CLAIM`.

### What M9.2 established

The preregistered public GitHub discovery scope found:

- accepted independent-adoption records: 0;
- countable positive-close records: 0;
- accepted independent drift records: 0.

Result retained as:

`KILLED — NO ACCEPTED INDEPENDENT EVIDENCE IN DECLARED SEARCH SCOPE (2026-09-27)`.

This is scoped negative evidence, not a claim that no users exist elsewhere.

## Alpha.3 release contents

Alpha.3 may include only already-accepted mainline product changes and documentation/evidence improvements, principally:

1. ptrace thread-group/lifecycle correctness hardening accepted through M9;
2. regression coverage for the bounded lifecycle race;
3. current public self-service / technical-evaluation documentation and intake;
4. distribution-workflow hardening already merged to main;
5. evidence/roadmap updates that do not alter runtime semantics.

The isolated `experiments/m8-ebpf/**` research assets may remain in source but are not promoted into public product authority.

## Authority invariants for alpha.3

Alpha.3 MUST retain:

- platform: Linux x86_64 public alpha;
- public/default backend: native ptrace;
- evidence schema v2 semantics;
- baseline/policy separation;
- PASS / REVIEW / BLOCK / ERROR meanings and exit codes;
- metadata-only privacy boundary;
- fail-closed incompleteness/event-budget semantics;
- eBPF `learn` / `check`: not authorized;
- eBPF PASS: not authorized;
- backend auto-selection: not authorized;
- ptrace/eBPF baseline interchange: not authorized;
- production persistent eBPF service: not authorized.

## Claims allowed for alpha.3

Allowed, with tested-scope qualification:

- alpha.3 contains the accepted ptrace lifecycle hardening that is not in alpha.2;
- the hardening was exercised by bounded regression/stress evidence and the pinned `casey/just` reproducer;
- zero-contact external runtime trials on pinned `ripgrep` and `fzf` completed repeated unchanged checks with PASS / 0 findings;
- the project now exposes a public independent-evaluation/intake path.

## Claims forbidden for alpha.3

Do not claim:

- independent adoption or third-party validation;
- universal Linux compatibility;
- universal performance improvement;
- that eBPF is faster, production-ready or equivalent to ptrace;
- that ExecSurface proves software safety, maliciousness, vulnerability status or all possible behavior;
- that build/test workload variation has been solved generally.

## Release acceptance gate

Do not publish/tag/promote alpha.3 until all of the following are true on the exact release candidate:

1. workspace package version and all exact internal dependency versions are `0.1.0-alpha.3`;
2. Cargo.lock is internally consistent with the candidate;
3. release request, Action release tag, README and user-facing version examples all agree on `v0.1.0-alpha.3`;
4. alpha.3 release notes state the correctness fix and the adoption/eBPF non-claims;
5. format passes;
6. strict Clippy passes;
7. complete workspace tests pass;
8. ptrace lifecycle regression gate passes;
9. distribution probe passes;
10. Action smoke passes, including privacy evidence;
11. registry packaging/readiness gate passes;
12. immutable release tag is created only by the existing gated promotion path after merge;
13. crates.io publication uses the existing resumable/tag-driven publication workflow and proves fresh install from the published registry version;
14. stable `v0.1` Action promotion occurs only through the existing gated release mechanism.

A failed gate is preserved as evidence; do not weaken the gate to obtain the release.

## Post-release public update rule

Only after the immutable alpha.3 release and registry install proof succeed may public-facing surfaces be updated to state that alpha.3 is the current public alpha.

Public updates must retain the M9.2 boundary: compatibility and internal evidence may be described accurately, but independent adoption must not be implied.

## Remaining product gap after alpha.3

Independent adoption remains open as a future external-evidence objective rather than a blocker for publishing an already-proved correctness fix.

A future build-ephemeral abstraction is also OPEN; M9 evidence does not authorize blanket temp-path suppression.
