# M9.2 — Independent Adoption Evidence Gate

Date: 2026-09-27
Status: **FROZEN BEFORE M9.2 DISCOVERY RESULTS**
Base main: `93158e044ff993037ce7da7a78e61fbb1b892d00`
Canonical evidence protocol: `docs/milestones/M9_PROTOCOL.md`
Canonical evidence schema: `docs/milestones/M9_EVIDENCE_SCHEMA.json`
Supplementary L0/L1/L2/L3 definitions: `docs/milestones/M9_EXTERNAL_ADOPTION_PROTOCOL.md`

## Objective

M9.2 answers one narrow question:

> Is there auditable evidence that a third party independently initiated and executed a real ExecSurface workflow?

M9.1 zero-contact compatibility evidence is closed and must never be promoted into adoption by reinterpretation.

## Team

### Fixed — Innovation Scientist / Adoption Architect

- search for the highest-information independent usage signals;
- prefer evidence that can be reproduced and audited;
- convert genuine external friction into later product work only after evidence is accepted.

### Fixed — Deviation Prevention / Scientific Integrity

- freeze M9.2 acceptance rules before discovery results;
- reject stars, downloads, views, comments, AETHER-authored PRs and AETHER-executed runs as adoption evidence;
- preserve absence of adoption as a valid result;
- reject retroactive relabeling of M9.1 evidence.

### Independent Red Team

Attempt to make non-independent evidence pass by using ambiguous attribution, AETHER-authored integrations, copied logs, empty attestations, public interest signals, or incomplete workflows.

### Dynamic specialists

- OSS adoption/repository evidence;
- CI/workflow provenance;
- developer experience;
- evidence/privacy review;
- release/distribution analytics when attributable evidence is available.

## Evidence-level terminology

The L1/L2/L3 labels below are evidence levels, not milestone numbers.

- **L1 — External reproduction:** a third party independently runs the prescribed workflow and returns auditable results.
- **L2 — Adoption trial:** a third party intentionally installs/integrates ExecSurface in a branch, PR, CI experiment or sustained evaluation.
- **L3 — Adoption:** a third party merges, retains or repeatedly uses ExecSurface after the initial trial.

No level is inferred from popularity or interest.

## Mandatory acceptance conditions

An M9.2 candidate can enter the accepted independent-evidence ledger only when all of these hold:

1. the canonical M9 record validates under `scripts/m9_validate_evidence.py`;
2. `independence_class == INDEPENDENT_USER`;
3. `provenance.initiated_by == THIRD_PARTY`;
4. `provenance.executed_by == THIRD_PARTY`;
5. `provenance.aether_x_material_involvement == false`;
6. `project.modified_by_aether_x == false`;
7. `provenance.third_party_attestation` is non-empty;
8. `external_reference` is non-empty and points to a public or consented external evidence location;
9. installation was actually attempted (`outcome.installation != NOT_RUN`);
10. baseline creation was attempted (`workflow.baseline_attempted == true` and `outcome.baseline != NOT_RUN`);
11. unchanged rerun/check was attempted (`workflow.rerun_check_attempted == true` and `outcome.rerun_check != NOT_RUN`);
12. metadata-only privacy is confirmed and prohibited sensitive payload collection is false;
13. at least one content-addressed artifact is preserved;
14. failures and exclusions remain retained;
15. the external reference is manually reviewed for control/attribution before any L1/L2/L3 claim is published.

A drift case is valuable but not mandatory for L1 acceptance. It becomes mandatory only for claims specifically about drift detection in the external workflow.

## Explicit non-evidence

None of the following, by itself, counts as L1/L2/L3:

- GitHub stars, forks, watchers or traffic;
- crates.io downloads;
- release downloads;
- README views;
- social-media reactions;
- an issue comment expressing interest;
- a maintainer reply without an independent execution;
- an AETHER X-authored PR in an external repository;
- an AETHER X-run reproduction of an external repository;
- a copied log whose executor cannot be attributed;
- collaborative execution where AETHER X materially operated the run.

These may be discovery signals only.

## Discovery rule

M9.2 discovery may search public repositories, issues, PRs, CI files and other public references for independent use. Discovery itself never upgrades evidence.

Every discovered candidate is one of:

- `DISCOVERY_ONLY` — interesting signal, not accepted evidence;
- `CANDIDATE_INDEPENDENT` — potentially qualifying, awaiting provenance/workflow review;
- `ACCEPTED_L1`, `ACCEPTED_L2`, or `ACCEPTED_L3` — only after both validators and manual attribution review pass;
- `KILLED` — fails independence, provenance, privacy or workflow requirements.

## No-manufactured-adoption rule

AETHER X must not create an external PR, issue, synthetic account, delegated run or guided execution and then classify the resulting activity as independent adoption.

External outreach may exist as a separate commercial/developer-relations action, but any materially assisted execution is `COLLABORATIVE_EXTERNAL`, not `INDEPENDENT_USER`.

## M9.2 closure conditions

M9.2 closes in one of two valid states:

### `PROVED — INDEPENDENT EVIDENCE EXISTS`

At least one accepted L1/L2/L3 record passes the frozen gate with auditable external provenance.

### `PROVED — NO ACCEPTED INDEPENDENT EVIDENCE FOUND IN DECLARED SEARCH SCOPE`

The declared discovery scope is executed and preserved, but no candidate passes the frozen gate. This is a valid negative product/adoption result and must not be softened by relabeling M9.1 compatibility evidence.

Either state may proceed to M9.3 evidence synthesis/product decision.
