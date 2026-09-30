# ExecSurface — P6 Competitive Falsification Closeout Decision

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/P6_COMPETITIVE_FALSIFICATION_PROTOCOL.md`

## Decision

**P6_FACTUAL_COMPARISON_COMPLETE_BOUNDED**

P6 is scientifically complete for the authorized frozen comparison scope.

This decision is factual and bounded. It does not rank products, name an overall winner, establish full semantic equivalence, or authorize a public release.

## Closed gates

### P6-A0 — source/capability freeze

Decision: `P6_A0_OVERLAP_MATRIX_FROZEN`

Accepted evidence:
- source: `f2174f08134d62ed3e19879b8ff54ff7e1a07aea`;
- run: `36769654116`;
- artifact: `11122743690`;
- digest: `sha256:23fb182b3bc1fc269883059c2ccdd700a414033aeae5fb26264866fcf01f08f5`.

A0 froze exact source/action identities, proposition applicability and the first comparison surface.

### P6-A1 — same-workload observation corpus

Decision: `P6_A1_OBSERVATION_CORPUS_COMPLETE_BOUNDED`

Accepted run:
- source: `110bf2ce400d50ac2cc2ae950de2de4e3a9cba12`;
- run: `36770813526`;
- workload blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`.

Retained historical harness stop:
- source: `bcf0da8e42648541b67a383db3c3c76ebc5b5790`;
- run: `36770551463`;
- S2 retained the real ExecSurface `ERROR/INCOMPLETE` product evidence while the harness incorrectly stopped on stable exit `2` before S3.

The correction changed only harness continuation behavior; it did not change workload, comparator pins, propositions, thresholds or acceptance vocabulary.

### P6-A2 — adversarial false-PASS / identity binding

Decision: `P6_A2_NO_FALSE_PASS_IN_FROZEN_SCOPE`

Accepted evidence:
- source: `411116ee87bbe45270b1fd8ce84b17abcecd8642`;
- run: `36772303119`;
- artifact: `11124710931`;
- digest: `sha256:aa2205eb719baf4a9168ee9694bc6da0cfc5d94fab2230db181ab3fe0a3ba8b8`.

No E1-E5 attack produced ExecSurface native PASS in the frozen scope. The accepted P5 cross-attestation corpus also re-proved 12/12.

### P6-A3 — reproducibility / privacy / CI-friction

Decision: `P6_A3_REPRODUCIBILITY_CONTEXT_COMPLETE_BOUNDED`

Independent repetition 1:
- source: `bba3f34eae5402719fe968c885f2a245c582268d`;
- run: `36772847999`;
- conclusion: `success`.

Independent repetition 2:
- source: `792026731db32547dae319a7ca584138aa42a417`;
- run: `36772939746`;
- conclusion: `success`.

Both repetitions used:
- workload blob `c5986f1acd8bc79a3945e613acc128d5c8a9d168`;
- workflow blob `d1ea125ebd8f0230b49d10b48c716d6743706d08`;
- unchanged comparator pins and acceptance criteria.

ExecSurface produced the same native vector in both repetitions:
- S0 `PASS`;
- S1 `REVIEW`;
- S2 `ERROR/INCOMPLETE`;
- S3 `REVIEW`.

False PASS under S1-S3 across A3 repetitions: `0`.

### P6-A4 — optional performance gate

Decision: `NO_VALID_PERFORMANCE_COMPARISON`

No timing benchmark was executed because the current frozen integrations do not expose an equivalent measured product boundary. Reporting total workflow time, action setup time or one combined overhead number would conflate different lifecycle, output, telemetry and completeness work.

No product speed ranking is authorized.

## Bounded factual matrix

### ExecSurface public alpha.4

Demonstrated in the frozen P6 scope:
- deterministic S0 control PASS;
- S1 process-exec drift REVIEW;
- S3 file-write drift REVIEW with explicit retained file evidence;
- S2 network scenario fails closed as `ERROR/INCOMPLETE` rather than producing a trusted canonical surface;
- baseline tamper/substitution rejected in A2;
- no false PASS survived the frozen adversarial corpus;
- reproducible native result vector across two A3 repetitions;
- no external ExecSurface telemetry endpoint configured by the tested A3 lane.

This does not claim that alpha.4 has the broadest event coverage.

### Harden-Runner

Demonstrated in the frozen P6 scope:
- pinned action operated successfully on GitHub-hosted Ubuntu;
- `example.com:443` network activity correlated to `curl` in retained A1 evidence;
- process and project-file monitoring initialization evidenced;
- job / Security Insights correlation evidenced;
- A3 operational context reproduced under `egress-policy: audit`;
- no attestation-equivalent machine artifact was established by the bounded A1/A3 evidence.

Absence of exact local S1/S3 lines in retained local output is not generalized into product inability.

### cicd-sensor

Demonstrated in the frozen P6 scope:
- pinned action and bundled sensor initialized/finalized successfully;
- A1 retained process ancestry, process-bound network evidence and exact S3 file evidence;
- Runtime Trace predicate carried GitHub run/job/workflow identity;
- A2 established that unchanged replay retains original run identity and that copied unsigned JSON requires an external integrity/signature contract for tamper resistance;
- A3 predicates/debug artifacts reproduced `example.com` evidence with process ancestry across independent runs.

The native predicate field `passed` is not normalized to ExecSurface `PASS`.

### Tetragon

`NOT_TESTABLE_YET` for live P6 because an immutable executable identity satisfying the A0 live-lane rule was not frozen. This is a bounded coverage gap, not a negative product result.

Falco/Tracee were not required for the first live P6 scope because no additional proposition was needed to answer the frozen A1-A3 questions.

## Negative evidence retained

P6 retains rather than removes:
- first A1 harness stop `36770551463`;
- ExecSurface alpha.4 S2 incompleteness;
- Tetragon live `NOT_TESTABLE_YET` status;
- cicd-sensor unsigned copied-predicate mutability observation;
- natural DNS/run-identity/artifact-digest variation across A3 repetitions;
- the decision not to create a misleading performance comparison.

## What P6 establishes

Within the frozen GitHub-hosted Linux CI scope, ExecSurface, Harden-Runner and cicd-sensor expose materially different evidence contracts over overlapping runtime behavior. The experiments are sufficient to preserve those differences proposition-by-proposition without collapsing them into a score or global equivalence claim.

P6 also demonstrates that ExecSurface's chosen differentiation remains centered on fail-closed behavioral verification, explicit evidence semantics and attestable binding rather than broad event-count competition.

## What P6 does not establish

P6 does not establish:
- an overall winner;
- product superiority or inferiority;
- full backend equivalence;
- universal privacy behavior;
- performance ranking;
- Tetragon live behavior in this corpus;
- cross-platform parity;
- public release readiness.

## Next parent-program phase

The next phase is **P7 — Platform / CI Expansion**, and it must remain evidence-gated.

Candidate order remains:
1. Linux arm64 only if evidence parity can be tested;
2. GitLab CI using the same semantic contract;
3. self-hosted CI packaging;
4. additional OS families only under separate evidence contracts.

P7 must not trade semantic confidence for platform count and must not modify the immutable public alpha.4 boundary without a separately authorized promotion gate.
