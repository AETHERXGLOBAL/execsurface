# M9.2 — Independent Adoption Evidence Gate

Date: 2026-09-27
Status: **FROZEN BEFORE M9.2 DISCOVERY RESULTS**
Base main: `93158e044ff993037ce7da7a78e61fbb1b892d00`
Canonical evidence protocol: `docs/milestones/M9_PROTOCOL.md`
Canonical evidence schema: `docs/milestones/M9_EVIDENCE_SCHEMA.json`
Current M9.2 protocol: `docs/milestones/M9_2_INDEPENDENT_ADOPTION_PROTOCOL.md`
Supplementary evidence-level semantics: `docs/milestones/M9_EXTERNAL_ADOPTION_PROTOCOL.md`

## Objective

M9.2 answers one narrow question:

> Is there auditable evidence that third parties independently initiated and executed real ExecSurface workflows?

M9.1 zero-contact compatibility evidence is closed and must never be promoted into adoption by reinterpretation.

The older `M9_EXTERNAL_ADOPTION_PROTOCOL.md` retains useful L0/L1/L2/L3 evidence-level terminology, but its original milestone numbering is historical. The current program uses **M9.2 for independent-adoption evidence**.

## Team

### Fixed — Innovation Scientist / Adoption Architect

- search for the highest-information independent usage signals;
- prefer evidence that can be reproduced and audited;
- convert genuine external friction into later product work only after evidence is accepted.

### Fixed — Deviation Prevention / Scientific Integrity

- freeze M9.2 acceptance rules before discovery results;
- reject stars, downloads, views, comments, AETHER-authored PRs and AETHER-executed runs as adoption evidence;
- preserve absence of adoption as a valid negative result;
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
2. the independent validator accepts the record;
3. `independence_class == INDEPENDENT_USER`;
4. `provenance.initiated_by == THIRD_PARTY`;
5. `provenance.executed_by == THIRD_PARTY`;
6. `provenance.aether_x_material_involvement == false`;
7. `project.modified_by_aether_x == false`;
8. `provenance.third_party_attestation` is non-empty;
9. `external_reference` is non-empty and points to a public or consented external evidence location;
10. installation and the declared runtime workflow were actually attempted;
11. metadata-only privacy is confirmed and prohibited sensitive payload collection is false;
12. at least one content-addressed artifact is preserved;
13. failures and exclusions remain retained;
14. the external reference is manually reviewed for control/attribution before any L1/L2/L3 claim is published.

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
- `ACCEPTED_L1`, `ACCEPTED_L2`, or `ACCEPTED_L3` — only after validators and manual attribution review pass;
- `KILLED` — fails independence, provenance, privacy or workflow requirements.

## No-manufactured-adoption rule

AETHER X must not create an external PR, issue, synthetic account, delegated run or guided execution and then classify the resulting activity as independent adoption.

External outreach may exist as a separate commercial/developer-relations action, but any materially assisted execution is `COLLABORATIVE_EXTERNAL`, not `INDEPENDENT_USER`.

## M9.2 closure conditions

M9.2 has two non-equivalent closure states.

### Positive: `PROVED — INDEPENDENT ADOPTION EVIDENCE`

Requires all of:

1. at least two countable `INDEPENDENT_USER` records;
2. at least two distinct external project repositories;
3. at least two distinct attributable external references/initiators confirmed by manual provenance review;
4. both countable records show successful install, doctor, baseline, unchanged check, comparable=true, complete=true, and no unresolved operational error;
5. at least one countable record contains a comparable, complete drift case with `REVIEW` or `BLOCK`;
6. Red Team finds no material AETHER X execution/help or cherry-picking that invalidates independence;
7. all negative evidence and friction remain preserved.

This is the only closure state that permits a positive independent-adoption claim.

### Negative: `KILLED — NO ACCEPTED INDEPENDENT EVIDENCE IN DECLARED SEARCH SCOPE`

Permitted only after a discovery scope is preregistered and executed. It means the search stage is complete and M9.3 may synthesize the negative result.

It requires:

1. search scope/queries/sources frozen before interpreting results;
2. all candidates retained and classified;
3. zero-contact, collaborative, AETHER-authored, or AETHER-executed activity remains non-independent;
4. the exact number of accepted independent records is stated;
5. if zero qualify, the closeout says `0 accepted independent adoption records`;
6. no claim is made that nobody uses or wants ExecSurface outside the declared scope.

A negative close is valid evidence but is **not adoption** and must not be labeled `PROVED independent adoption`.

Either state may proceed to M9.3 evidence synthesis/product decision, but only the positive state supports an adoption claim.
