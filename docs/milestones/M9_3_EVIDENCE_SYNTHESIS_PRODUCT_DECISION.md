# M9.3 — Evidence Synthesis / Product Decision

Date: 2026-09-27
Status: **CLOSED — CONTINUE PUBLIC ALPHA UNDER BOUNDED CLAIMS**
Parent gate: issue #51
M9.3 gate: issue #83

## Decision

ExecSurface should continue as a public alpha with the existing public authority boundary.

The M9 evidence does **not** justify a broader product claim, a production-readiness claim, an independent-adoption claim, a general performance claim, or any public eBPF authority expansion.

The current public product remains:

- public release: `v0.1.0-alpha.3`;
- supported public scope: Linux x86_64;
- public correctness-reference backend: native `ptrace`;
- eBPF: research-only;
- baseline, policy, verdict and evidence authority: unchanged.

No runtime semantics change is authorized by M9.3.

No new binary release is required solely by this synthesis. A future release should be tied to an evidence-backed runtime/reliability/UX change or a separately justified distribution change, not to claim inflation or milestone numbering.

## 1. What M9.1 established

M9.1 closed with mixed evidence preserved.

Accepted zero-contact external runtime evidence showed that the post-hardening ptrace product completed baseline creation and two unchanged checks with zero findings on two pinned, unmodified external CLI runtime workloads:

- `BurntSushi/ripgrep`;
- `junegunn/fzf`.

This is `ZERO_CONTACT_EXTERNAL_REPRO` compatibility evidence. It is **not** independent adoption.

M9.1 also preserved PARTIAL external build/test evidence where legitimate run-to-run temporary build/test artifacts generated large execution-surface variation. Those results must not be normalized away merely to force PASS.

### Performance interpretation

On the exact tested GitHub-hosted environment, the accepted runtime cases crossed the predeclared M6.5 ptrace-cost trigger:

- ripgrep runtime: direct median about `114.0 ms`, ptrace median about `515.0 ms`, slowdown about `4.52x`;
- fzf runtime: direct median about `114.1 ms`, ptrace median about `565.1 ms`, slowdown about `4.95x`.

The build/test cases also showed substantial ptrace overhead.

These are exact-workload/exact-host measurements. They do **not** support a universal Linux, CI, or production performance claim.

## 2. eBPF synthesis

The predeclared M6.5 trigger legitimately opened one bounded same-workload eBPF value screen.

That experiment ended:

`KILLED FOR VALUE CLAIM / PARTIAL_NO_VALUE_CLAIM`

The current per-invocation libbpf path failed its own completeness/health gates on the trigger workloads before accepted measured samples could begin. Observed failures included `incomplete_lifecycle` and `incomplete_capability`.

Therefore:

- no eBPF performance-value claim is established;
- no eBPF PASS authority is established;
- no eBPF `learn` or `check` authority is established;
- no backend auto-selection is authorized;
- no baseline interchangeability is authorized;
- no public eBPF integration is authorized.

`ptrace` remains the correctness-reference and public backend.

A future eBPF experiment may be considered only after the lifecycle/capability completeness failures are closed without weakening the completeness rules.

## 3. M9.2 synthesis

M9.2 is immutable and remains:

`KILLED — NO ACCEPTED INDEPENDENT EVIDENCE IN DECLARED SEARCH SCOPE (2026-09-27)`

Accepted independent records at M9.2 closeout: **0**.

This does not mean ExecSurface has no users or no market interest. It means the declared and frozen discovery scope did not produce evidence satisfying the `INDEPENDENT_USER` provenance/workflow gate.

The later external-evaluation outreach is prospective post-M9.2 work. It does not reopen, rewrite or retroactively change M9.2. A future qualifying third-party run may be accepted as new evidence under the existing provenance/privacy rules.

## 4. Evidence-backed product interpretation

### Supported

The current evidence supports saying that ExecSurface:

