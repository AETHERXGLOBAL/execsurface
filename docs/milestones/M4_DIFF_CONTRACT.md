# M4 — Diff Engine Contract

Date: 2026-09-25

Status: **IMPLEMENTATION GATE OPEN**

Issue: #8

## Purpose

M4 compares a verified M3 baseline against a fresh M2 canonical surface.

It emits deterministic evidence only:

- added effects;
- removed effects;
- conservatively paired changed effects.

M4 assigns no severity and no PASS / REVIEW / BLOCK verdict.

## Comparability gate

The comparison is rejected if these contracts differ:

- platform OS;
- platform architecture;
- observer backend name;
- observer capability set;
- canonical-surface schema version;
- normalization profile version;
- semantic-root label set;
- canonical command executable;
- command argument count.

Observer limitation prose is retained as evidence but is not used as a hard equality key.

## Exact diff

Exact baseline-only effects are removed.

Exact candidate-only effects are added.

All effects are compared as deterministic ordered sets.

## Conservative changed pairing

A changed finding requires a semantic subject.

Supported M4 v1 subjects:

- file access: actor + operation + canonical target path value;
- file rename: actor + canonical source path value;
- IPv4/IPv6 connect: actor + address family + remote IP;
- Unix connect: actor + canonical socket path value.

For one subject, changed is emitted only when exactly one unmatched baseline effect and exactly one unmatched candidate effect remain.

If either side has multiple unmatched effects for one subject, M4 refuses to guess the pairing and leaves them as added/removed.

Process exec/spawn differences remain added/removed in M4 v1.

## Exit semantics

M4 check:

- exit 0: comparison was computed, including when drift exists;
- exit 2: baseline, observation, normalization, comparability or serialization error.

Policy-aware exit behavior belongs to M5.

## Privacy

The report contains canonical metadata only.

It does not contain target argv values or environment values.

## Non-goals

No:

- severity;
- allowlist;
- policy;
- PASS / REVIEW / BLOCK;
- SARIF;
- GitHub Action.
