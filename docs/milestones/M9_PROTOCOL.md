# M9 — Independent Adoption & Real-Workload Evidence Protocol

Date: 2026-09-26
Status: **M9.0 OPEN — PROTOCOL FROZEN BEFORE ACCEPTED EVIDENCE**
Tracking: #51
Base: `8f1ec63219e084f10678655ee698fd2a482312d6`

## Objective

M9 evaluates the existing public ExecSurface product on real external developer/CI workloads. It does not add observer features and does not presume that eBPF should be reopened.

The primary questions are:

1. Can independent or zero-contact external users/projects install and use the current product successfully?
2. Do real baseline -> rerun/check -> drift workflows preserve current semantics outside AETHER X-controlled fixtures?
3. What false-positive, false-negative, operational-failure, and usability-friction evidence appears?
4. What is the real ptrace cost on external workloads?
5. Does any accepted workload satisfy a predeclared M6.5 eBPF reopen trigger?
6. Does any external workflow require a capability unavailable under the current ptrace product?

## Fixed governance roles

### Innovation Scientist / Adoption Architect

Find the smallest, highest-value external evidence paths while preserving independence and reproducibility.

### Deviation Prevention / Scientific Integrity

Reject cherry-picking, hidden failures, post-hoc thresholds, benchmark gaming, relabeling self-run evidence as adoption, or semantic changes made only to improve M9 results.

### Independent Red Team

Attack attribution, independence classification, reproducibility, privacy, completeness, false-positive/false-negative claims, performance methodology, and any proposed eBPF reopen.

## Authority invariants

M9 cannot change these without a separate evidence gate:

- `ptrace` correctness reference: retained;
- default public backend: `ptrace`;
- eBPF public exposure: deferred;
- eBPF full-surface comparability: false;
- eBPF `learn` / `check`: not authorized;
- eBPF PASS: not authorized;
- backend auto-selection: not authorized;
- ptrace/eBPF baseline interchangeability: not authorized;
- production persistent eBPF service: not authorized;
- default public install path: unchanged.

## Independence classes

Every accepted M9 record MUST carry exactly one class.

### `INDEPENDENT_USER`

A third party initiates and executes the workflow without AETHER X controlling the workload or desired outcome. This may count toward independent adoption when attribution and evidence are auditable.

### `ZERO_CONTACT_EXTERNAL_REPRO`

AETHER X runs the unchanged public product against a pinned, unmodified external project/workload without maintainer coordination. This is external compatibility/performance evidence, **not adoption**.

### `COLLABORATIVE_EXTERNAL`

A third party supplies or owns the workload but AETHER X materially assists execution. Useful external evidence, but not independent adoption.

### `INTERNAL_FIXTURE`

AETHER X controls the workload/fixture. It may validate tooling but cannot satisfy M9 external-adoption gates.

## Evidence statuses

Use only:

- `PROVED`
- `COMPUTATIONAL_EVIDENCE`
- `PARTIAL`
- `OPEN`
- `KILLED`

No claim without an attributable evidence record.

## Required record fields

Machine-readable records MUST conform to `docs/milestones/M9_EVIDENCE_SCHEMA.json` and include at minimum:

- evidence ID;
- timestamp;
- independence class;
- claim status;
- public/consented external reference when available;
- pinned project revision;
- exact command/workflow;
- ExecSurface version and source SHA;
- installation path;
- OS/kernel/architecture/CPU facts;
- direct outcome;
- ExecSurface outcome;
- verdict, comparability, completeness and failure state;
- privacy confirmation;
- raw artifact references and SHA-256 digests;
- failures/exclusions with reasons;
- reproduction instructions.

Performance records additionally require raw timing samples and frozen measurement parameters.

## Privacy boundary

M9 MUST NOT collect or require:

- file contents;
- environment-variable values;
- stdin contents;
- network payloads;
- secrets/tokens/credentials;
- unrestricted argv capture.

If an external contribution contains prohibited sensitive content, the record is not accepted as-is. Preserve only the minimum privacy-safe metadata needed for the claim.

## Compatibility/adoption workflow acceptance

For a normal external workflow, record separately:

1. installation result;
2. `execsurface doctor` result where applicable;
3. baseline creation result;
4. unchanged rerun/check result;
5. drift case result if a natural or controlled drift is available;
6. comparability/completeness state;
7. operational errors/timeouts/truncation;
8. user friction or intervention required.

A successful command alone is not adoption evidence.

## Failure preservation

Failures are first-class evidence.

Never delete or silently exclude:

- installation failures;
- unsupported-host results;
- permission failures;
- observer errors;
- timeouts;
- truncation;
- non-comparable evidence;
- false positives;
- suspected false negatives;
- user confusion or manual intervention.

Any exclusion must remain machine-readable with a reason and cannot disappear from the evidence ledger.

## Performance protocol

### Eligibility

Performance may be measured on any external workload, but only workloads with direct median >= `100 ms` can satisfy the relative M6.5 eBPF trigger.

### Frozen sample plan

For every accepted performance case:

- pin project revision and command before measurement;
- use the same host and host state for direct and ptrace-observed samples;
- record 3 warm-up executions for each mode, excluded from statistics but retained in raw evidence;
- record 15 measured direct samples and 15 measured ptrace samples;
- alternate modes in paired order (`direct`, `ptrace`, then `ptrace`, `direct`) repeatedly to reduce monotonic host drift;
- do not discard measured samples except for a predeclared infrastructure failure; any exclusion remains recorded with reason;
- report median direct runtime, median observed runtime, median absolute overhead, and median ratio;
- retain all raw timings in milliseconds;
- record host/kernel/runner facts and ExecSurface SHA/version.

### Trigger calculation

Let:

- `D = median(direct_ms)`
- `P = median(ptrace_ms)`
- absolute overhead = `P - D`
- slowdown ratio = `P / D`

The predeclared performance reopen trigger is satisfied only if either:

1. `D >= 100 ms` and `P / D > 2.0`; or
2. `P - D > 500 ms`.

Crossing this trigger **does not authorize eBPF**. It only permits a separate same-workload eBPF value experiment under the existing safety and authority gates.

### Timeout rule

Timeouts must be fixed in the evidence record before the measured series starts. A timeout failure is retained as evidence and cannot be converted into a clean sample by silently extending the timeout after observation.

### Claims boundary

Single-host results are exact-host evidence only. No universal Linux speed/performance claim is allowed from M9 measurements unless a later gate establishes it.

## Capability-based eBPF reopen condition

A capability-based reopen requires all of:

1. a concrete external user/workflow need;
2. evidence that the current ptrace product cannot provide it reliably or acceptably;
3. evidence that an eBPF route can provide the required capability under explicit completeness and privacy semantics;
4. a separate privilege/security/product gate;
5. independent Red Team review.

A speculative feature idea is insufficient.

## M9.0 acceptance criteria

M9.0 closes only when:

- the protocol is committed before accepted M9 evidence;
- the machine-readable schema is committed;
- independence classes are frozen;
- performance sample plan and trigger rules are frozen;
- privacy and failure-preservation rules are frozen;
- product authority invariants are restated;
- CI remains green;
- Red Team finds no path for self-run compatibility evidence to be mislabeled as independent adoption.

## Next gate after M9.0

**M9.1 — Zero-contact external workload expansion.**

Select several public, pinned, unmodified external projects/workloads using criteria frozen before result collection. Run the existing public product without maintainer coordination. Preserve successes and failures. Classify all such results as `ZERO_CONTACT_EXTERNAL_REPRO`, never `INDEPENDENT_USER`.
