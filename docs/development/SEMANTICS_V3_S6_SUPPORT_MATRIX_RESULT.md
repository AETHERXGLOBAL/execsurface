# ExecSurface — Semantics v3 S6 Cross-Backend Proposition Mapping Result

Date: 2026-09-29
Tracking: #101
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/SEMANTICS_V3_S6_CROSS_BACKEND_PROTOCOL.md`
Historical evidence basis:
- `docs/milestones/M8_5_SEMANTIC_PARITY.md`
- `docs/milestones/M8_EBPF_ARCHITECTURE.md`
- `docs/development/SEMANTICS_V3_PROPOSITION_TAXONOMY.md`
- `docs/development/SEMANTICS_V3_AUTHORITY_COMPLETENESS_MODEL.md`
- `docs/development/SEMANTICS_V3_S5_SHARED_FD_RESULT.md`

## Formal result

**S6 CLOSED — PASS_RESTRICTED**

The current ptrace reference and research eBPF evidence can be represented in the Semantics v3 proposition/guarantee model without requiring whole-backend equivalence, a scalar authority score, or reinterpretation of historical M8.5 claims.

This is a semantic mapping result only. No public backend authority, product integration, or baseline interchangeability is authorized.

## Support-state vocabulary

- `supported_bounded` — accepted evidence supports the proposition under explicit prerequisites and scope.
- `supported_attempt_only` — only a weaker attempt/intent proposition is supported.
- `unsupported` — current backend contract does not claim the proposition.
- `blocked_incomplete` — relevant capability exists but evidence health/identity prevents the requested proposition in the current case.
- `research_candidate` — plausible mechanism exists but no accepted proposition proof yet.

## Proposition support matrix

| Semantics v3 proposition | ptrace reference / research-v3 path | research eBPF path | Boundaries |
|---|---|---|---|
| `process.child_created` | `supported_bounded` | `supported_bounded` | eBPF bounded to accepted classic fork/clone/vfork witnesses; unresolved clone3 is not generalized |
| `process.exec_succeeded` | `supported_bounded` | `supported_bounded` | bounded structural-role witnesses only; no executable content identity claim |
| `process.lineage_associated` | `supported_bounded` | `supported_bounded` | structural actor mapping; raw PID/TID equality prohibited |
| `process.fd_table_relation` | `supported_bounded` in S5 research-v3 record; public v2 does not retain it | `research_candidate` | no accepted eBPF v3 proof yet for exact `CLONE_FILES` relation contract |
| `file.pathname_attempt_observed` | `supported_bounded` | `unsupported` under current declared candidate contract | ptrace userspace-argument/pre-kernel evidence only; never kernel-object identity |
| `file.open_intent_observed` | `supported_bounded` | `unsupported` under current declared candidate contract | intent is distinct from success |
| `file.open_fd_associated` | `supported_bounded` for covered post-success fd correlation | `supported_bounded` only for accepted focused positive successful-open witness | absence under unresolved candidate identity remains blocked/non-comparable |
| `file.fd_read_effect_observed` | `supported_bounded` for covered positive-byte fd operations with admissible fd state | `unsupported` | no projection from open evidence to read effect |
| `file.fd_write_effect_observed` | `supported_bounded` for covered positive-byte fd operations with admissible fd state | `unsupported` | no projection from open evidence to write effect |
| `file.rename_attempt_observed` | `supported_bounded` | `unsupported` | attempt only; no success claim |
| `file.delete_attempt_observed` | `supported_bounded` | `unsupported` | attempt only; no success claim |
| `file.kernel_object_operation_observed` | `unsupported` | `unsupported` under current accepted M8 research evidence | future BPF-LSM/kernel-hook candidate only; not synthesized from paths/fds |
| `network.connect_destination_attempt_observed` | `supported_bounded` | `unsupported` | ptrace destination attempt, not connection success |
| `network.connection_established` | `unsupported` | `unsupported` | future proposition requires separate evidence |
| `observer.session_scope_established` | `supported_bounded` | `supported_bounded` | eBPF bounded to accepted session/epoch/root-barrier model |
| `observer.lifecycle_complete` | `supported_bounded` under ptrace health contract | `supported_bounded` only when persistent-session health prerequisites pass | historical intermittent lifecycle failures remain negative evidence |
| `observer.transport_complete` | `supported_bounded` with native transport requirement effectively not-applicable where no separate producer channel exists | `supported_bounded` when explicit loss/routing/decode gates pass | eBPF ring-buffer/collector loss is an explicit blocker |
| `observer.resource_complete` | `supported_bounded` | `supported_bounded` | event/resource truncation blocks dependent propositions |
| `observer.capability_supported` | `supported_bounded` | `supported_bounded` | eBPF capability set is a strict subset and remains explicit |

## Authority/guarantee mapping

### Ptrace examples

#### `file.pathname_attempt_observed`
- observation point: `userspace_argument_pre_kernel`;
- identity basis: `lexical_argument` or `trace_time_dirfd_resolved_argument`;
- temporal binding: `pre_operation_intent`;
- causal binding: direct/state-machine actor correlation as applicable.

This profile cannot satisfy a kernel-object-grounded requirement.

#### `file.open_fd_associated`
- observation point: `syscall_result_post_operation` plus derived fd state;
- identity basis: `runtime_fd_path_correlated`;
- temporal binding: successful result / post-operation derived state;
- causal binding: `state_machine_correlated`;
- dependencies include admissible fd-table relation and observer completeness.

This remains distinct from kernel-object identity.

#### `process.fd_table_relation`
- observation source in current research path: retained clone/clone3 semantic bits where available plus spawn transition context;
- relation: `shared`, `independent_copy`, continuity, or `unknown`;
- unknown is non-satisfying for propositions requiring exact relation;
- exact relation alone does not establish later fd-effect completeness.

### Research eBPF examples

#### `process.child_created` / lineage
- observation point: accepted kernel task-creation evidence / tracepoint path;
- identity: structural process/task role rather than cross-run numeric identity;
- temporal binding: lifecycle transition;
- causal binding: lineage-derived/session-membership state;
- prerequisites: root/session established, no relevant loss/routing/lifecycle blocker.

#### `file.open_fd_associated`
For the accepted M8.5 focused positive witness only:
- positive successful-open evidence is mapped to the bounded proposition;
- identity remains runtime path/fd correlated under the research collector's resolution mechanism;
- it is **not** promoted to `kernel_object_grounded` merely because collection uses eBPF;
- unresolved unrelated opens do not erase a proved exact positive focused witness, but they can block absence/contradiction claims where relevant.

## M8.5 re-projection into v3

Historical results are retained exactly, not strengthened:

1. classic controlled fork/clone/vfork witnesses map to bounded `process.child_created` / `process.lineage_associated` comparison;
2. controlled successful exec witnesses map to bounded `process.exec_succeeded` comparison;
3. focused `/dev/zero` positive witness maps to bounded positive `file.open_fd_associated` comparison;
4. failed/missing-path cases do **not** become successful-open evidence;
5. unresolved successful-open identity blocks absence where the missing evidence is relevant;
6. clone3 unresolved mechanism remains unresolved/non-comparable rather than inferred;
7. forced loss/incompleteness maps to observer completeness failure and `blocked_incomplete` for dependent propositions;
8. unsupported behavioral families remain non-comparable.

No historical M8.5 `equivalent` verdict is generalized beyond its controlled proposition/witness.

## Falsification review

The preregistered S6 falsification cases were mapped as follows.

### F1 — same event count / different spawn semantics
**Classified.** Historical M8.5 counterexample produced contradiction until syscall-name labeling was corrected. Count equality is never semantic equivalence.

### F2 — TGID-vs-TID lineage confusion
**Classified.** Historical nested-thread counterexample demonstrated the failure. S6 uses task/structural lineage semantics and preserves the counterexample.

### F3 — clone3 unresolved mechanism
**Classified.** Remains unresolved/non-comparable; no mechanism or relation is invented.

### F4 — positive focused open with unrelated unresolved opens
**Classified.** Exact positive existential witness may be proved for the focus path while unrelated unresolved events remain recorded; no absence conclusion is inferred from that rule.

### F5 — failed open + unresolved identity used to infer absence
**Blocked.** Missing focused success cannot prove absence while relevant identity is unresolved. Historical runner variability remains preserved.

### F6 — ptrace pathname attempt vs eBPF successful-open identity
**Non-comparable as equivalent propositions.** They are different proposition kinds/temporal guarantees; no authority laundering occurs.

### F7 — `CLONE_FILES` relation presented as proof of all later fd effects
**Blocked by dependency model.** Relation exactness is necessary for some fd propositions but is not sufficient; close/dup/reuse, object identity, lifecycle, resource and transport prerequisites remain independent.

### F8 — backend name used as authority shortcut
**Rejected by model.** `eBPF`, `kernel`, and `ptrace` are sources, not scalar authority levels. Guarantee sets decide admissibility.

### F9 — matching behavior with lifecycle/resource loss
**Blocked incomplete.** Observer completeness prerequisites dominate matching behavioral strings.

### F10 — unsupported read/write/network projected from supported open/process evidence
**Unsupported / non-comparable.** No cross-family promotion is allowed.

## Red-team decision

Within the declared S6 scope, the falsification review found **no unclassified authority-laundering path** that requires changing the S1 taxonomy or S2/S3 authority model.

This does not claim the model is universally complete. New counterexamples remain authoritative inputs to later gates.

## Why the result is restricted

The mapping is intentionally asymmetric:
- ptrace supports a broader current public surface but with userspace/path and fd-correlation limitations;
- research eBPF supports a narrower accepted proposition set with explicit collector/transport health semantics;
- neither backend is globally superior;
- many proposition pairs remain unsupported or non-comparable;
- `file.kernel_object_operation_observed` remains unproved by the current accepted evidence.

Therefore S6 is not a backend-selection result.

## Authority after S6

Unchanged:
- public release remains `v0.1.0-alpha.4`;
- ptrace remains public/default correctness reference;
- public v2 evidence semantics remain authoritative;
- eBPF remains research-only;
- no public v3 learn/check/PASS;
- no cross-backend baseline interchangeability;
- no automatic backend selection;
- no full-surface equivalence claim.

## Next authorized gate

**S7 — v2 Preservation / v3 Compatibility & Migration Design.**

S7 must guarantee that old v2 evidence and lockfiles retain their original meaning forever, v3 uses explicit schema/digest boundaries, and any cross-schema comparison either uses a proved projection or fails comparability explicitly.
