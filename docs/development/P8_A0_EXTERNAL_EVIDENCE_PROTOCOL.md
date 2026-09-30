# ExecSurface — P8-A0 External-Evidence Qualification & Evaluator-Pack Protocol

Date: 2026-09-30
Parent program: #100
P8 issue: #114
External engagement issue: #94
Predecessor: `P7_PLATFORM_CI_RESEARCH_COMPLETE_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO CURRENT EXTERNAL VALIDATION RESULT YET**

## 1. Question

Can ExecSurface freeze an evaluator packet and evidence-qualification contract such that later community responses, reproductions, criticisms, counterexamples, no-fit findings and real-workload reports are recorded without self-validation, endorsement inflation, selective retention or post-hoc reclassification?

A0 is a pack/qualification gate. It cannot itself satisfy P8 external-validation success.

## 2. Hypothesis

External evidence can be processed rigorously if:

1. the target public/research state is explicitly identified before the response is evaluated;
2. evaluator independence/relationship is declared rather than inferred from prestige;
3. assistance level is frozen for the initial attempt;
4. failure/partial/no-fit results are retained exactly;
5. all technically meaningful feedback enters the ledger before AETHER X argues or fixes;
6. assisted follow-up is a separate evidence item and never rewrites the initial attempt;
7. participation, routing, praise and organizational affiliation are non-qualifying by themselves.

## 3. Fixed roles

1. **Innovation Scientist / Systems Architect** — translate qualified criticism into hypotheses and architectural improvements, not rebuttal prose.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks self-validation, cherry-picking, prestige weighting and endorsement/adoption inflation.
3. **Independent Falsifier / Red Team** — attempts to disqualify evidence that is affiliated, assisted, unverifiable, stale, security-sensitive or technically empty.
4. **Independent Critical-Milestone Reviewer** — verifies exact target identities, source locator, timestamps, assistance state, evidence retention and decision wording.

Dynamic specialists:
- reproducibility / scientific-method reviewer;
- OpenSSF/open-source community lead;
- Rust ecosystem reviewer;
- Linux runtime / ptrace / LSM reviewer;
- supply-chain attestation standards specialist;
- vulnerability disclosure reviewer.

## 4. Frozen target identities

Public evaluator target:
- repository: `AETHERXGLOBAL/execsurface`;
- public main at A0 freeze: `b47721ce675444ce029b048edce6fc94631d6408`;
- immutable release source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- release: `v0.1.0-alpha.4`;
- stable Action: `AETHERXGLOBAL/execsurface@v0.1`;
- public platform: Linux x86_64 only.

Research-architecture target, when the evaluator explicitly chooses architecture review rather than product reproduction:
- P7 closeout: `aba8d9ebdc13c76c71552fec0abe9cffbd58c745`;
- P5 decision: `P5_EXISTING_STANDARDS_COMPOSITION_SUFFICIENT_BOUNDED`;
- P6 decision: bounded factual-comparison closeout;
- P7 decision: `P7_PLATFORM_CI_RESEARCH_COMPLETE_BOUNDED`.

Research results do not silently alter public alpha.4 semantics.

## 5. Evidence classes

Qualifying classes:
- `ZERO_ASSISTANCE_REPRODUCTION`;
- `REPRODUCTION_FAILURE`;
- `ARCHITECTURE_CRITICISM`;
- `COUNTEREXAMPLE`;
- `NO_FIT_OR_REDUNDANCY`;
- `INTEROPERABILITY_GUIDANCE`;
- `EXTERNAL_REAL_WORKLOAD_REPORT`;
- `EXTERNAL_CONTRIBUTION`.

Non-qualifying alone:
- `ROUTING_GUIDANCE`;
- `COMMUNITY_PARTICIPATION`;
- `ACKNOWLEDGEMENT_OR_PRAISE`;
- internal AETHER X CI/rehearsal;
- unactionable anonymous opinion;
- organization membership.

Non-qualifying evidence is still retained when relevant; it simply cannot satisfy an external-validation gate.

## 6. Independence / relationship declaration

Every external record must declare one of:
- `UNAFFILIATED`;
- `UPSTREAM_OR_ECOSYSTEM_MAINTAINER`;
- `USER_OR_EVALUATOR`;
- `PRIOR_CONTACT_NO_PROJECT_ROLE`;
- `CONTRIBUTOR_TO_EXECSURFACE`;
- `AETHER_X_AFFILIATED`;
- `UNKNOWN`.

`AETHER_X_AFFILIATED` cannot satisfy external independence.
`CONTRIBUTOR_TO_EXECSURFACE` may provide technically useful feedback but does not qualify as independent zero-assistance reproduction for the contribution area without explicit separation evidence.
Prestige/job title never upgrades evidence quality.

## 7. Assistance levels

Every attempt declares:
- `ZERO_ASSISTANCE` — evaluator uses only frozen public links/docs/artifacts;
- `DOCS_ONLY_CLARIFICATION` — clarification of already-public documentation, no debugging guidance;
- `ASSISTED` — AETHER X gives troubleshooting or procedural help;
- `COLLABORATIVE` — joint investigation;
- `NOT_APPLICABLE` — non-reproduction evidence such as architecture criticism.

An initial `ZERO_ASSISTANCE` failure remains a failure even if a later assisted attempt succeeds.

## 8. Required external evidence fields

Each record must preserve:
- evidence ID;
- evidence class;
- source / channel;
- external actor or stable pseudonymous handle when appropriate;
- relationship declaration;
- received date/time if known;
- exact target release/commit/docs used;
- assistance level;
- evaluator environment when reproduction is attempted;
- external workload origin if applicable;
- raw claim/result summary;
- source locator or archived evidence locator;
- security-sensitive flag;
- qualification state;
- reproduction/test state;
- resulting action/decision.

Never copy secrets, private Slack tokens, private email headers beyond what is necessary, or vulnerability details that belong in private disclosure.

## 9. Qualification state

Allowed qualification states:
- `QUALIFYING_EXTERNAL_EVIDENCE`;
- `TECHNICALLY_USEFUL_NONINDEPENDENT`;
- `ROUTING_ONLY`;
- `PARTICIPATION_ONLY`;
- `INSUFFICIENT_DETAIL`;
- `SECURITY_PRIVATE_PROCESS`;
- `DISQUALIFIED_SELF_EVIDENCE`.

Qualification is separate from whether the criticism/result is scientifically correct.

## 10. Scientific processing states

After qualification:
- `OPEN`;
- `REPRODUCED`;
- `NOT_REPRODUCED_TESTED_SCOPE`;
- `ACCEPTED`;
- `REJECTED_WITH_EVIDENCE`;
- `INCONCLUSIVE`;
- `NO_FIT`.

A qualifying external claim may ultimately be wrong and still remain valuable external evidence. A non-qualifying routing message may be accurate and still not count as validation.

## 11. Historical baseline classification

At A0 freeze:
- Greg Kroah-Hartman ptrace/LSM criticism (`EXT-0001`) = historical `ARCHITECTURE_CRITICISM`, accepted and already acted upon; retained as prior external evidence, not a substitute for current-state review;
- Linux Foundation/OpenSSF response = `ROUTING_GUIDANCE`;
- OpenSSF ORBIT Slack entry = `COMMUNITY_PARTICIPATION`;
- Rust Foundation response directing review to Rust users Code Review = `ROUTING_GUIDANCE`;
- AETHER X zero-assistance CI rehearsal = `DISQUALIFIED_SELF_EVIDENCE` for independence, though valid as pack-integrity evidence.

## 12. Evaluator tracks

### Track R — public zero-assistance reproduction
Evaluator uses only the public evaluator packet and public artifacts. Report PASS, failure, partial, unsupported environment, documentation friction and observed false PASS/REVIEW/incompleteness exactly.

### Track C — architecture criticism / counterexample
Evaluator may inspect the public product plus named research decisions. Questions focus on evidence authority, ptrace boundary, proposition completeness, baseline/policy separation, fail-closed semantics, and whether the intended layer is redundant/no-fit.

### Track I — standards/interoperability
Evaluator inspects the bounded P5 composition using in-toto Runtime Trace, SCAI, SVR, SLSA and GitHub/Sigstore identity. Request representation/interoperability criticism, not standards endorsement.

### Track W — external real workload
Evaluator runs ExecSurface on a workload not authored by AETHER X and records setup friction, verdict behavior, evidence completeness, privacy footprint, and unexpected behavior.

## 13. A0 acceptance

A0 may close only as:

**`P8_A0_EXTERNAL_VALIDATION_PACK_READY_BOUNDED`**

if:
- evaluator packet is frozen;
- intake template is frozen;
- exact public release identity/checksum can be reverified;
- packet explicitly welcomes negative/no-fit results;
- no internal execution is mislabeled external validation;
- #94 and #114 states remain consistent.

A0 does not require any external person to respond. P8-A1+ do.

## 14. Security boundary

Any plausible S4 vulnerability report is not reproduced publicly. Record only a safe private-process marker and follow `SECURITY.md`.

## 15. Promotion / claims boundary

No number of internal rehearsals, automated checks, emails sent, Slack posts or community memberships can become an external validation/adoption claim.

P8 can close only from qualified external evidence under later gates.
