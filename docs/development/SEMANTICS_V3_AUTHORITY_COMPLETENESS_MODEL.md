# ExecSurface — Semantics v3 Authority & Completeness Model

Date: 2026-09-29
Tracking: #101
Parent: #100
Predecessors:
- `SEMANTICS_V3_CURRENT_CONTRACT_AUDIT.md`
- `SEMANTICS_V3_PROPOSITION_TAXONOMY.md`

Status: **S2/S3 DESIGN FROZEN FOR PROTOTYPE / RED TEAM**

## 0. Executive design decision

ExecSurface v3 will **not** use a single numeric confidence/authority score.

Authority is modeled as a set of explicit semantic guarantees. A proposition declares the guarantees it requires; an evidence record declares the guarantees it actually supports. Satisfaction is set/constraint based and therefore only partially ordered.

This avoids false statements such as "LSM > ptrace" in the abstract. A kernel hook may be stronger for object identity while a ptrace lifecycle event may carry a different useful guarantee for per-command lineage. Backends are not globally ranked.

Completeness is likewise decomposed into explicit dimensions with dependency propagation. A conservative session summary remains available, but one bounded ambiguity must not automatically erase unrelated propositions unless a dependency proves that it should.

## 1. Core semantic objects

### 1.1 Proposition

A bounded semantic statement from the S1 taxonomy, e.g.:

- `file.pathname_attempt_observed`
- `file.fd_read_effect_observed`
- `process.exec_succeeded`
- `network.connect_destination_attempt_observed`

A proposition contains canonical subject/object/value and does not itself claim evidence quality.

### 1.2 Evidence guarantee

A typed fact about **how** a proposition was established.

Initial guarantee dimensions follow.

#### Observation point

Candidate values:

- `userspace_argument_pre_kernel`
- `ptrace_lifecycle_event`
- `syscall_result_post_operation`
- `derived_runtime_fd_state`
- `kernel_security_hook`
- `kernel_tracepoint`
- `imported_attested_trace`

These are descriptive, not inherently ordered.

#### Identity basis

Candidate values:

- `none`
- `lexical_argument`
- `trace_time_dirfd_resolved_argument`
- `runtime_fd_path_correlated`
- `kernel_object_grounded`
- `socket_address_argument`

`runtime_fd_path_correlated` is not automatically equivalent to `kernel_object_grounded`.

#### Temporal binding

Candidate values:

- `pre_operation_intent`
- `successful_operation_result`
- `post_operation_derived_state`
- `kernel_decision_point`
- `lifecycle_transition`

#### Causal binding

Candidate values:

- `direct_event`
- `state_machine_correlated`
- `lineage_derived`

A proposition may carry multiple guarantees across dimensions.

### 1.3 Proof requirement

A proposition kind declares minimum guarantees and completeness dependencies.

Example — current ptrace pathname attempt:

```text
proposition: file.pathname_attempt_observed
requires:
  observation_point = userspace_argument_pre_kernel
  identity_basis in {lexical_argument, trace_time_dirfd_resolved_argument}
  temporal_binding = pre_operation_intent
  session_scope = complete
  lifecycle = complete enough to bind actor
```

This requirement does not contain `kernel_object_grounded`, so the record cannot be misrepresented as an object identity proof.

### 1.4 Evidence record

Conceptually:

```text
ProofCarryingObservation {
    proposition,
    guarantees,
    completeness_dependencies,
    ambiguity_annotations,
    backend_source,
    canonical_proof_profile,
    optional_noncanonical_trace_links
}
```

The canonical proof profile must be deterministic. Raw event sequence numbers, unstable TIDs, timestamps and ephemeral transport IDs must not leak into a baseline digest unless normalized by an explicitly versioned rule.

## 2. Authority is a partial order over guarantees

Define evidence record A to satisfy requirement R only if A proves every guarantee constraint in R and no invalidating ambiguity/completeness state applies.

For comparable guarantee sets, one evidence record may dominate another by strict guarantee inclusion.

But not every pair is comparable.

Example:

- Evidence A: ptrace successful exec lifecycle event + strong per-command lineage.
- Evidence B: kernel object hook around file permission + strong object grounding.

Neither is globally "higher authority" because they prove different propositions/guarantees.

### Design rule A1

**No global backend authority score.**

### Design rule A2

Named authority profiles may exist only as deterministic shorthands for guarantee sets, never as unexplained ordinal levels.

### Design rule A3

A report may state "stronger for proposition X because it satisfies guarantees G1/G2 that the other record lacks"; it may not state "backend X is more trustworthy" without proposition scope.

## 3. Completeness dimensions

Initial v3 completeness state is decomposed into the following dimensions.

### `session_scope`

Did the observer establish the declared root/session before accepting evidence?

### `lifecycle`

Are target/descendant creation, exec and terminal transitions complete enough for the proposition's actor/lineage dependencies?

### `transport`

Could producer/ring-buffer/IPC/decode loss have omitted relevant records?

### `resource_budget`

Did an event/resource limit truncate required evidence?

### `capability`

Does the active backend/environment implement the proposition family at all?

### `object_identity`

Is object identity established under the proposition's declared identity basis, or is there a bounded ambiguity such as fd reuse/shared-table uncertainty?

### `fd_table_relation`

Is the fd-table relationship required by the proposition known (`shared`, `independent_copy`, continuity) or unknown?

### `causal_lineage`

Is the actor/execution-chain dependency complete for this proposition?

