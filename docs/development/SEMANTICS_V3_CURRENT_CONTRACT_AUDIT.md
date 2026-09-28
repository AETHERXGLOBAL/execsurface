# ExecSurface — Semantics v3 Current-Contract Audit

Date: 2026-09-29
Tracking: #101
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **S0 CLOSED — CURRENT CONTRACT FROZEN FOR P2 DESIGN**

## Purpose

Freeze what ExecSurface v2 actually means before designing v3.

This audit does not change alpha.4 behavior, schemas, baseline digests, policy, observer authority, or public backend selection. Its purpose is to prevent v3 from laundering stronger claims into existing event names.

## Sources inspected

- `crates/execsurface-model/src/lib.rs` — raw observation schema v2
- `crates/execsurface-model/src/canonical.rs` — canonical surface schema v2
- `crates/execsurface-normalize/src/lib.rs` — raw→canonical mapping
- `crates/execsurface-baseline/src/lib.rs` — baseline lock/digest v2
- `docs/architecture/PTRACE_VS_LSM_ARCHITECTURE_REVIEW.md`
- `docs/milestones/M12_1_SEMANTIC_INTEGRATION_RESULT.md`
- `docs/milestones/POST_M9_EBPF_E4_SAME_WORKLOAD_VALUE_RESULT.md`

## 1. Raw v2 contract

`Observation` contains:

- one `BackendMetadata` object for the session;
- one global `complete: bool`;
- one command outcome;
- a vector of `RawEvent`;
- a vector of free-form `ObserverWarning` records.

`BackendMetadata` exposes capability/limitation **strings**, not typed proof obligations.

Each `RawEvent` has sequence, TID and an event kind, but does not carry:

- proposition identity;
- per-event authority;
- per-event capability proof;
- per-event completeness prerequisites;
- explicit ambiguity class;
- source-evidence digest/reference.

### S0 finding R1 — completeness is session-global

If `Observation.complete == false`, current normalization rejects the whole observation with `IncompleteObservation`.

This is safe and fail-closed, but coarse. A localized ambiguity can invalidate every canonical effect even where unrelated propositions remain well-supported.

**Important:** P2 is not authorized to simply make completeness local. Some failures (lost lifecycle ancestry, unknown transport loss, event truncation) can invalidate many/all propositions transitively and must remain global or propagate broadly.

### S0 finding R2 — clone evidence loses the fact P2 now needs

`ProcessSpawn` stores only:

- child TID;
- mechanism (`fork`, `vfork`, `clone`).

Raw v2 does not retain clone flags such as `CLONE_FILES`.

Therefore clone concurrency cannot be separated from actual shared-fd-table semantics after the fact. Alpha.4 correctly responds conservatively with `shared_fd_table_ambiguity`.

E4 demonstrated that this conservative ambiguity is not theoretical: both pinned real concurrent workloads became incomplete at ptrace warmup #1.

### S0 finding R3 — event name is not authority

Raw event classes do not encode whether a value was:

- read from a userspace argument before kernel consumption;
- correlated with a successful syscall result;
- inferred from later fd state;
- obtained from a kernel-object hook.

Backend knowledge is currently implicit in implementation and documentation rather than carried with each semantic proposition.

## 2. Canonical v2 contract

`CanonicalSurface` contains normalization metadata and a set of `CanonicalEffect` values.

Canonical effects intentionally remove PID/TID/timestamp/raw sequence variance.

This is valuable for deterministic baselines, but the canonical effect itself does not carry evidence provenance or proof obligations.

### S0 finding C1 — different evidence strengths converge into one effect family

`FilePathAccess` is the canonical family for both pathname-oriented access metadata and fd-attributed read/write effects.

The normalizer partially preserves a distinction in `CanonicalPath.resolution`:

- lexical/relative/path-traversal states for pathname observations;
- `KernelFdResolved` for fd-attributed paths.

This is useful but insufficient as a general authority model. Path resolution mode is not equivalent to proof authority and does not describe all required ambiguity/loss prerequisites.

### S0 finding C2 — pathname canonicalization can look stronger than source semantics

A canonical absolute path is a stable normalized value. It must not be interpreted as proof that the kernel consumed that exact pathname or operated on the corresponding object.

For ptrace entry-time pointer reads, PATH-TOCTOU remains a real counterexample. Therefore a canonical path value needs an explicit semantic proposition such as **pathname argument/access attempt observed**, not an implied kernel-object statement.

### S0 finding C3 — causal lineage has no explicit confidence object

Canonical effects may contain actor and execution-chain context, but there is no typed statement of whether lineage is complete, inferred, bounded, or invalidated by lifecycle loss.

A future proof record must treat causal context as a dependency, not merely decoration.

## 3. Baseline v2 contract

Baseline v2 stores:

