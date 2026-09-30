# ExecSurface — P4-C External Trace Import Protocol

Date: 2026-09-30
Parent program: #100
Parent P4: #107
Predecessor: `P4_B_PTRACE_SUCCESS_AUTHORITY_ESTABLISHED_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — SCHEMA-LEVEL IMPORT ONLY**

## Scientific question

Can a mature external runtime trace source be imported into the frozen ExecSurface proposition/authority contract without laundering producer semantics, backend identity, event presence, or unknown completeness into stronger verification authority?

P4-C is interoperability research only. It is **not** collector adoption, backend promotion, a Tetragon dependency, a product integration, or a claim of backend equivalence.

## Selected external source

Selected source: **Cilium Tetragon JSON event schema**.

Pinned external source identity:

- repository: `cilium/tetragon`
- source commit: `666efe6f91e3605ad58683ad226d759d9cf970ca`
- `api/v1/tetragon/events.proto` blob: `d6bd56769241da983f7e0042a54d5b3a81f40817`
- `api/v1/tetragon/tetragon.proto` is the process/event payload schema at the same source commit
- license: Apache-2.0

Selection rationale:

- Tetragon exposes structured events through JSON and gRPC;
- the public protobuf schema distinguishes `PROCESS_EXEC` and defines process execution identity/metadata;
- process flags explicitly distinguish `execve`-origin events from `procFS` bootstrap events and identify truncation/error states;
- the source is used only as an external trace format. No Tetragon runtime component is added to ExecSurface.

## Fixed roles

1. **Innovation Scientist / Evidence Interoperability Architect** — seek the smallest import boundary that preserves provenance and leaves authority proposition-scoped.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks collector adoption, security-product drift, backend-name authority, implicit completeness, public-v2 changes, and unsupported equivalence claims.
3. **Independent Falsifier / Red Team** — attacks producer substitution, schema substitution, `procFS` laundering, truncation/error laundering, causal mismatch, raw-event substitution, event-type confusion, completeness inflation, and deterministic-identity failure.
4. **Independent Critical-Milestone Reviewer** — verifies exact external schema pin, ExecSurface source SHA, static/test evidence, artifact digest, predecessor reproofs, immutable tags, and retained failures.

Dynamic specialists: protobuf/JSON schema interoperability, process/exec semantics, formal evidence semantics, deterministic serialization, provenance binding, Rust, CI reproducibility, software licensing.

## Frozen scope

Only one proposition is eligible for a positive mapping in this experiment:

`P4.EXEC.SUCCESS`

The experiment consumes a minimal Tetragon JSON `process_exec` event and produces a research-only imported evidence record.

No file, network, FD, rename/delete, process-create, observer-health, or causal-lineage proposition is promoted from this experiment. Those remain unsupported unless separately proven.

## Import contract

Every imported record must preserve:

- external producer identity fixed by the importer, not trusted from event-controlled text;
- external repository/source commit;
- schema path/blob identity;
- raw canonical event SHA-256;
- proposition ID;
- subject/executable identity at the declared resolution level;
- authority state;
- completeness state;
- explicit reason codes for ambiguity/incompleteness;
- deterministic record identity.

The raw external evidence digest and imported-record digest are distinct identities.

### Candidate positive mapping

A Tetragon `process_exec` event may establish bounded **event-level direct evidence** for `P4.EXEC.SUCCESS` only if all of the following are true:

1. the pinned schema identity exactly matches this protocol;
2. `process_exec.process.exec_id` is non-empty;
3. `process_exec.process.binary` is an absolute non-empty path;
4. process flags contain `execve`;
5. process flags do **not** contain `procFS`;
6. process flags do not contain `truncFilename`, `errorFilename`, `unknown`, or `miss`;
7. `process_exec.process.parent_exec_id` is non-empty;
8. `process_exec.parent.exec_id` equals `process.parent_exec_id`;
9. `process_exec.parent.binary` is a non-empty absolute path;
10. the event is not represented as an aggregated `process_exec` response.

Even when all ten conditions hold, this experiment does **not** possess independent proof that the complete external observation session had no loss, export filtering, rate limiting, field filtering, restart gap, or producer-side drop. Therefore the imported proposition MUST be:

- authority: `direct`
- completeness: `incomplete`
- mandatory reason: `external_session_completeness_unproven`

It MUST NOT satisfy a proof requirement that requires complete session/lifecycle/transport evidence.

This distinction is deliberate: direct event evidence is not the same thing as complete observation of the runtime scope.

## Explicit fail-closed mappings

- `procFS` without `execve` -> non-success authority for `P4.EXEC.SUCCESS`; never an observed transition.
- `procFS` plus `execve` -> ambiguous/incomplete; contradictory origin semantics are not normalized away.
- truncation/error/unknown/miss flags -> ambiguous/incomplete.
- missing/mismatched parent execution identity -> ambiguous/incomplete.
- non-absolute or empty executable identity -> ambiguous/incomplete.
- absent `process_exec` / another event type -> unsupported/not-applicable for this importer.
- wrong/unpinned external schema identity -> import rejection before proposition construction.
- event-controlled producer/backend/profile strings -> ignored for authority; importer uses a fixed profile identity.
- field/order differences that are JSON-semantically identical -> identical canonical raw-event digest.
- any external session-completeness claim not represented by a separately proven contract -> ignored for this gate.

## Frozen executable falsification corpus

Exactly **14 tests** are preregistered. Acceptance is **14/14 PASS**, zero ignored/filtered.

1. qualifying `execve` event maps to `P4.EXEC.SUCCESS` with `direct + incomplete` only;
2. valid imported event is not admissible to a proof requirement requiring complete session scope;
3. `procFS` bootstrap event cannot become exec-success authority;
4. contradictory `execve procFS` flags fail closed;
5. `truncFilename` fails closed;
6. `errorFilename`, `unknown`, and `miss` each fail closed;
7. parent exec-ID mismatch fails closed;
8. missing causal parent identity fails closed;
9. empty/relative executable identity fails closed;
10. non-`process_exec` event is unsupported/not-applicable;
11. unpinned schema source/blob identity is rejected;
12. producer/backend-name substitution cannot upgrade authority or completeness;
13. JSON object-key reordering preserves canonical raw-event digest and deterministic imported-record identity;
14. raw evidence mutation changes raw-event digest and imported-record identity.

## Mandatory same-source reproof

The accepted P4-C source must also re-prove:

- P4-B cross-proposition corpus: **12/12 PASS**
- backend-name anti-inflation: **1/1 PASS**
- B1 live: **10/10 PASS**
- B2 live: **10/10 PASS**
- B3 live: **10/10 PASS**
- Semantics v3: **7/7 PASS**
- public M11: **6/6 PASS**
- `v0.1.0-alpha.4` and `v0.1` remain at `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- no changes under `crates/` after the accepted P4-B source.

## Kill criteria

Any of the following prevents a positive P4-C decision:

- backend/producer name can upgrade weak evidence;
- `procFS` can masquerade as a new exec transition;
- malformed/truncated/error-marked evidence can become direct+complete;
- causal parent substitution preserves positive authority;
- external schema substitution is accepted silently;
- event presence implies session completeness;
- unsupported external event type is interpreted as absence/success;
- identical semantic JSON produces unstable canonical evidence identity;
- mutated raw evidence preserves the same imported-record identity.

Counterexamples are retained. Thresholds and positive conditions are not weakened after execution.

## Allowed decisions

- `P4_C_EXTERNAL_IMPORT_PATH_ESTABLISHED_BOUNDED`
- `P4_C_IMPORT_SEMANTIC_LAUNDERING_FOUND`
- `P4_C_EXTERNAL_SCHEMA_INSUFFICIENT`
- `P4_C_INCOMPLETE_EVIDENCE`

A positive result means only that a pinned external trace can be mapped through a fail-closed research adapter while preserving ExecSurface semantics and provenance boundaries. It does not authorize Tetragon as a runtime dependency, public integration, backend equivalence, release promotion, or complete imported-runtime verification.