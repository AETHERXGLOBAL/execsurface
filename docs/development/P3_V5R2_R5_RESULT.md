# ExecSurface — P3 V5-R2 R5 Independent False-PASS / Falsification Result

Date: 2026-09-29
Tracking: #105
Parent: #103 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — P3_R5_FALSIFICATION_PASS_BOUNDED**

## Governance

Fixed roles retained:
- Innovation Scientist / Systems Architect
- Anti-Drift / Scientific Integrity Reviewer
- Independent Falsifier / Red Team
- Independent Milestone Reviewer

Dynamic R5 specialists:
- supply-chain attack modeling
- canonical effect identity / provenance semantics
- Rust adversarial testing
- build-system nondeterminism
- CI artifact integrity
- Linux observer completeness
- normalization collision analysis

## Frozen evidence boundary

R3-C: `P3_V5_REAL_WORKLOAD_VALUE_REQUALIFIED_BOUNDED`.
R4: `P3_R4_CHECK_SET_COMPLETE_SENTINEL_PROTECTED`.

Immutable inputs verified:
- R2-C learning artifact `11035133723`
- R2-C artifact SHA-256 `sha256:a98a1f022e601f0b0653474df8c4500f33fa5670f7e25f105740b2e79bd2fd26`
- R4 artifact `11037682900`
- R4 artifact SHA-256 `sha256:99f0acef1213a94f08ea6ff5d5908da927098a80505cbc1f4fec1eb74ee4afd1`
- raw learning-set digest `sha256:437a9f8fca9f0ff4c8573c1cfee06b46cb325d1d1a890649ba536acd8e2e648d`
- projected learning-set digest `sha256:16e8519c83ea374359d1e44872d570ccc15be88b7ccf8f9e3b54b5e3d6b7ab30`
- explicit accepted-variable set: **empty**

## Preserved engineering failures

### Attempt 1
- workflow `36577891839`
- source `0304236a2a89cf3e61ee3ca25839a326c5ac37dc`
- artifact `11037309410`
- artifact SHA-256 `sha256:0e81e74c176f09d8e9b56792e945393907fe12e46785a8004dda87d3171a465d`

Artifact integrity, corruption rejection, R4 replay, sentinel replay and the original V4 10/10 falsifier all passed. The job stopped at Rust 1.90 formatting of the new R5 test source before current-boundary tests. Classification: `FORMATTING_ONLY / PRE_R5_CURRENT_BOUNDARY_TESTS`.

### Attempt 2
- workflow `36578320811`
- source `2112fba19bcf24bf0aa49a62fdaa313f055eb4f1`
- artifact `11036964856`
- artifact SHA-256 `sha256:adc311c7377f7109d7da12ec391939fbdafc207ec3a5e6d2ab05385de1614918`

The same frozen preliminary gates passed. One remaining rustfmt-only representation difference stopped the job before current-boundary tests. Classification remains `FORMATTING_ONLY / PRE_R5_CURRENT_BOUNDARY_TESTS`.

Neither attempt altered an attack, assertion, artifact reference, acceptance rule, threshold, workload, projection grammar or expected outcome.

## Accepted execution

Source:
`ed5ca6bc07273ade4864eba821f0dc55e10f76a8`

Workflow:
`36578564586`

Job:
`109440426994`

Evidence artifact:
`11038625288`

Artifact SHA-256:
`sha256:fe76d24e44ec6324f8aeceecd67e04f8798ee0670722b9c3828b1a5717487c2c`

## Falsification evidence

### Provenance / artifact integrity

- exact R2-C learning artifact digest: PASS
- exact R4 artifact digest: PASS
- intentionally corrupted local R4 archive rejected: PASS
- marker: `R5_PROVENANCE_CORRUPTION_REJECTED`

### Frozen R4 replay

- six-check admission remained `6/6`
- GCC eligible findings remained `[4,4,4,4,4,4]`
- explicit accepted-variable matches remained all zero
- no multi-run explicit-variance residual improvement was invented
- R4 sentinel replay produced `R5_SENTINEL_REPLAY_UNSEEN_MATCHES=2`
- marker: `R5_R4_FALSE_PASS_INVARIANTS_PASS`

### Original poisoning / variance falsifier replay

The previous V4 adversarial suite replayed **10/10 PASS**:
1. single-run malicious injection not authorized
2. repeated malicious injection not auto-authorized
3. duplicate evidence rejected
4. incomplete evidence rejected
5. run-order permutation byte-stable
6. profile/observer/normalization mismatch rejected
7. GCC grammar mimicry rejected outside bounded role
8. actor substitution cannot inherit exact acceptance
9. unseen-similar target cannot inherit exact acceptance
10. invariant laundering rejected

### Current-boundary R5 tests

New R5 adversarial suite: **4/4 PASS**:
1. causal-chain substitution cannot inherit exact acceptance
2. bounded GCC projection preserves non-target effects exactly
3. wrong-actor GCC mimic is neither eligible nor projected
4. recurrent variable behavior remains unauthorized under the frozen empty selection

No false authorization or non-target collapse was observed in the declared R5 matrix.

### Public-path anti-drift reproof

Historical M11 shared-FD public contract reran after R5:
- **6/6 PASS**

The public/default v2 path therefore remains legacy conservative/fail-closed in the declared regression suite.

## Decision

`P3_R5_FALSIFICATION_PASS_BOUNDED`

This is bounded falsification evidence, not a universal security, poisoning-resistance, production-readiness or observer-completeness claim.

R5 supports the following narrow statements only:
- the frozen GCC ephemeral projection survived the declared false-PASS attack matrix;
- non-target effects remained visible in the declared projection attacks;
- recurrence still grants no authorization;
- the meaningful-drift sentinel remained unseen/unaccepted;
- immutable artifact corruption was rejected;
- the public alpha.4/v2 fail-closed contract remained unchanged.

## Product-decision implication

R3-C + R4 + R5 establish value for the **bounded evidence-backed GCC ephemeral identity projection** in the pinned FZF/Go environment.

They do **not** establish additional product value for automatic multi-run variance authorization: the explicit accepted-variable set remained empty, and R4 observed no check where the explicit-variance layer reduced residual findings below the bounded ephemeral-only projection.

Therefore the next authorized step is a P3 product decision constrained to the already-declared #105 outcomes. No public integration or release is authorized by R5 itself.
