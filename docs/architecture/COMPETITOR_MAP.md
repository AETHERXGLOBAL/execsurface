# M0 — Adjacent Systems and Prior-Art Map

This is a positioning/prior-art map, **not a novelty claim**.

## strace

Primary role: system-call/process tracing.

ExecSurface may consume tracing evidence, but its intended product semantics are canonical baseline → deterministic candidate diff → independent policy → CI verdict.

https://strace.io/

## Falco

Primary role: runtime security detection against event streams and rules. Falco documents syscall events, process context and network-related events.

ExecSurface targets a different workflow: developer/PR-oriented comparison against an accepted execution-surface baseline.

https://falco.org/docs/
https://falco.org/docs/reference/rules/supported-events/

## Sandboxes / syscall enforcement

Primary role: isolate or restrict behavior.

ExecSurface MVP observes and compares. It does not claim enforcement.

## SLSA

SLSA is a software-supply-chain security specification. Its provenance describes verifiable information about how artifacts were produced.

ExecSurface runtime-effect evidence may complement provenance; it does not replace SLSA.

https://slsa.dev/spec/v1.2/
https://slsa.dev/spec/v1.2/provenance

## in-toto

in-toto protects software-supply-chain integrity through layouts and link metadata about steps/materials/products.

ExecSurface may later export evidence into an attestation workflow; it does not replace in-toto.

https://in-toto.io/docs/getting-started/

## Sigstore / Cosign

Sigstore/Cosign signs and verifies software artifacts and attestations.

ExecSurface creates runtime evidence; signing/verifying that evidence is a separate layer.

https://docs.sigstore.dev/quickstart/quickstart-cosign/
https://docs.sigstore.dev/cosign/verifying/attestation/

## GitHub SARIF / Code Scanning

GitHub can ingest third-party SARIF. ExecSurface may export suitable runtime findings, but must not invent a source-code location when the evidence is runtime-only.

https://docs.github.com/en/code-security/how-tos/find-and-fix-code-vulnerabilities/integrate-with-existing-tools/upload-sarif-file
https://docs.github.com/en/code-security/reference/code-scanning/sarif-files/sarif-support

## Positioning hypothesis

> PR-oriented execution-surface baseline + deterministic diff + policy-aware CI verdict.

This is a design target. It is not a claim of uniqueness, first-in-world status or patentability.

## Novelty gate

No novelty/patentability claim before a dedicated review of:
- academic literature,
- open-source repositories,
- commercial tooling,
- relevant patents,
- functional overlap and claim scope.

Status: **OPEN**.
