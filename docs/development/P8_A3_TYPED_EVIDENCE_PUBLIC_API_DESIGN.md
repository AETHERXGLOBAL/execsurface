# P8-A3.2 — Bounded Public Typed-Evidence API Design

Date: 2026-10-04  
Tracking: #151  
Parent prototype: #149 / PR #150  
External source: `P8-EXT-0004` / `probityai/agent-evidence-observer#41`  
Status: **DESIGN DECISION — NO PUBLIC IMPLEMENTATION AUTHORIZED**

## Decision

`OBSERVE_OPT_IN_TYPED_FILE_SELECTED`

Candidate public surface:

```bash
execsurface observe --evidence-output evidence.json -- COMMAND [ARGS...]
```

The existing raw `observe` stdout contract remains unchanged.

The new file is an explicitly versioned typed observation-evidence report generated from the **same internal observation run**.

This is a design selection only. The option does not exist publicly until a later implementation gate passes and is separately authorized.

## D1 — Semantic separation

ExecSurface has three distinct evidence objects and they must remain distinct.

### Raw observation

Current lower-level object:

`execsurface observe -- COMMAND`

Purpose:
- retain backend/raw event evidence;
- preserve current schema-v2 observation semantics.

It is not a policy verdict and must not silently become the typed report.

### Typed observation evidence

Candidate opt-in object:

`--evidence-output PATH`

Purpose:
- expose typed collection health;
- expose bounded proposition/guarantee semantics already known at the internal `BackendObservation` boundary;
- preserve explicit limitations and unsupported capabilities.

It is not a baseline, policy result, vulnerability verdict, attestation/custody claim, or replacement for raw evidence.

### Verdict report

Existing object:

`execsurface check --json-output PATH`

Purpose:
- compare against an approved baseline;
- apply policy;
- emit PASS / REVIEW / BLOCK / ERROR.

Its schema and meaning remain unchanged.

Decision:

`RAW_OBSERVATION != TYPED_OBSERVATION_EVIDENCE != VERDICT_REPORT`

## D2 — UX / misuse analysis

### Complete health is not a security verdict

A typed evidence state such as `complete` means the required collection boundary was established for the declared evidence profile.

It must not be described as:
- safe;
- vulnerability-free;
- policy compliant;
- trustworthy in a universal sense.

Candidate documentation language:

> Collection health describes evidence acquisition under the declared profile. It is not a security or policy verdict.

### Successful write effect is bounded

A successful fd-write proposition means the selected observer retained evidence corresponding to successful positive-byte I/O under its declared semantics.

It does not imply:
- exact byte count;
- file contents;
- final filesystem state;
- absence of intermediate writes;
- source-code causality.

### Backend identity is not authority

A backend/profile name may select a semantic mapping only after the implementation/profile is recognized and validated.

The report must never infer stronger authority from a prestigious or user-supplied backend name.

### Typed evidence is not `check`

The report must contain no PASS / REVIEW / BLOCK verdict field.

If a consumer needs policy evaluation, it must use the existing `check` path or a separately authorized composition.

## D3 — Compatibility analysis

### Candidate surface A — mutate `observe` stdout

**KILLED.**

Reason:
- silently changes an existing interface;
- breaks consumers expecting raw schema v2;
- conflates raw and interpreted evidence;
- would create unnecessary compatibility risk.

Decision:

`MUTATE_OBSERVE_STDOUT_KILLED_COMPATIBILITY`

### Candidate surface C — separate `evidence` command

**KILLED for the current need.**

Reason:
- Probity-class consumers need both the retained raw trace and typed semantics for the same execution;
- a separate command would naturally execute the workload again;
- two executions cannot be assumed behaviorally identical;
- joining the second typed run to the first raw run would recreate the binding problem the bridge is intended to remove.

A future offline converter over a cryptographically/semantically bound retained observation is a separate research question and is not selected here.

Decision:

`SEPARATE_EVIDENCE_COMMAND_KILLED_SAME_RUN_BINDING`

### Candidate surface B — opt-in file on `observe`

**SELECTED.**

