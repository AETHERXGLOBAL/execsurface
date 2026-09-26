# M8.5 — ptrace/eBPF Semantic Parity & Cross-Backend Comparability

Date: 2026-09-26
Status: **CLOSED / ACCEPTED — BOUNDED PARITY ONLY**
Tracking: #42
Parent: `docs/milestones/M8_EBPF_ARCHITECTURE.md`
Red-team review: `docs/milestones/M8_5_RED_TEAM_REVIEW.md`
Reference backend: `linux-ptrace-metadata-v2`
Candidate backend: `linux-libbpf-metadata-experimental-v1`

## Objective

Determine, per semantic capability, whether the experimental eBPF evidence is actually equivalent to the ptrace correctness reference, a strict subset, a representation difference, non-comparable, contradicted, or blocked by incomplete evidence.

M8.5 does **not** compare raw event counts, raw PIDs/TIDs, or raw sequence numbers as evidence of semantic parity.

M8.5 closure does **not** authorize eBPF PASS, full-surface backend interchangeability, production readiness, or cross-backend baseline compatibility.

## Fixed governance roles

- **Innovation Scientist / Architect:** define the strongest backend-independent semantic projection and comparability model without coupling product semantics to ptrace or eBPF implementation details.
- **Deviation Prevention / Scientific Integrity:** reject event-name/count similarity as proof, preserve counterexamples, classify every mismatch, and prevent premature eBPF PASS authority.
- **Independent parity red team:** attempt to falsify every equivalence claim and verify that incomplete or unsupported evidence cannot silently become parity or PASS.

## Parity states

Machine-readable parity verdicts are:

- `equivalent` — the selected semantic class has the same backend-independent meaning and the controlled witnesses agree.
- `reference_subset` — the ptrace evidence is a proved subset of candidate evidence for the selected class.
- `candidate_subset` — the eBPF evidence is a proved subset of ptrace evidence for the selected class.
- `representation_difference` — both backends establish related facts, but the current evidence interfaces do not expose an equivalent semantic projection.
- `non_comparable` — one backend does not claim the semantic class or the current proof obligations cannot be aligned.
- `contradicted` — both backends claim the class, evidence is sufficiently complete, and the controlled semantic projections disagree.
- `blocked_incomplete` — loss, truncation, decode, collector, lifecycle, unresolved identity, or equivalent evidence-health failure prevents the requested parity conclusion.

## Structural identity rule

Separate ptrace and eBPF runs have different runtime process identifiers and scheduling sequences. Therefore:

- raw PID/TID equality is forbidden as a parity criterion;
- raw event sequence equality is forbidden as a parity criterion;
- process lineage is projected into structural roles such as `root`, `root/fork#1`, `root/fork#1/clone#1`;
- occurrence indexes are local to one parent and spawn mechanism and are used only in deterministic fixtures where that ordering is part of the workload contract;
- adversarial fixtures challenge ordering and parent-identity assumptions before any scoped equivalence is accepted.

## Health gate

Parity is evaluated only after evidence-health classification.

Hard blockers for selected-class parity include:

- `incomplete_loss`;
- `incomplete_limit`;
- `incomplete_decode`;
- `incomplete_collector`;
- `incomplete_lifecycle`;
- unresolved identity when the requested proposition depends on proving absence or contradiction;
- ptrace `complete=false` when the warning is not solely an explicitly out-of-scope capability declaration.

The eBPF collector currently reports `incomplete_capability` because it intentionally supports only a strict subset of the full ExecSurface surface. That state:

- does **not** authorize full-surface equivalence;
- may permit class-local comparison only for explicitly shared capabilities and when no relevant hard blocker exists.

Full-surface cross-backend comparability remains **false**.

## Candidate capability intersection

The experimental eBPF descriptor currently supports only:

1. `ProcessSpawnLineage`
2. `ProcessExecOccurrence`
3. `SuccessfulOpenFdIdentity`
4. `LossTruncationVisibility`

Other semantic classes remain `non_comparable` unless a later explicit gate expands and proves the candidate capability contract.

## M8.5a — first differential harness — CLOSED / ACCEPTED

The isolated differential harness runs the same deterministic workload independently under ptrace and eBPF, verifies backend identities, maps runtime identities to structural roles separately, and emits machine-readable per-class verdicts.

### Preserved counterexample: syscall name != spawn semantics

The first executable semantic run contradicted parity:

- ptrace: `root|fork|root/fork#1`;
- eBPF: `root|clone|root/clone#1`.

Cause: libc implemented `fork()` through the Linux `clone` syscall. The original eBPF collector labeled the event by syscall name, while ptrace classified process creation by Linux clone flags / exit-signal semantics.

The acceptance contract was not weakened. Classic clone classification was corrected:

- `CLONE_VFORK` -> `vfork`;
- `CSIGNAL == SIGCHLD` -> `fork`;
- otherwise -> `clone`.

