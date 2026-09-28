# M11.7 — Marketplace / Public Compatibility Regression Protocol

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Status: **PREREGISTERED — EXECUTION AUTHORIZED**

## Objective

Attempt to break the existing public user contract before any M11 release decision. M11 must not silently change the currently published GitHub Action, stable `@v0.1` channel, release-binary path, baseline-v2 interpretation, default backend, exit-code contract, or privilege requirements.

This gate is a compatibility test, not a promotion gate for the hybrid backend.

## Fixed roles

- **Innovative Systems Architect** — identify a path that preserves current self-service usability while retaining the stronger authority model.
- **Anti-Deviation / Skeptical Reviewer** — treat any silent public-surface drift, implicit privilege requirement, baseline reinterpretation, or auto-promotion of hybrid evidence as a blocker.

## Dynamic specialists

- GitHub Actions / Marketplace release engineering
- CLI and backward-compatibility review
- baseline serialization/digest compatibility
- Linux CI / privilege-boundary review
- external-consumer smoke testing
- Rust integration and regression testing

## Frozen public reference

At preregistration time:

- public `main`: `db11761e2d75ebca7d4458dc39094f4aed6a26bb`
- public release channel: `v0.1`
- current release pointer: `v0.1.0-alpha.3`
- supported public platform remains Linux x86_64 Public Alpha.

## Adversarial checks

### MP1 — Marketplace surface immutability

Compare the research branch against `origin/main` for:

- `action.yml`
- `action/install.sh`
- `action/run.sh`
- `action/release-tag.txt`

Any difference blocks this gate unless separately justified as an intentional public compatibility change. No such change is authorized in M11.7.

### MP2 — Current published binary path

Install the checksum-verified release referenced by `action/release-tag.txt` and prove:

- `--version` matches the release pointer;
- `doctor` succeeds on the supported runner;
- learn/check unchanged workload yields PASS;
- controlled drift yields REVIEW with stable exit code `10`.

### MP3 — Stable Marketplace action

Use `AETHERXGLOBAL/execsurface@v0.1` from a consumer-style workflow and prove:

- unchanged workload -> output verdict `pass`, exit code `0`;
- controlled drift -> output verdict `review`, exit code `10`;
- no hybrid/BPF setup is requested from the consumer.

### MP4 — Research-branch default remains portable

Build the M11 branch CLI with the normal workspace and run `doctor`, learn, and check on the normal GitHub-hosted runner without enabling BPF LSM, privileged containers, or a managed kernel. The current default user path must not require hybrid prerequisites.

### MP5 — Frozen baseline-v2

Run the frozen baseline-v2 digest/serialization regression. M11 authority metadata must not reinterpret existing baseline-v2 artifacts or make cross-backend reuse implicit.

### MP6 — Explicit hybrid only

Unit tests must continue to prove:

- hybrid activation requires explicit managed prerequisites;
- no silent ptrace fallback from an explicitly requested hybrid contract;
- no automatic baseline interchangeability;
- unsupported hybrid contexts fail explicitly rather than changing the portable public default.

## Acceptance criteria

1. Marketplace surface files are identical to `main`.
2. Current release binary self-service smoke passes.
3. Stable `@v0.1` PASS and controlled REVIEW behavior passes.
4. M11 branch default CLI works without BPF/privileged setup on the supported GitHub-hosted runner.
5. frozen baseline-v2 digest remains green.
6. hybrid remains explicit/managed/research-only.
7. full M11 authority/compatibility CI remains green.
8. `main`, public tags, and Marketplace publication are not mutated.

## Stop rule

Any breaking Action input/output change, exit-code change, baseline-v2 reinterpretation, new public privilege requirement, silent hybrid auto-selection, or failure of the current public consumer path blocks M11.8.

## Interpretation rule

Passing M11.7 proves compatibility only for the tested public paths and runner. It does not prove production readiness, universal Linux portability, or hybrid deployability on arbitrary CI hosts.