- is a public Linux x86_64 developer tool for detecting **observed execution-surface drift** under a recorded observer and explicit policy;
- has a public self-service installation and evaluation path;
- has completed zero-contact compatibility evaluation on pinned external workloads, including accepted ptrace runtime cases for ripgrep and fzf;
- fails closed on incomplete evidence under the current authority model;
- preserves negative and partial evidence rather than editing it away.

### Not supported

The current evidence does **not** support saying that ExecSurface:

- proves software is safe;
- is antivirus, EDR, malware detection, or a sandbox;
- has established independent adoption;
- is production-ready across Linux generally;
- has universally acceptable ptrace performance;
- has a validated faster eBPF backend;
- can interchange ptrace/eBPF baselines;
- should automatically select eBPF;
- has solved build/test nondeterminism generally.

## 5. Public wording freeze

Preferred bounded wording:

> ExecSurface detects observed runtime execution-surface drift under its recorded observer and explicit policy. The current public alpha supports Linux x86_64 with native ptrace as the correctness-reference backend.

For external evidence:

> Zero-contact compatibility evidence exists on pinned external workloads. Independent third-party adoption has not yet been established under the project's strict evidence gate.

For performance:

> Some tested external workloads showed substantial ptrace overhead on the exact measured host. No universal performance claim is made.

For eBPF:

> eBPF remains research-only. The current value experiment did not pass completeness gates and established no public performance or authority claim.

## 6. Evidence-backed next work

Priority order:

1. **Prospective independent evidence intake**
   - continue monitoring post-M9.2 outreach and public evaluation submissions;
   - classify replies/runs prospectively without rewriting M9.2;
   - preserve negative and collaborative results distinctly from `INDEPENDENT_USER`.

2. **Ptrace cost reduction without semantic weakening**
   - optimize only under the existing completeness/evidence rules;
   - use the same pinned real workloads and preregistered measurement rules for before/after evidence;
   - no performance claim without accepted samples.

3. **Build/test ephemeral-behavior research**
   - investigate an explicit, typed, narrowly scoped treatment of legitimate ephemeral build/test artifacts;
   - do not introduce blanket temp-path suppression or post-hoc normalization merely to obtain PASS;
   - any abstraction must preserve causal/runtime meaning and be independently testable.

4. **eBPF completeness research**
   - only after closing `incomplete_lifecycle` / `incomplete_capability` at the collector level;
   - remains research-only until a separate authority gate succeeds.

5. **Self-service/discovery maintenance**
   - keep Marketplace, Discussions, issue forms, quickstarts and release references aligned;
   - discovery metadata never counts as adoption evidence.

## 7. Release decision

**Keep `v0.1.0-alpha.3` as the current public alpha.**

M9.3 does not justify promoting the maturity label or cutting a new binary release by itself.

A future release is justified when there is a concrete evidence-backed product change, such as:

- a semantics-preserving ptrace performance improvement;
- a validated reliability/UX improvement on real workloads;
- an explicit and proven build/test ephemeral abstraction;
- or another bounded product change with clean CI/evidence.

## 8. M9 final result

M9 succeeded as an evidence program even though it did not establish adoption.

It produced:

- real external-workload compatibility evidence;
- preserved negative/partial build-test evidence;
- measured ptrace cost sufficient to trigger a bounded eBPF value experiment;
- a negative eBPF value result that prevented premature backend promotion;
- a strict independent-adoption gate with zero accepted records in the frozen search scope;
- a public self-service and external-evaluation intake surface.

Final product decision:

**CONTINUE PUBLIC ALPHA / KEEP PTRACE AUTHORITY / KEEP EBPF RESEARCH-ONLY / WITHHOLD ADOPTION AND GENERAL PERFORMANCE CLAIMS / PRIORITIZE PROSPECTIVE EXTERNAL EVIDENCE AND SEMANTICS-PRESERVING COST/UX WORK.**

No history rewrite, negative-evidence deletion, runtime semantic relaxation, evidence-authority change, or backend-authority expansion is authorized by this closeout.
