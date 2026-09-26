# M7.1 — Registry Distribution Parity Evidence

Status: **CLOSED / ACCEPTED**

Release: `v0.1.0-alpha.2`

Release source commit:

`c6cabf1b4d1399898b9643a7fce011664aac0918`

## Objective

Establish a Rust-native, zero-contact installation path for ExecSurface without changing the M6.5/M7 execution-surface semantics.

Target user command:

```bash
cargo install execsurface --locked
```

## Published crates

Version `0.1.0-alpha.2` was published for:

1. `execsurface-model`
2. `execsurface-observe`
3. `execsurface-normalize`
4. `execsurface-baseline`
5. `execsurface-diff`
6. `execsurface-policy`
7. `execsurface-report`
8. `execsurface`

`execsurface-bench` remains intentionally `publish = false`.

## Preserved failures

### F1 — account email not verified

The first authenticated publication reached the crates.io upload step but was rejected because the account email was not yet verified.

Result: **BLOCKED / ACCOUNT CONFIGURATION**

No package was accepted during that failed attempt.

### F2 — crates.io new-crate rate limiting

After multiple new crate names were accepted, crates.io enforced its new-crate rate limit and supplied a future retry boundary.

Result: **BLOCKED / REGISTRY RATE LIMIT**

This was not treated as a package, source, token, or release-identity failure.

## Recovery design

The recovery workflow:

- checked out immutable `v0.1.0-alpha.2` source;
- verified the release commit identity;
- detected versions already present in crates.io;
- skipped previously published packages;
- resumed the dependency chain safely;
- completed the remaining publications;
- did not rewrite or move the release tag.

## Final zero-contact registry proof

A fresh Ubuntu 24.04 runner executed:

```bash
cargo install execsurface --version "=0.1.0-alpha.2" --locked
```

Cargo confirmed installation of:

```text
execsurface v0.1.0-alpha.2
```

The installed binary then passed:

```text
execsurface --version
execsurface doctor
execsurface learn -- /bin/bash -lc true
execsurface check -- /bin/bash -lc true
```

Observed doctor result:

```text
[PASS] Linux
[PASS] x86_64
[PASS] ptrace observer available
[PASS] workspace writable
[PASS] ExecSurface 0.1.0-alpha.2
Ready.
```

Observed no-drift result:

```text
ExecSurface: PASS
findings: total=0 allow=0 review=0 block=0
```

## Gate result

- crates.io package availability: **PASS**
- dependency-chain publication: **PASS**
- immutable release identity: **PASS**
- fresh registry installation: **PASS**
- `doctor`: **PASS**
- `learn`: **PASS**
- no-drift `check`: **PASS**
- runtime/evidence semantics changed: **NO**

M7.1 is therefore **CLOSED / ACCEPTED**.
