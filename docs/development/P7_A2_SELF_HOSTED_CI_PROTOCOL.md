# ExecSurface — P7-A2 Self-Hosted CI Packaging & Evidence Contract

Date: 2026-09-30
Parent program: #100
P7 issue: #112
Predecessor: `P7_A1_GITLAB_CONTEXT_ADAPTER_PASS_BOUNDED`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — NO A2 RESULT YET**

## 1. Question

Can ExecSurface be packaged for a self-hosted Linux x86_64 CI runner without allowing runner ownership, labels, environment variables, or same-job learning to alter the frozen alpha.4 behavioral semantics, baseline identity, verdict, or observer authority?

This is a research packaging/evidence-contract gate. It is not a public self-hosted support or release-promotion gate.

## 2. Hypothesis

A minimal self-hosted CI package is semantics-preserving if it:

1. installs only the exact public alpha.4 x86_64 release asset after pinned SHA256 verification;
2. refuses non-Linux/non-x86_64 execution;
3. accepts an approved baseline only through an explicit checked configuration, never by CI environment auto-selection;
4. validates the baseline bytes against the configured SHA256 before execution;
5. invokes `execsurface check` without rewriting its exit code or verdict;
6. records host/CI context only as `execution_environment_only` metadata;
7. never upgrades behavioral authority because the machine is self-hosted, privileged, trusted, owned, labeled, or long-lived;
8. treats ptrace/runtime failure as fail-closed execution failure, never PASS.

## 3. Fixed roles

1. **Innovation Scientist / Systems Architect** — minimize packaging-specific semantics and preserve portability of the evidence contract.
2. **Anti-Drift / Scientific Integrity Reviewer** — blocks self-hosted trust inflation, baseline auto-approval, platform overclaim and exit-code laundering.
3. **Independent Falsifier / Red Team** — attacks package checksum, architecture, baseline substitution, env steering, runner-label trust, verdict override and incomplete execution.
4. **Independent Critical-Milestone Reviewer** — verifies exact release identity, frozen corpus, retained failures and promotion wording.

Dynamic specialists:
- CI packaging / release distribution;
- Linux x86_64 ptrace/runtime;
- supply-chain artifact verification;
- baseline/evidence integrity;
- shell/Python reproducibility.

## 4. Immutable public boundary

A2 MUST NOT:

- modify alpha.4 source, release assets, tags, stable `@v0.1`, public v2 semantics, default observer, or public crates;
- support arm64 by bypassing A0's negative result;
- infer authority from `self-hosted`, runner labels, machine ownership, root/admin status, or CI provider;
- learn and approve a baseline inside the same verification job;
- select a baseline from untrusted CI environment variables;
- replace ExecSurface exit codes with a CI-specific PASS;
- reinterpret ERROR/INCOMPLETE as success;
- delete negative evidence.

## 5. Frozen release identity

Release: `v0.1.0-alpha.4`
Source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Asset: `execsurface-v0.1.0-alpha.4-x86_64-unknown-linux-gnu.tar.gz`
Asset SHA256: `55776cd130784d1c03451a14ab05ed113ffae4fa714c986666cbcadf242d8d1d`

The package contract must reject any asset digest mismatch before extraction/execution.

## 6. Approved baseline contract

The verification configuration contains:

- `baseline_path` — relative path under the workspace only;
- `baseline_sha256` — exact 64-hex digest of the approved baseline bytes;
- `approval_origin` — fixed value `external_preapproved`;
- `package_version` — fixed `v0.1.0-alpha.4`;
- `package_sha256` — the frozen release-asset digest.

The runner environment cannot override any of these fields.

A harness may create a baseline fixture solely to test the packaging contract, but that fixture MUST be created before invoking the package wrapper and its digest must then be frozen into the harness configuration. This is test setup, not an authorization pattern for production CI.

## 7. Environment context

The wrapper may record:

- OS;
- machine architecture;
- kernel release;
- CI provider label if supplied;
- runner name/labels if supplied.

These fields are descriptive only and MUST be emitted with:

- authority: `execution_environment_only`;
- `may_select_baseline=false`;
- `may_change_verdict=false`;
- `may_upgrade_observer_authority=false`.

Unknown environment variables are ignored.

## 8. Frozen falsification corpus

A2 must execute exactly these tests:

1. valid configuration maps successfully;
2. canonical configuration serialization is deterministic;
3. wrong package version fails closed;
4. wrong package SHA256 fails closed;
5. absolute baseline path fails closed;
6. path traversal baseline path fails closed;
7. malformed baseline SHA256 fails closed;
8. non-`external_preapproved` origin fails closed;
9. unsupported OS fails closed;
10. unsupported architecture fails closed;
11. CI env cannot replace baseline path;
12. CI env cannot replace baseline digest;
13. CI env cannot override verdict;
14. self-hosted/trusted/root labels cannot upgrade authority;
15. missing baseline file fails closed;
16. substituted baseline bytes fail digest verification;
17. matching baseline bytes pass digest verification;
18. wrapper preserves native ExecSurface exit code unchanged.

Acceptance threshold: **18/18**.

## 9. Executable packaging probe

After the model corpus passes, the workflow must on Ubuntu 24.04 x86_64:

1. verify immutable tags;
2. download the exact alpha.4 release archive;
3. verify the frozen SHA256 before extraction;
4. verify binary version `execsurface 0.1.0-alpha.4`;
5. create one harness-only S0 baseline fixture using the exact binary;
6. freeze the fixture baseline SHA256 into a generated A2 test configuration;
7. invoke the A2 wrapper against S0-S3 using that explicit config;
8. require native result vector `0 / 10 / 2 / 10` for the frozen P6 workload;
9. require zero S1-S3 false PASS;
10. retain package/baseline/config/report/context digests.

The harness baseline is evidence setup only and is not a product recommendation to learn/approve inside the same pipeline.

## 10. Decision classes

- `P7_A2_SELF_HOSTED_PACKAGE_CONTRACT_PASS_BOUNDED` — 18/18 plus executable packaging probe and immutable-boundary checks pass;
- `P7_A2_SELF_HOSTED_PACKAGE_CONTRACT_FAIL` — a surviving packaging/evidence-integrity flaw remains;
- `P7_A2_INCOMPLETE` — infrastructure/harness prevents scientific execution.

## 11. Promotion boundary

A positive A2 result proves a bounded packaging/evidence contract under the tested x86_64 Linux environment only. It does not establish every self-hosted runner configuration, public support, hardened-host compatibility, privilege portability, or zero-assistance deployment.