Accepted evidence at commit `2a1bf761207ca24cab5a87595a9cacd66ac8c340`:

- normal repository CI: **PASS** — run `36252039674`;
- M8.5 differential: **PASS** — run `36252039675`;
- controlled fork/exec fixture:
  - `ProcessSpawnLineage = equivalent`;
  - `ProcessExecOccurrence = equivalent`;
  - unsupported classes = `non_comparable`;
  - `full_surface_comparable = false`;
  - `ebpf_pass_authorized = false`;
- deliberately mutated same-count/different-mechanism candidate: `ProcessSpawnLineage = contradicted`;
- forced incomplete evidence: selected shared classes = `blocked_incomplete`.

Preserved negative evidence:

- run `36251621000` — setup-only rustfmt failure before semantics executed;
- run `36251855360` — real fork-vs-clone semantic counterexample.

Conclusion: **PROVED only for the controlled single-threaded fork -> child exec projection.**

## M8.5b — nested-thread lineage — CLOSED / ACCEPTED

An adversarial nested-thread fixture exposed a second real contradiction.

Expected ptrace structure retained task-level parentage, while eBPF initially flattened a child created by a non-leader thread back to the process leader.

Cause: eBPF used TGID as spawn parent identity. Ptrace raw spawn evidence is task/TID scoped.

Correction: use the current TID for spawn parent identity. This retains ordinary leader behavior while preserving non-leader task lineage.

Accepted evidence at commit `24afe8ce75cdd4d121460939a0b25b0f2b647791`:

- normal CI: **PASS** — run `36252591556`;
- M8.5 differential: **PASS** — run `36252591634`;
- controlled fork/exec parity retained;
- nested classic-clone lineage became `equivalent`;
- same-count semantic mutation remained `contradicted`;
- incomplete evidence remained `blocked_incomplete`;
- `full_surface_comparable = false`;
- `ebpf_pass_authorized = false`.

Preserved negative evidence:

- run `36252411516` — nested-thread parent-attribution contradiction before TID correction.

Conclusion: **PROVED for the controlled fork and nested classic-clone lineage witnesses only.**

## M8.5c — vfork / clone3 hardening — CLOSED / ACCEPTED

### vfork

Controlled vfork -> exec semantics reached scoped parity with ptrace.

The first fixture used Rust `libc::vfork()` with all Rust-owned objects prepared before the call and only `execl/_exit` in the child. The Rust/libc binding nevertheless emits an explicit deprecation/memory-corruption warning for `vfork`, so the red team rejected that path as the final evidence fixture.

Final vfork evidence therefore uses a native C fixture compiled with clang. It opens and reads `/dev/zero`, invokes `vfork`, performs only `execl`/`_exit` in the child, waits for clean child exit, and keeps the descriptor alive briefly for metadata resolution. The Rust vfork execution path was removed.

### clone3

An attempted user-memory metadata path required a GPL-restricted BPF helper under the tested attachment/program context. ExecSurface retained the Apache-2.0 license boundary rather than changing license or weakening the gate.

When clone3 mechanism metadata cannot be established defensibly, the candidate remains unresolved/fail-closed rather than inventing `fork`, `vfork`, or `clone` equivalence.

Accepted behavior:

- native-C vfork controlled fixture: scoped parity **PASS**;
- clone3 adversarial fixture: unresolved mechanism remains explicit and does not become equivalence;
- eBPF PASS authority remains disabled.

## M8.5d — SuccessfulOpenFdIdentity — CLOSED / ACCEPTED

### Initial representation gap

Ptrace proves successful open internally on syscall exit and stores returned FD/path state, but public Observation v2 does not serialize a dedicated successful-open identity event. A later FD-attributed read/write can expose a successful-open witness.

The eBPF candidate explicitly emits `successful_open_identity { pid, fd, path? }` after successful open.

Direct raw-event equality is therefore invalid.

### Backend-independent focused witness

M8.5d introduced a parity-only controlled projection without changing public Observation v2:

- ptrace positive witness: later `FileDescriptorAccess` for the exact focused path;
- eBPF positive witness: resolved `successful_open_identity` for the exact focused path;
- numeric FD equality across runs is not required;
- structural process role + focused path establish the controlled proposition.

The positive fixture opens `/dev/zero`, reads from the returned descriptor, and keeps the descriptor alive briefly so userspace metadata resolution is not deliberately raced.

### Preserved counterexample: unrelated unresolved open blocked the positive proposition

Run `36255562056` showed that the first focused harness was too conservative: it blocked the `/dev/zero` positive proposition whenever *any unrelated* candidate successful-open event lacked path identity.

The scientific correction distinguished two different propositions:

- **positive existence:** matching resolved non-empty witnesses for the exact focus path can prove that focused open occurred even when unrelated opens are unresolved;
- **absence / contradiction:** missing focused witnesses cannot prove absence while any relevant candidate successful-open identity remains unresolved.

