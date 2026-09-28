# OpenSSF Zero-Assistance Self-Service Rehearsal

Tracking: #94
Date: 2026-09-28
Classification: `PASS — PUBLIC_ARTIFACT_ZERO_ASSISTANCE_REHEARSAL`

## Intent

Rehearse the evaluator path as if the evaluator had only public documentation and the published alpha.4 artifact.

No repository source build, private knowledge, internal binary or policy weakening was allowed.

## Environment

GitHub-hosted clean runner:

- Ubuntu 24.04.5 LTS
- Linux kernel `6.17.0-1022-azure`
- x86_64

Workflow: `OpenSSF zero-assistance self-service rehearsal`
Run: `36478741625`
Job: `109118876854`
Result: **success**

Evidence artifact:

- ID: `10994536766`
- name: `openssf-zero-assistance-alpha4-36478741625`
- digest: `sha256:9ea245278a2a43f182cda04cca6f5133ca88443138feca2949e887a94304e5c3`

## Public install path

Downloaded only:

`v0.1.0-alpha.4 / x86_64-unknown-linux-gnu`

Checksum validation:

`execsurface-v0.1.0-alpha.4-x86_64-unknown-linux-gnu.tar.gz: OK`

Version:

`execsurface 0.1.0-alpha.4`

## Doctor

All declared first-run readiness checks passed:

- Linux: PASS
- x86_64: PASS
- ptrace observer available: PASS
- workspace writable: PASS
- version alpha.4: PASS

Doctor output retained the product boundary:

`observed behavior is not all possible behavior`

## Learn

Command:

`/bin/bash -lc 'true'`

Result:

- baseline learned successfully;
- digest: `sha256:f1a58f6617735f8146256e9982cf15695c39cb7fa1b338f391bbce4583f8f667`;
- canonical effects: `486`.

## Unchanged check

Same command returned:

- `ExecSurface: PASS`
- exit status `0`
- findings `0`.

## Controlled drift

Command:

`/bin/bash -lc 'true; /bin/echo controlled-drift >/dev/null'`

Result:

- `ExecSurface: REVIEW`
- exit status `10`
- findings `34` in the tested host, including added process/file effects.

The exact finding count is environment/evidence dependent and is **not** part of the portable contract; the required contract for this controlled case is visible drift and REVIEW under the built-in review behavior.

## Friction / undocumented assumptions

No blocking undocumented assumption was observed in this rehearsal.

One CI-platform maintenance warning appeared after evidence upload: GitHub currently forces `actions/upload-artifact@v4` from Node.js 20 to Node.js 24. This warning is in the evidence-preservation harness, not ExecSurface runtime/install behavior, and did not affect the evaluation result.

## Decision

The public alpha.4 binary path is reproducible in the tested clean hosted-CI environment without AETHER X implementation assistance.

This is AETHER X internal rehearsal evidence, **not independent external validation**.

External evaluators must still be allowed to fail independently; an external failure must be preserved before any assistance is offered.

OpenSSF pack readiness can proceed to claims review / public merge.