## 4. Completeness state vocabulary

Each required dimension uses a typed state rather than a free-form Boolean:

```text
not_required
complete
incomplete(reason_code)
ambiguous(reason_code)
unsupported(reason_code)
```

`unsupported` means the backend/environment does not claim the required capability.

`incomplete` means the capability exists but required evidence is known missing/broken.

`ambiguous` means evidence exists but does not justify selecting among materially different semantic interpretations.

All three are non-satisfying for a requirement that needs `complete`.

## 5. Conservative session summary

For compatibility with simple CLI decisions, v3 may derive a conservative session summary:

- `complete_for_declared_surface`
- `incomplete_for_declared_surface`

This summary is **derived**, not the source of truth.

A session can contain:

- some propositions admissible and complete;
- some unsupported;
- some ambiguous;
- some incomplete.

A PASS decision may only use proposition families for which the baseline/policy contract has all required complete proofs. Missing required proof remains non-PASS-eligible.

## 6. Dependency propagation

Completeness is not local by default. It propagates along explicit dependency edges.

Example:

```text
observer.session_scope
    -> process.child_created
        -> process.fd_table_relation
            -> file.open_fd_associated
                -> file.fd_read_effect_observed
```

If `process.fd_table_relation` is ambiguous for tasks A/B, fd-derived propositions depending on that relation become ambiguous/incomplete.

But a separately observed `network.connect_destination_attempt_observed` from an unaffected actor need not automatically become incomplete **if** its own session/lifecycle/transport dependencies remain satisfied.

### Propagation rule C1

No failure may be scoped locally unless all downstream dependencies are known.

### Propagation rule C2

Unknown dependency graph = fail broader, not narrower.

## 7. Shared-FD case under v3

Alpha.4 behavior:

```text
any clone-based concurrency where exact CLONE_FILES relation cannot be certified
    -> global observation.complete = false
```

This is correct under raw v2 because the retained evidence is insufficient.

Candidate v3 behavior, only if S5 proves exactness:

```text
clone/clone3 captured with flags
    -> fd_table_relation(shared | independent_copy | unknown)

unknown
    -> affected fd-derived propositions non-admissible

independent_copy
    -> no shared-table ambiguity merely from clone concurrency

shared
    -> maintain shared table identity and test close/dup/reuse races adversarially
```

This does **not** mean all other evidence becomes complete automatically. Lifecycle/transport/resource prerequisites still apply.

## 8. Two-layer provenance design

A major determinism problem is that forensic trace links can contain unstable IDs while baselines must remain deterministic.

Therefore v3 separates:

### Canonical proof profile — baseline eligible

Contains only deterministic semantic facts:

- proposition kind;
- canonical subject/object;
- guarantee set;
- required completeness classes;
- stable ambiguity class codes;
- backend semantic profile/version, not transient run identity.

### Trace evidence links — report/evaluation only by default

May contain:

- raw event sequence references;
- run-local TIDs;
- artifact/evidence digests;
- attestation references;
- collector event IDs.

These links support auditability without destabilizing accepted surface digests.

A later attestation layer can cryptographically bind the canonical proof record to external evidence artifacts without making ephemeral identifiers part of semantic equality.

## 9. Baseline authority contract

A v3 accepted baseline should represent both:

1. **what behavior was accepted**;
2. **what minimum proof contract was accepted for that behavior**.

This prevents an accepted kernel-object-grounded proposition from later being silently satisfied by a weaker pathname-intent observation.

Conversely, a stronger future backend may satisfy a weaker requirement if a formally defined guarantee-entailment rule proves it.

No cross-backend comparison is allowed merely because canonical behavior strings match.

## 10. Diff semantics implication

v3 diff must distinguish at least:

- behavior added/removed/changed;
- proof authority changed;
- proof completeness changed;
- proposition became unsupported;
- proposition became ambiguous;
- same behavior with stronger proof;
- same behavior with weaker/inadmissible proof.

A proof downgrade cannot silently appear as "no drift".

## 11. Policy implication

Policy remains separate from baseline.

Potential future policy inputs:

- behavioral drift;
- authority downgrade;
- completeness downgrade;
- unsupported required proposition;
- explicit ambiguity class.

No P2 design authorizes new public verdict semantics yet.

## 12. Red-team obligations before prototype acceptance

The S4 prototype must survive fixtures for:

1. weak pathname observation mislabeled as kernel object identity;
2. successful target exit with incomplete observer evidence;
3. clone concurrency with no `CLONE_FILES`;
4. clone concurrency with `CLONE_FILES`;
5. fd close/dup/reuse race;
6. lifecycle loss that invalidates lineage transitively;
7. transport loss affecting unknown event families;
8. two backends producing same canonical value with different guarantees;
9. proof downgrade hidden behind stable behavior;
10. unstable raw event IDs attempting to alter baseline digest.

## 13. S2/S3 decision

**`S2_S3_AUTHORITY_COMPLETENESS_MODEL_ACCEPTED_FOR_PROTOTYPE`**

The model provides a materially more precise semantics than v2 while retaining fail-closed behavior:

- authority is explicit and proposition-scoped;
- completeness is typed and dependency-aware;
- backends are not globally ranked;
- baseline proof requirements cannot be silently downgraded;
- deterministic semantics are separated from forensic trace links.

Next authorized step: S4 isolated proof-carrying record prototype with deterministic serialization tests. No integration into public learn/check or alpha.4 is authorized.