The gate was not weakened; the proposition was made explicit.

### Positive successful-open evidence

A later controlled run established:

- ptrace witness: `root|/dev/zero`;
- eBPF witness: `root|/dev/zero`;
- focused `SuccessfulOpenFdIdentity = equivalent` for this controlled positive existential proposition;
- unrelated unresolved candidate opens, when present, remain recorded;
- `full_surface_comparable = false`;
- `ebpf_pass_authorized = false`.

### Failed-open negative evidence and resolution variability

A controlled missing path `/__execsurface_m8_5_missing__/open-probe` is observed as an open attempt by ptrace but must never be promoted into successful-open evidence.

Two valid runner outcomes were observed:

- when unrelated candidate successful-open identities remain unresolved, absence is `blocked_incomplete`;
- when every candidate successful-open identity in that run resolves and both focused witness sets are empty, the class is `non_comparable`.

In neither case is the missing path promoted to successful-open evidence, and in neither case does absence become equivalence.

Run `36256614065` was deliberately preserved because it exposed this scheduler/runner-dependent resolution-health distinction: the semantic harness correctly returned `non_comparable`, while the workflow still expected `blocked_incomplete`. The workflow was corrected to bind its expected verdict to the actual candidate resolution health rather than to one runner-specific shape.

## Final accepted M8.5 evidence

Final closure candidate at commit `a73cf1bd7ce485d0649268480cc3696113cb8a6c`:

- normal repository CI: **PASS** — run `36256996309`;
- M8.5 semantic differential: **PASS** — run `36256996339`;
- clean fork/exec class-local parity: **PASS**;
- focused `/dev/zero` positive successful-open proposition: `equivalent`;
- failed missing-path open: **not promoted**, with verdict bound to actual resolution health rather than a hard-coded runner outcome;
- native C vfork controlled parity: **PASS**;
- clone3 unresolved behavior remains fail-closed;
- same-count/different-semantics mutation remains `contradicted`;
- forced incomplete evidence remains `blocked_incomplete`;
- `full_surface_comparable = false`;
- `ebpf_pass_authorized = false`.

## Independent red-team closure

The independent review in `M8_5_RED_TEAM_REVIEW.md` attempted to defeat parity through:

- raw event counts;
- syscall-name equivalence;
- TGID/TID parent confusion;
- clone3 overclaim;
- loss/truncation;
- unrelated unresolved successful opens;
- failed-open absence inference under variable resolution health;
- unsafe/ambiguous vfork fixture construction;
- unsupported capability promotion;
- premature CLI/PASS integration.

Decision: **ACCEPT M8.5 for the explicitly proved semantic projections only.**

## Explicit comparability decision

After M8.5:

- ptrace remains the correctness reference;
- selected controlled `ProcessSpawnLineage` projections: **parity-proven for the recorded classic fork/clone/vfork fixtures only**;
- selected controlled `ProcessExecOccurrence` projections: **parity-proven for the recorded structural-role fixtures only**;
- selected controlled used-FD positive `SuccessfulOpenFdIdentity`: **parity-proven for the focused positive witness proposition only**;
- unresolved absence: **blocked / not proved**;
- clone3 universal parity: **not proved**;
- unsupported capability classes: **non-comparable**;
- full-surface cross-backend comparability: **false**;
- automatic cross-backend baseline interchangeability: **not authorized**;
- eBPF PASS authority: **not authorized**;
- production readiness: **not implied**.

## Preserved negative evidence summary

M8.5 intentionally retains failures because they define the safe boundary:

- `36251855360` — syscall-name clone vs semantic fork contradiction;
- `36252411516` — nested-thread TGID/TID parent attribution contradiction;
- verifier rejection of the GPL-restricted clone3 helper path under the Apache-2.0 boundary;
- `36255562056` — positive focused-open proposition incorrectly blocked by unrelated unresolved opens;
- `36256075043` — positive focused proof passed and the first negative-test shape exposed the need for evidence-health-dependent absence handling;
- `36256614065` — all relevant candidate identities resolved on a later runner, correctly yielding `non_comparable` and exposing the stale hard-coded `blocked_incomplete` test expectation.

## M8.5 conclusion

**CLOSED / ACCEPTED — bounded semantic parity only.**

M8.5 satisfies the required parity framework and controlled workload matrix without granting broader authority than the evidence supports.

Next gate: **M8.6 — Performance / Compatibility**.

## Non-negotiable boundaries retained

- ptrace remains the correctness reference.
- eBPF PASS authority remains **NOT AUTHORIZED**.
- selected-scenario parity cannot be generalized to unsupported capability classes.
- same event count is not semantic equivalence.
- raw runtime identity equality is not semantic equivalence.
- a representation gap is not silently converted into equivalence.
- unresolved or incomplete evidence cannot prove absence or parity where the proposition depends on the missing evidence.
- no public Observation v2 schema churn was required by M8.5.
