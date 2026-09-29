# ExecSurface — P4-A1.3 Ptrace Adapter Adversarial Gate

Date: 2026-09-29
Parent program: #100
Parent P4: #107
Branch: `development/post-alpha4-behavioral-integrity`
Predecessor source: `8ad1fcf1d9ab474fd6a0d007f198844aef4694fb`
Predecessor workflow: `36584250301`
Predecessor decision: `P4_A1_2_PTRACE_MAPPING_PASS_RESEARCH_ONLY`
Status: **PREREGISTERED — ADVERSARIAL IMPLEMENTATION MAY START**

## Objective

Attempt to falsify the research-only ptrace-to-proposition mapping before A1 may close and before any A2 authority-gap decision is allowed.

This gate does not seek more coverage. It tries to create false authority, false completeness, negative-proof laundering, actor substitution, or causal-lineage substitution using the frozen raw-v2 evidence model.

## Fixed roles

1. **Innovation Scientist / Systems Architect** — seek the smallest proposition-preserving mapping; no collector expansion.
2. **Anti-Drift / Scientific Integrity Reviewer** — rejects post-hoc weakening, missing-gap relabeling, or public-v2 reinterpretation.
3. **Independent Falsifier / Red Team** — owns the adversarial cases and attempts false `direct + complete` states.
4. **Independent Milestone Reviewer** — verifies source SHA, exact test set, CI evidence, retained failures, and promotion boundary.

## Dynamic specialists

- Linux ptrace/process lifecycle
- filesystem pathname/object identity
- fd lifecycle and `CLONE_FILES`
- causal provenance / exec lineage
- Rust typed evidence APIs
- deterministic serialization/reproducibility
- CI evidence sealing

## Immutable boundaries

- public `v0.1.0-alpha.4`, `main`, stable `@v0.1`, raw-v2 bytes and public verdict behavior remain unchanged;
- no collector modification is authorized by A1.3;
- no new backend is authorized by A1.3;
- capability gaps remain gaps and cannot be interpreted as proof of absence;
- pathname evidence remains attempt-only unless separate object evidence exists;
- raw-v2 clone without retained flags remains ambiguous for exact fd-table relation;
- observer loss/warnings block dependent effect authority;
- actor/path equality never substitutes for causal-lineage identity;
- all failures remain retained.

## Frozen adversarial cases

A1.3 must execute all of the following without changing the assertions after results are observed:

1. **PATH-TOCTOU / object laundering** — an open pathname attempt must not produce `P4.FILE.OPEN_OBJECT` direct authority.
2. **Failed/unsupported open negative-proof laundering** — lack of successful-object evidence must remain an explicit capability gap and must not become proof that no object effect occurred.
3. **Failed/no-success exec laundering** — without a successful raw-v2 `ProcessExec` event, no `P4.EXEC.SUCCESS` record may appear.
4. **Unknown clone relation** — clone evidence without retained flags must make exact fd-table relation and dependent covered FD attribution ambiguous/incomplete.
5. **Observer loss** — `complete=false` or warning-bearing evidence must stop effect mapping and emit loss/health evidence instead.
6. **Causal-chain substitution** — the same final actor/target under a different ancestor chain must produce a different causal proposition; matching final executable is insufficient.
7. **Wrong-actor substitution** — the same target/effect under a different current actor must not reuse the original proposition authority identity.
8. **Unsupported/no-event negative proof** — a proposition outside raw-v2 proof strength must stay a declared gap; absence of a record must never be treated as a supported negative proposition.

## Acceptance

A1.3 may pass only if:
- all frozen adversarial cases pass;
- no false `direct + complete` path is found;
- no capability gap is converted into proof of absence;
- canonical A0/A1 tests remain green;
- Semantics v3 tests remain green;
- public M11 shared-FD contract remains green;
- no public/runtime file is modified.

Allowed outcomes:
- `P4_A1_3_FALSIFICATION_PASS_BOUNDED`
- `P4_A1_FALSE_AUTHORITY_PATH_FOUND`
- `P4_A1_PUBLIC_CONTRACT_REGRESSION`
- `P4_A1_INCOMPLETE_EVIDENCE`

Only `P4_A1_3_FALSIFICATION_PASS_BOUNDED`, followed by the A1.4 public anti-drift reproof, can close A1 as `P4_A1_PTRACE_ADAPTER_PASS_RESEARCH_ONLY` and authorize A2.