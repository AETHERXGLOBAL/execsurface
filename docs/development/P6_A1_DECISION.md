# ExecSurface — P6-A1 Same-Workload Observation Decision

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Protocol: `docs/development/P6_A1_SAME_WORKLOAD_PROTOCOL.md`
Extraction: `docs/development/P6_A1_EVIDENCE_EXTRACTION_001.md`
Machine matrix: `experiments/p6-competitive-falsification/p6-a1-observation-matrix.json`

## Decision

**P6_A1_OBSERVATION_CORPUS_COMPLETE_BOUNDED**

This is a bounded factual-observation decision for the A1 lanes authorized by the preregistered protocol. It is not a product ranking, semantic-equivalence claim, performance claim, or release decision.

## Accepted executable evidence

Corrected same-workload run:
- source SHA: `110bf2ce400d50ac2cc2ae950de2de4e3a9cba12`;
- run: `36770813526`;
- conclusion: `success`;
- frozen workload blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`.

All authorized lanes completed S0–S3:
- ExecSurface alpha.4;
- Harden-Runner;
- cicd-sensor.

Tetragon was not an authorized live lane because the A0 exact-execution pin condition was not yet satisfied. It therefore remains `NOT_TESTABLE_YET` and is not counted as a negative comparative result.

## Retained historical negative evidence

The first A1 run remains immutable evidence:
- source: `bcf0da8e42648541b67a383db3c3c76ebc5b5790`;
- run: `36770551463`;
- ExecSurface artifact: `11123635852`;
- product evidence: S2 returned native `ERROR` because raw observation was incomplete;
- harness defect: the workflow incorrectly stopped on stable exit `2` before S3.

The correction did not change workload, comparator pins, propositions, thresholds, or acceptance vocabulary. It only retained exit `2` as native evidence and continued the corpus.

## Bounded findings

### ExecSurface public alpha.4
- S0: native `pass`;
- S1: native `review`, added process-exec drift;
- S2: native `ERROR/INCOMPLETE`, fail-closed before trusted canonical surface;
- S3: native `review`, file open/write drift observed.

### Harden-Runner
- exact `example.com:443` endpoint was correlated to `curl` in retained job log;
- process monitor and project file monitor initialization were evidenced;
- exact S1 child lineage was not emitted in the retained local job output;
- exact S3 `/tmp/execsurface-p6/s3.txt` write was not emitted; the retained log stated the project file monitor scope as the repository workspace;
- job/security-insights correlation was exposed, but no attestation-equivalent machine artifact was demonstrated by A1.

### cicd-sensor
- process execution + ancestry was retained for S1;
- process-bound domain/network evidence was retained for S2, including explicit TCP destination evidence for curl;
- exact S3 file write/read events were retained with process ancestry;
- a machine-consumable Runtime Trace predicate was produced with GitHub run/job/workflow identity.

Native cicd-sensor predicate `passed` is not mapped to ExecSurface `PASS`.

## Scientific boundary

A1 establishes only that the same frozen workload produced a reproducible factual corpus with meaningful differences in exposed evidence and semantics.

It does NOT establish:
- an overall stronger/weaker product;
- event-count superiority;
- cross-backend authority equivalence;
- that Harden-Runner cannot observe processes/files outside the retained local evidence;
- that cicd-sensor `network_connect` equals P4 success-authority semantics;
- that ExecSurface's S2 incompleteness is a competitor advantage/disadvantage;
- any performance result.

## Next gate

P6-A2 — adversarial false-PASS / identity-binding corpus.

A2 must target ExecSurface itself first. Required attack classes include:
- baseline/evidence replay under a different expected run/source identity;
- current-surface or evidence substitution;
- workflow/source identity substitution;
- observer incompleteness laundering into PASS;
- process/file/network drift that might be incorrectly accepted;
- CP4 predicate/attestation replay or unsigned-artifact substitution where applicable, reported only against each system's demonstrated verification contract.

No A1 result may be used to weaken A2 acceptance criteria.