- tool identity;
- command identity;
- platform identity;
- one global observer identity with capability/limitation strings;
- the canonical surface;
- deterministic digest contract.

### S0 finding B1 — baseline observer metadata is session/global, not proposition-scoped

The lockfile says which observer produced the accepted surface, but cannot say:

> effect X is supported by capability A with authority class Y, while effect Z depends on a weaker or different proof path.

This prevents safe mixed-authority or mixed-backend composition without additional semantics.

### S0 finding B2 — v2 digest compatibility is a hard boundary

Current baseline digest serialization is frozen and tested. Adding proof metadata in-place would mutate the digest contract and silently redefine accepted baselines.

Therefore v3 must use explicit schema/digest versioning. Existing v2 lockfiles remain v2 evidence and must continue to verify under v2 rules.

## 4. Current fail-closed hardening

M12.1 deliberately made clone-based shared-fd uncertainty asymmetric:

- ambiguity sets observation incomplete;
- warning `shared_fd_table_ambiguity` is emitted;
- incomplete ambiguity is not PASS-eligible;
- the change does not claim exact fd-table repair.

### S0 finding H1 — the guard is correct under raw v2

Given raw v2 lacks enough clone detail, relaxing the guard would recreate a known false-completeness class.

P2 must improve the evidence available to classification, not simply narrow the guard by assumption.

## 5. Session-global vs proposition-scoped failure candidates

This is a design input, not final v3 semantics.

### Likely session-global / transitively broad

- unknown producer/ring-buffer loss affecting event membership;
- event-budget truncation where omitted event families are unknown;
- root/session registration failure;
- unbounded lifecycle/descendant membership loss;
- decode/integrity failure where affected records cannot be bounded;
- execution-chain corruption when actor/descendant scope becomes unknowable.

### Potentially proposition-scoped if bounded and proved

- pathname object-identity ambiguity for one pathname proposition;
- unsupported file-object authority while process lifecycle remains sound;
- exact shared-fd attribution uncertainty affecting fd-derived read/write propositions while unrelated network/process propositions remain independently supported;
- missing capability for an event family that cannot influence other families' prerequisites.

No item moves to proposition-scoped status without an explicit dependency proof and adversarial test.

## 6. Authority dimensions that must not be collapsed into one score

P2 must distinguish at least these concepts:

1. **Observation point** — userspace argument, syscall result, fd state, kernel object hook, external imported trace.
2. **Identity grounding** — lexical/path argument, fd-correlated identity, kernel object identity.
3. **Temporal relation** — pre-operation intent, successful-operation result, later derived state.
4. **Causal scope** — target/descendant membership and actor chain confidence.
5. **Completeness** — whether the required event family and dependencies are complete.
6. **Transport integrity** — whether records could have been lost/dropped/truncated.

A single numeric 'confidence' or 'authority score' would hide incomparable failure modes and is rejected for the initial design.

## 7. Proposition families extracted from v2

The following semantic propositions are implicit today and must become explicit candidates in S1:

- descendant process/thread creation observed;
- successful executable transition observed;
- pathname access argument/attempt observed;
- open/openat/openat2 intent observed;
- successful open correlated to an fd/object path surrogate;
- fd-attributed read observed;
- fd-attributed write observed;
- rename/delete attempt/occurrence under current observer semantics;
- network connect destination attempt observed;
- causal executable actor/chain association;
- observer/session health and incompleteness facts.

The exact wording must be tightened in S1 so names do not imply more authority than evidence proves.

## 8. P2 invariants frozen by S0

1. **No authority laundering:** canonicalization cannot turn a weak source observation into a stronger proposition.
2. **No silent schema reinterpretation:** v2 remains v2 forever.
3. **No completeness weakening:** unknown evidence remains non-PASS-eligible where current prerequisites require it.
4. **No backend prestige rule:** an event is not stronger merely because its backend is 'eBPF', 'LSM' or 'kernel'. Authority is proposition + observation-point specific.
5. **No whole-backend equivalence:** equivalence is proposition-by-proposition with explicit prerequisites.
6. **Causal dependencies are evidence dependencies:** actor/lineage must have their own health semantics.
7. **Privacy remains metadata-bounded:** stronger proof does not authorize payload/content/secret collection.
8. **Determinism remains mandatory:** proof metadata that enters accepted baselines must have deterministic canonical serialization.

## 9. S0 decision

**`S0_CURRENT_CONTRACT_AUDIT_ACCEPTED`**

There is a real semantic gap worth addressing:

> v2 is conservative and safe, but it couples authority/completeness too coarsely to the session/backend and too weakly to individual canonical propositions.

This creates both false-confidence risk if semantics are overread and false-incompleteness cost when one bounded ambiguity invalidates unrelated effects.

The gap is sufficiently concrete to proceed to S1 Proposition Taxonomy. No runtime or public-schema change is authorized by S0.