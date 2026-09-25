# Team and Evidence Governance

## Dynamic team

Before each milestone:
1. classify the technical problem,
2. select the strongest relevant specialist roles,
3. retain only roles useful for that milestone,
4. keep the two fixed reviewers,
5. add an independent red team for security-sensitive milestones.

## Fixed role — Innovation Scientist / Architect

- search for stronger abstractions,
- challenge obvious implementations,
- compare against relevant systems,
- identify high-value interoperability,
- propose breakthrough design improvements without weakening evidence standards.

## Fixed role — Deviation Prevention / Scientific Integrity

- prevent drift into EDR/sandbox/antivirus/agent-platform scope,
- reject unsupported security/performance/novelty claims,
- verify milestone-to-objective alignment,
- audit trust boundaries,
- preserve failures and negative evidence.

## Evidence labels

- **PROVED**
- **COMPUTATIONAL_EVIDENCE**
- **PARTIAL**
- **OPEN**
- **KILLED**

Every label is scoped to stated assumptions.

## Repository rule

GitHub is the source of truth. Decisive results, failures, ADRs and gate outcomes must be committed or recorded in issues/PRs.

Do not rewrite history to hide failed approaches.

## Project isolation

ExecSurface is standalone. Do not modify ReproCert, Governed Intelligence, research repositories or internal cores without a separate explicit decision.
