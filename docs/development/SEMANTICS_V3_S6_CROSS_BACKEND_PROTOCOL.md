# ExecSurface — Semantics v3 S6 Cross-Backend Proposition Mapping Protocol

Date: 2026-09-29
Tracking: #101
Parent program: #100
Branch: `development/post-alpha4-behavioral-integrity`
Predecessor: `SEMANTICS_V3_S5_SHARED_FD_RESULT.md` — **PASS_RESTRICTED**
Status: **PREREGISTERED BEFORE S6 MAPPING/IMPLEMENTATION**

## Question

Can ExecSurface map current ptrace and research eBPF evidence into the Semantics v3 proposition vocabulary without laundering backend-specific observations into stronger semantic claims or implying whole-surface equivalence?

## Existing evidence that constrains S6

S6 does not reopen M8.5. M8.5 already established bounded parity only for controlled witnesses and retained counterexamples.

The current experimental eBPF capability intersection is limited to:
- process spawn lineage;
- process exec occurrence;
- focused positive successful-open fd identity;
- explicit loss/truncation visibility.

M8.5 explicitly did **not** establish:
- full-surface equivalence;
- clone3 universal parity;
- successful-open absence equivalence under unresolved identity;
- cross-backend baseline interchangeability;
- eBPF PASS authority.

S5 additionally established that ptrace can retain enough spawn-transition information in the tested paths to classify fd-table relation more precisely in a research v3 record, while public raw v2 remains unchanged and fail-closed.

## Immutable boundaries

S6 MUST NOT:
- reopen or rewrite M8.5 parity results;
- infer semantic equivalence from event names, counts, strings, PID/TID equality, or backend prestige;
- promote unsupported eBPF classes to comparable;
- authorize public eBPF `learn`, `check`, PASS, auto-selection, or baseline interchangeability;
- reinterpret v2 evidence as v3;
- treat pathname identity as kernel-object identity;
- treat a missing witness as proof of absence when relevant identity/completeness is unresolved;
- collapse authority into one scalar backend score;
- change alpha.4 runtime behavior.

## S6-A — Proposition support matrix

For each Semantics v3 proposition, classify each backend as one of:

- `supported_bounded` — backend can support the proposition under explicit prerequisites;
- `supported_attempt_only` — backend supports only a weaker attempt/intent proposition;
- `unsupported` — backend does not claim the proposition;
- `blocked_incomplete` — backend nominally has relevant evidence but current loss/identity/lifecycle state prevents the proposition;
- `research_candidate` — plausible support exists but no accepted evidence yet.

The initial matrix must cover at least:

### Process/lifecycle
- `process.child_created`
- `process.exec_succeeded`
- `process.lineage_associated`
- `process.fd_table_relation`

### File/path
- `file.pathname_attempt_observed`
- `file.open_intent_observed`
- `file.open_fd_associated`
- `file.fd_read_effect_observed`
- `file.fd_write_effect_observed`
- `file.rename_attempt_observed`
- `file.delete_attempt_observed`
- future `file.kernel_object_operation_observed`

### Network
- `network.connect_destination_attempt_observed`
- future `network.connection_established`

### Observer/session
- `observer.session_scope_established`
- `observer.lifecycle_complete`
- `observer.transport_complete`
- `observer.resource_complete`
- `observer.capability_supported`

## S6-B — Authority mapping

For every `supported_bounded` proposition, record the guarantees actually established by the observation path.

At minimum keep separate:
- observation point;
- identity grounding;
- temporal meaning;
- causal scope;
- completeness prerequisites;
- ambiguity/loss annotations.

A backend may support the same proposition kind with different guarantee sets. Same proposition name therefore does not automatically mean interchangeable evidence.

## S6-C — Re-project accepted M8.5 witnesses into v3

Re-use, do not rerun by default, the accepted M8.5 evidence to map only the propositions already proved there:

1. controlled fork/clone/vfork lineage -> candidate `process.child_created` / `process.lineage_associated` mapping;
2. controlled successful exec -> `process.exec_succeeded` mapping;
3. focused `/dev/zero` positive successful-open witness -> `file.open_fd_associated` mapping only for the bounded positive existential proposition;
4. explicit producer loss / event limits -> observer completeness propositions;
5. unresolved clone3 or successful-open identity -> explicit blocked/unsupported state, never inferred equivalence.

If the v3 proposition is materially stronger than the M8.5 projection, classify it as unproved rather than stretching the old evidence.

## S6-D — Cross-backend comparison rule

A cross-backend comparison for proposition `P` is admissible only if:

1. both backends declare support for `P` under the current environment;
2. each satisfies the proposition's required guarantee set or an explicitly proved compatible projection;
3. all transitive observer/session prerequisites are admissible;
4. unresolved identity relevant to `P` is absent or explicitly outside the positive proposition being proved;
5. structural actor identity is used instead of raw runtime identifiers;
6. the comparison result is proposition-scoped.

Allowed comparison verdicts remain:
- `equivalent_bounded`;
- `representation_difference`;
- `reference_subset`;
- `candidate_subset`;
- `non_comparable`;
- `contradicted`;
- `blocked_incomplete`.

No verdict implies whole-backend interchangeability.

## S6-E — Mandatory falsification cases

Red team must attempt at minimum:

1. same event count / different spawn semantics;
2. TGID-vs-TID lineage confusion;
3. clone3 unresolved mechanism;
4. positive open witness with unrelated unresolved opens;
5. failed open with unresolved identity and invalid absence inference;
6. ptrace pathname attempt vs eBPF successful-open identity presented as equivalent;
7. `CLONE_FILES` relation evidence presented as proof of later fd-effect completeness;
8. backend name (`eBPF`, `kernel`, `ptrace`) used as an authority shortcut;
9. lifecycle/resource loss hidden by an otherwise matching behavioral witness;
10. unsupported read/write/network proposition silently projected from a supported open/process proposition.

## Acceptance

S6 can close `PASS_RESTRICTED` only if:

1. every S1 proposition has an explicit support state for ptrace and current research eBPF;
2. accepted M8.5 witnesses are mapped without strengthening their historical claims;
3. same proposition kinds can retain different authority/completeness guarantees without forced equality;
4. unsupported and unresolved classes remain non-comparable or blocked;
5. falsification cases do not produce an unclassified authority-laundering path;
6. no public alpha.4 behavior or backend authority changes.

If the mapping cannot express the existing M8.5 boundaries cleanly, record `S6_SEMANTIC_MAPPING_INSUFFICIENT` and revise the v3 taxonomy/authority model before proceeding.

## Non-claim

Even an S6 PASS would establish only a bounded backend-independent semantic mapping framework. It would not establish full-surface ptrace/eBPF equivalence, production eBPF value, public v3 migration, or cross-backend baseline compatibility.

## Next gate after S6

If S6 closes successfully, proceed to **S7 — v2 preservation / v3 compatibility and migration design** before any runtime integration or public schema proposal.
