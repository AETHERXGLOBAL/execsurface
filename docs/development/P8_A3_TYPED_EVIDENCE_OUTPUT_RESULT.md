# P8-A3.3 — Experimental Same-Run Typed Evidence Output

Date: 2026-10-04  
Tracking: #153  
Design authority: #151 / PR #152  
Prototype authority: #149 / PR #150  
Implementation PR: #154  
Status: **E7 CLOSEOUT CANDIDATE**

## Decision candidate

`EXPERIMENTAL_TYPED_EVIDENCE_OUTPUT_IMPLEMENTATION_ACCEPTED`

This decision is bounded to source/main integration after final-head rechecks. It does not authorize a release, stable-v1 promise, GitHub Action exposure, or P8-A5 closeout.

## Experimental interface

```bash
execsurface observe --evidence-output evidence.json -- COMMAND [ARGS...]
```

Without `--evidence-output`, the legacy code path remains:

`observe_command -> raw Observation -> pretty JSON stdout`

The existing raw schema-v2 stdout is not replaced.

## E0 — Compatibility freeze

Accepted no-flag gate verifies the raw object remains schema v2 with the existing top-level shape:

- `schema_version`;
- `backend`;
- `complete`;
- `outcome`;
- `events`;
- `warnings`.

No typed-report fields appear in the legacy raw object.

Accepted result:

`E0_LEGACY_OBSERVE_SHAPE_PRESERVED`

## E1 — Same-run envelope

The implementation exposes a workspace-internal `observe_command_with_backend` path that returns the existing internal `BackendObservation`.

The experimental CLI path invokes that observer exactly once.

The evidence envelope embeds:

`raw_observation = observed.observation`

and derives typed health/effects from the same in-memory object.

The dedicated gate parses:
1. stdout raw JSON from the experimental invocation;
2. the evidence file;
3. requires exact JSON-object equality:

`evidence.raw_observation == stdout_raw_observation`.

No second target execution or caller-supplied binding identity is used.

Accepted result:

`E1_SAME_RUN_RAW_TYPED_BINDING_PASS`

## E2 — Typed semantics

Experimental schema:

- evidence schema version: `1`;
- report kind: `typed_observation_evidence`;
- stability: `experimental`.

The accepted implementation exposes:
- recognized backend/profile identity;
- platform / architecture / privacy profile;
- typed collection-health state;
- warning codes;
- unsupported capabilities;
- bounded fd-write effects;
- explicit limitations and non-claims;
- the same-run raw observation.

A retained fd-write effect may claim:
- `derived_runtime_fd_state`;
- `syscall_result_post_operation`;
- `runtime_fd_path_correlated`;
- `successful_operation_result`;
- `state_machine_correlated`.

It does not claim exact transferred bytes or content.

Accepted result:

`E2_TYPED_SUCCESSFUL_EFFECT_SEMANTICS_PASS_BOUNDED`

## E3 — Fail-closed and misuse attacks

Accepted tests cover:
- collection-health laundering;
- backend-name/profile inflation;
- missing fd-effect capability with retained fd-write;
- process/TID binding mismatch;
- repeated writes remain distinct;
- forbidden claim fields are absent;
- duplicate `--evidence-output` rejected;
- evidence output rejected for the experimental libbpf route;
- evidence output-path failure returns explicit CLI error;
- malformed option use fails before creating a false evidence file.

The envelope contains no verdict field.

Accepted result:

`E3_NON_INFLATION_AND_CLI_FAILURE_MATRIX_PASS`

## E4 — Privacy

A high-entropy sentinel supplied only as target argv is absent from the evidence file.

The implementation adds no collection path and projects only the already-produced `BackendObservation`.

Full existing observer/privacy regressions also remain required through CI/adversarial regression.

Accepted result:

`E4_METADATA_ONLY_PRIVACY_PRESERVED`

## E5 — Public compatibility and regression

Accepted pre-closeout head:

`e0c8b127c3f968abf135c76115fc4bb85317a625`

Accepted runs:
- experimental typed evidence gate `37197490327`: **SUCCESS**
- full CI `37197490331`: **SUCCESS**
- P9.3 Compatibility Contract `37197490330`: **SUCCESS**
- Adversarial Regression `37197490376`: **SUCCESS**
  - Ubuntu 22.04: PASS
  - Ubuntu 24.04: PASS
- prior typed-report prototype regression `37197490326`: **SUCCESS**

Existing GitHub Action contract remained green under P9.3.

Accepted result:

`E5_COMPATIBILITY_AND_ADVERSARIAL_REGRESSION_PASS`

## E6 — Probity-style bounded consumer replay

The dedicated gate demonstrates that one evidence file supplies:
- original raw schema-v2 trace;
- typed collection health;
- successful fd-write effect semantics.

It also verifies explicit non-claims covering:
- exact byte count;
- file contents;
- before/after state roots;
- custody;
- trusted time;
- signature-as-behavioral-authority;
- causal source-code provenance.

Therefore the first-party report can remove semantic reconstruction work identified by the external Probity integration while leaving Probity's separate state/authority/signing/publication-policy layer outside ExecSurface.

Accepted result:

`E6_PROBITY_STYLE_CONSUMER_REPLAY_PASS_BOUNDED`

## Evidence artifact

Accepted gate run:

`37197490327`

Artifact:
`11301670198`

Artifact digest:

`sha256:88348f33b0599899cae676af72c602c0197e13b759f295b0cabbd760fd782aee`

## Retained negative evidence

Initial implementation heads failed before scientific execution because `cargo fmt --check` rejected source formatting.

Material retained runs include:
- typed evidence run `37197432751`: static formatting failure before E0-E6;
- CI `37197432711`: formatting failure;
- prototype regression `37197432718`: formatting failure;
- intermediate typed evidence run `37197489206` and corresponding CI/prototype failures while only part of the rustfmt-equivalent correction had been applied.

P9.3 and adversarial regression remained green on those intermediate heads.

Corrections were formatting-only. No evidence claim, test assertion, threshold, health rule, authority rule, privacy boundary, or compatibility condition was weakened.

## E7 — Bounded decision

Subject to successful final-head rechecks after this documentation-only closeout:

`EXPERIMENTAL_TYPED_EVIDENCE_OUTPUT_IMPLEMENTATION_ACCEPTED`

Meaning:
- the experimental source interface may live on `main`;
- it remains non-stable and unreleased;
- no GitHub Action surface is added;
- no stable-v1 promise is created;
- no release/tag/promotion is authorized.

## P8 boundary

P8 #114 remains OPEN.

Internal implementation success cannot substitute for the missing current A1/A4 external execution/use record.
