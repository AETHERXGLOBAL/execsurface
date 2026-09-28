# ExecSurface — Semantics v3 Proposition Taxonomy

Date: 2026-09-29
Tracking: #101
Parent program: #100
Predecessor: `SEMANTICS_V3_CURRENT_CONTRACT_AUDIT.md`
Status: **S1 INITIAL TAXONOMY FROZEN FOR FALSIFICATION**

## 0. Rule

An ExecSurface proposition is a bounded statement that can be supported or rejected by explicit evidence prerequisites.

An event name is not a proposition, and a canonical path is not automatically an object-identity claim.

The taxonomy uses deliberately narrow language so later backends can support only the propositions they actually prove.

## 1. Proposition record shape — conceptual only

Every v3 proposition candidate must eventually define:

- `proposition_kind`;
- subject/actor scope;
- predicate;
- object/value;
- temporal meaning;
- identity basis;
- required capabilities;
- required causal dependencies;
- invalidating ambiguity/loss classes;
- privacy-safe canonical representation;
- equality/deduplication rule.

This document does not yet freeze Rust types or wire format.

## 2. Process/lifecycle propositions

### `process.child_created`

**Statement:** a child task/process was created from a tracked parent under the observer's declared descendant-scope semantics.

Does not by itself claim:
- separate process vs same-thread-group semantics unless flags/context prove it;
- fd-table independence;
- successful exec by the child.

Required evidence includes tracked parent membership and a creation event tied to the child.

Potential invalidators:
- lost lifecycle event;
- unregistered root/session;
- untracked child transition;
- unknown clone semantics when a dependent proposition needs them.

### `process.exec_succeeded`

**Statement:** a tracked task completed a successful executable-image transition to the observed executable identity under the backend's exec semantics.

For current ptrace, a successful ptrace exec event is materially stronger than merely reading an `execve` pathname argument at entry.

Does not imply executable file-content identity unless separately bound to an object/digest proposition.

### `process.lineage_associated`

**Statement:** a canonical actor/execution chain is causally associated with another proposition under the session lifecycle model.

This proposition is a dependency/provenance statement, not merely presentation metadata.

Invalidated or weakened by lifecycle gaps, unknown descendant membership, or execution-chain overflow.

## 3. Path/file propositions

### `file.pathname_attempt_observed`

**Statement:** a tracked task presented a pathname value to a covered file-related syscall at the observer's argument-observation point.

For ptrace entry-time pointer reads this is **userspace-argument evidence**.

It explicitly does **not** state:
- that the kernel consumed the exact same bytes after resume;
- that path resolution succeeded;
- that a file object was opened/accessed;
- that the named path identified the later fd object.

PATH-TOCTOU is a permanent counterexample unless a stronger observation point proves otherwise.

Subfields may carry operation intent such as open/create/delete and open/openat/openat2 flags.

### `file.open_intent_observed`

**Statement:** a covered open-family call expressed a privacy-safe set of open/resolve intent flags at the observation point.

This is distinct from successful open and object identity.

### `file.open_fd_associated`

**Statement:** a covered open-family syscall returned a successful fd and the observer associated that fd with a runtime path/object surrogate under its fd-resolution mechanism.

For current ptrace this is a **post-success correlated fd identity**, not an atomic proof of the object returned by the kernel under all concurrency.

Required dependencies include:
- successful syscall result;
- trustworthy fd-table state for the proposition scope;
- successful fd identity resolution;
- no invalidating shared-fd reuse ambiguity.

### `file.fd_read_effect_observed`

**Statement:** a covered positive-byte read-like operation occurred on an fd whose identity was associated under the observer's fd-state model.

Required dependencies include valid fd-table lineage and an admissible `file.open_fd_associated`/inherited-fd identity path.

It does not cover memory-mapped reads or all io_uring paths under current ptrace semantics.

### `file.fd_write_effect_observed`

Same structure as `file.fd_read_effect_observed`, for covered positive-byte write-like operations.

### `file.rename_attempt_observed`

**Statement:** a tracked task presented source and destination pathname arguments to a covered rename-family syscall.

Current canonical behavior must not be described as proof that rename succeeded merely because a rename event is present. Any future `file.rename_succeeded` proposition requires explicit successful-result evidence.

### `file.delete_attempt_observed`

**Statement:** a tracked task presented a pathname to a covered delete/unlink-family syscall.

Again, attempt is distinct from success and object deletion.

### Future candidate: `file.kernel_object_operation_observed`

Reserved for a kernel-hook/object-grounded proposition only if a research backend can prove a well-defined object identity at the relevant hook.