Properties:
- existing invocation without the flag behaves exactly as before;
- current stdout stays raw observation output;
- typed report is opt-in;
- raw and typed views derive from one internal observer execution;
- the typed file can use a separately versioned schema;
- no baseline or policy is required.

Decision:

`OBSERVE_OPT_IN_TYPED_FILE_SELECTED`

## D4 — Transport and privacy

Structured typed evidence must be written directly to a dedicated file.

It must not rely on stdout redirection.

Reason:
the observed target inherits process stdout/stderr. M6 already established that target output can corrupt structured JSON transported through stdout.

The selected option therefore follows the same evidence-file principle already accepted for `check --json-output`.

### Existing stdout

For compatibility, `observe` stdout remains unchanged in this design gate.

This does not newly certify stdout as a robust structured transport. Consumers requiring reliable typed evidence use the dedicated file.

### Acquisition boundary

The typed report may only project information already available from the selected internal `BackendObservation`.

No new target-memory or content reads are authorized.

## D5 — Same-run binding requirement

This is a hard implementation requirement.

The typed report and raw observation must be demonstrably derived from the **same observer execution**.

The later implementation gate must choose and prove one bounded binding mechanism, such as:

1. an internal deterministic digest of the exact raw observation object carried in the typed report; or
2. another explicit run/evidence identity that cannot be supplied independently by an untrusted caller.

The implementation gate must reject designs that:
- run the target twice;
- accept a user-supplied raw digest as proof of binding;
- allow a typed report from run B to be represented as describing raw observation from run A.

No binding mechanism is frozen by this design document; it must be executable and falsifiable before public integration.

## D6 — Candidate typed-report contract

The later implementation gate may expose only a minimal separately versioned schema.

Required semantic classes:

- report schema/version identity;
- selected backend semantic profile;
- platform / architecture / privacy profile;
- typed collection-health state;
- warning / ambiguity reason codes;
- unsupported capabilities;
- bounded typed effects with proposition-scoped guarantees;
- explicit limitations and non-claims;
- same-run binding to the raw observation.

Explicitly prohibited:

- PASS / REVIEW / BLOCK as evidence-report verdicts;
- file contents;
- exact transferred byte counts not independently retained;
- before/after state roots;
- custody;
- trusted time;
- signatures treated as behavioral authority;
- causal source-code provenance;
- automatic baseline or policy mutation.

The exact field names are **not stabilized by this gate**.

## GitHub Action decision

No Action input/output is added by this design gate.

Reason:
the external interoperability need is first established at the low-level observation/evidence boundary. Adding Action surface area before the CLI/evidence contract is executable and externally usable would create a second compatibility obligation prematurely.

Potential Action exposure is deferred to a later gate after:
- public CLI contract qualification;
- same-run binding proof;
- external consumer trial.

## External-consumer fit — Probity mapping

The selected interface can remove first-party semantic reconstruction work from a Probity-class integration.

It can directly provide:
- typed collection health instead of `unknown-no-typed-envelope`;
- explicit successful-operation semantics for retained fd-write effects;
- backend/profile and unsupported-capability semantics;
- a same-run link to raw evidence, once the implementation binding gate passes.

It intentionally does **not** replace Probity's:
- before/after content snapshots;
- state-root construction;
- authority plan;
- signing;
- commitment;
- custody model;
- publication policy;
- higher-level in-toto/ObservedEffect composition.

That separation is desirable: ExecSurface should expose truthful runtime evidence, not absorb the consumer's separate state/custody protocol.

## Final design decision

`P8_A3_2_OBSERVE_OPT_IN_TYPED_FILE_SELECTED_BOUNDED`

The next legitimate step is a **separate experimental implementation gate** that must:

1. implement the selected option behind an explicitly experimental/non-stable contract;
2. preserve legacy `observe` behavior byte/semantically for invocations without the flag;
3. prove same-run raw/typed binding;
4. run privacy sentinels;
5. run authority/non-inflation adversarial tests;
6. pass full CI, P9.3 compatibility and adversarial regression;
7. include at least one consumer-style replay modeled on the concrete Probity integration.

No public release or stable-v1 promise is authorized by this design decision.

## P8 boundary

P8 #114 remains OPEN.

A successful design or internal implementation does not substitute for the missing current A1/A4 external execution/use record.
