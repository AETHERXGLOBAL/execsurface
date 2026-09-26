# ExecSurface Troubleshooting

Start with:

```bash
execsurface doctor
```

The doctor is diagnostic only. It never changes privileges, ptrace sysctls or host security configuration.

## Unsupported OS

**Symptom**

`doctor` reports a non-Linux OS.

**Action**

Use a Linux x86_64 environment. The public alpha does not claim macOS or Windows support.

Do not treat a compatibility layer as an officially supported observer unless it has separate evidence.

## Unsupported architecture

**Symptom**

`doctor` reports an architecture other than `x86_64`.

**Action**

Use a Linux x86_64 runner/host. No arm64 release is claimed in the public alpha.

## ptrace restrictions

**Symptom**

`doctor` reports that the ptrace observer is unavailable or cannot produce complete evidence.

**Action**

1. Confirm you are running the command directly on Linux x86_64.
2. Confirm the environment allows a process to trace the child it launches.
3. If this is a container, review the container/runtime security policy.
4. If an organization policy intentionally blocks ptrace, use an approved runner rather than weakening the policy blindly.

ExecSurface does not automatically run as root, add capabilities, or change `kernel.yama.ptrace_scope`.

## Observer ERROR

**Symptom**

ExecSurface exits 2 or prints `ExecSurface: ERROR`.

**Action**

Read the emitted observer/comparability error first. ERROR is deliberately distinct from PASS.

Common causes include:

- incomplete observation;
- unreadable required metadata;
- unsupported platform/backend;
- baseline mismatch;
- invalid policy.

Do not convert ERROR into PASS in CI.

## Baseline schema mismatch

**Symptom**

A lockfile is rejected because its schema/digest format is not current.

**Action**

Do not hand-edit the lockfile version.

Relearn the baseline with the current binary:

```bash
execsurface learn -- /bin/bash -lc 'YOUR COMMAND'
```

Then review the new evidence before replacing the accepted baseline.

## v1 → v2 baseline migration

M6.5 changed execution-surface meaning by adding actual fd-attributed read/write effects and causal execution chains.

v1 lockfiles are intentionally **not** reinterpreted as v2.

Relearn with the current public alpha and review the new surface.

See [M6.5 Schema Migration](milestones/M6_5_SCHEMA_MIGRATION.md).

## Wrapper mismatch

**Symptom**

A local baseline and GitHub Action run are incomparable or unexpectedly different.

**Cause**

The public Action executes:

```text
/bin/bash -lc <command>
```

**Action**

Learn locally with the identical wrapper:

```bash
execsurface learn -- /bin/bash -lc 'YOUR COMMAND'
```

`execsurface init --command "YOUR COMMAND" --github-actions` prints the exact matching learn/check commands.

## Policy parse errors

**Symptom**

ExecSurface exits 2 before producing a policy verdict.

**Action**

- verify valid JSON;
- verify `schema_version`;
- remove unknown fields;
- use policy v2 for `file_read` / `file_write` matchers.

Do not replace an invalid policy with an implicit allow-all rule.

## Unexpected REVIEW

**Meaning**

REVIEW means observed drift exists and the applicable policy requires review. It is not a malware verdict.

**Action**

1. Re-run only if the workload is expected to be deterministic.
2. Inspect added/removed/changed findings.
3. Identify whether the change is expected.
4. Update policy only if the behavior is intentionally allowed.
5. Relearn a baseline only when the accepted execution surface itself has intentionally changed.

Do not relearn automatically just to make REVIEW disappear.

## Evidence incomplete or truncated

**Symptom**

The observer reports incomplete evidence, an event-budget warning, or canonicalization refuses the observation.

**Action**

Treat the run as ERROR.

If the workload legitimately exceeds the supported observation envelope, preserve the evidence and report the case. Do not weaken fail-closed behavior.

## CI differs from local

Common reasons:

- different executable paths or toolchain;
- different home/workspace/temp roots;
- shell startup behavior;
- different dependency/cache state;
- different Linux/runtime policy;
- different observer capabilities.

Use the same command wrapper and keep declared normalization roots deliberate.

Do not add broad wildcard normalization merely to force local/CI equality.

## Release checksum mismatch

**Action**

Do not run the binary.

Delete the downloaded files and download the release again from the official repository.

Verify:

```bash
sha256sum -c execsurface-v0.1.0-alpha.2-x86_64-unknown-linux-gnu.tar.gz.sha256
```

Optionally verify GitHub build provenance:

```bash
gh attestation verify execsurface-v0.1.0-alpha.2-x86_64-unknown-linux-gnu.tar.gz \
  -R AETHERXGLOBAL/execsurface
```

An attestation proves provenance, not safety.

## GitHub Action cannot download the release

The stable Action consumes the immutable public-alpha release asset.

Check:

- outbound access to `github.com`;
- `curl`, `tar` and `sha256sum` exist on the runner;
- the runner is Linux x86_64.

For repository development only, `uses: ./` uses a source build.

## Still blocked?

Read [SUPPORT.md](../SUPPORT.md) and open a focused issue without secrets, tokens, private file contents or confidential paths.