It must not be synthesized from current pathname-attempt evidence.

## 4. Network propositions

### `network.connect_destination_attempt_observed`

**Statement:** a tracked task presented a covered socket destination to `connect` at the observer's argument-observation point.

For current ptrace this is destination-attempt metadata, not proof that:
- the connection succeeded;
- packets were transmitted;
- DNS/hostname intent is known;
- the sockaddr bytes could not race before kernel consumption.

### Future candidate: `network.connection_established`

Requires explicit post-result/kernel-state evidence and is not implied by current `NetworkConnectAttempt`.

## 5. Observer/session propositions

### `observer.session_scope_established`

**Statement:** the observer established its declared root/session scope before authority-bearing evidence was accepted.

### `observer.lifecycle_complete`

**Statement:** required lifecycle/descendant events for the declared proposition set were complete under the session model.

### `observer.transport_complete`

**Statement:** no unbounded producer/transport/drop/decode loss invalidated the declared proposition set.

### `observer.resource_complete`

**Statement:** event/resource limits did not truncate required evidence.

### `observer.capability_supported`

**Statement:** the selected backend actually supports the capability required by a proposition in the current environment/session.

These observer propositions can be transitive prerequisites of behavioral propositions.

## 6. FD-sharing propositions — new explicit family

E4 makes fd-table semantics the first mandatory v3 precision target.

### `process.fd_table_relation`

**Statement:** two tracked tasks have a declared fd-table relationship for a bounded lifecycle interval.

Candidate relation values:
- `shared`;
- `independent_copy`;
- `same_task_continuity`;
- `unknown`.

This is **not yet an accepted wire enum**.

Evidence may include clone/clone3 flags and later exec/thread-group transitions where relevant.

### Critical observation from current ptrace implementation

The ptrace implementation already stores clone/clone3 flags transiently in `PendingSyscall::Clone { flags }` and uses them while constructing live fd-table state. Raw v2 `ProcessSpawn` then discards those flags and keeps only `SpawnMechanism::Clone`.

Therefore the current false-incompleteness problem is at least partly an **evidence-retention/representation gap**, not proof that ptrace can never distinguish `CLONE_FILES`.

S5 must test this hypothesis adversarially before any completeness claim changes.

## 7. Proposition dependencies

Initial dependency graph:

```text
observer.session_scope_established
        |
        +--> process.child_created
        |        |
        |        +--> process.lineage_associated
        |        |
        |        +--> process.fd_table_relation
        |
        +--> process.exec_succeeded

file.pathname_attempt_observed     (argument-observation path)
file.open_intent_observed          (argument-observation path)

process.fd_table_relation
        +
post-success open result
        +
fd identity resolution
        |
        v
file.open_fd_associated
        |
        +--> file.fd_read_effect_observed
        +--> file.fd_write_effect_observed
```

Transport/resource/lifecycle completeness propositions apply as prerequisites according to the event family and failure scope.

## 8. Explicit non-equivalences

P2 freezes these non-equivalences:

- `file.pathname_attempt_observed` != `file.open_fd_associated`;
- lexical canonical path != kernel-object identity;
- `network.connect_destination_attempt_observed` != connection success;
- clone observed != `CLONE_FILES` sharing;
- process spawn != process exec;
- successful target exit != observer completeness;
- backend name != authority level;
- canonical equality != evidence-authority equality;
- two backends producing similar strings != proposition equivalence.

## 9. Privacy boundary

No proposition in S1 requires:

- file contents;
- environment values;
- stdin;
- network payloads;
- full child argv.

If a proposed proposition later requires secret-bearing payload capture, it must be rejected or separately privacy-gated rather than silently expanding the default boundary.

## 10. S1 falsification questions

Before S1 becomes final semantics, red team must try to show:

1. a proposition name still implies more than current evidence proves;
2. two propositions should be split because their invalidators differ;
3. a proposed dependency is missing a lifecycle/transport prerequisite;
4. a proposition cannot be canonicalized deterministically without unsafe information loss;
5. `process.fd_table_relation` cannot be maintained correctly across clone/clone3/exec/dup/close/reuse;
6. an apparent local ambiguity actually invalidates broader session semantics;
7. a backend can satisfy the event shape but not the semantic proposition.

## 11. S1 decision

**`S1_INITIAL_PROPOSITION_TAXONOMY_ACCEPTED_FOR_FALSIFICATION`**

This taxonomy is sufficiently more precise than raw event labels to proceed to S2 authority/completeness design.

It does not authorize schema v3 runtime integration, public behavior changes, or any relaxation of alpha.4 fail-closed handling.