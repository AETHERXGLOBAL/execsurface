# ExecSurface — Technical Evaluation Pack

**Purpose:** let an external engineer independently reproduce one unchanged-runtime PASS and one controlled runtime-drift REVIEW in roughly 5–10 minutes, with machine-readable evidence and without AETHER X assistance.

**Current public release:** `v0.1.0-alpha.5`

**Current boundary:** Linux x86_64 Public Alpha. Native `ptrace` is the public default/reference observer. This evaluation is not a malware test, sandbox, EDR assessment, enforcement-system test, or proof of program safety.

## What this evaluation demonstrates

The evaluator will:

1. install the exact published alpha.4 binary and verify its checksum;
2. verify environment readiness;
3. learn an explicit tiny baseline;
4. re-run the same command and obtain `PASS`;
5. change runtime behavior and obtain `REVIEW`;
6. preserve JSON and Markdown verdict reports for inspection.

A failure, unsupported environment, incomplete observation or unexpected result is valid evaluation evidence. Do not weaken host security settings, policy, or baseline to manufacture PASS.

## 1. Install the exact public evaluation release

No Rust toolchain is required for the primary evaluation path.

```bash
VERSION=v0.1.0-alpha.5
TARGET=x86_64-unknown-linux-gnu
ASSET="execsurface-${VERSION}-${TARGET}.tar.gz"

curl -fLO "https://github.com/AETHERXGLOBAL/execsurface/releases/download/${VERSION}/${ASSET}"
curl -fLO "https://github.com/AETHERXGLOBAL/execsurface/releases/download/${VERSION}/${ASSET}.sha256"
sha256sum -c "${ASSET}.sha256"
tar -xzf "${ASSET}"

mkdir -p "$HOME/.local/bin"
install -m 0755 "execsurface-${VERSION}-${TARGET}/execsurface" "$HOME/.local/bin/execsurface"
export PATH="$HOME/.local/bin:$PATH"

execsurface --version
execsurface doctor
```

Expected version:

```text
execsurface 0.1.0-alpha.5
```

Optional build-provenance verification when GitHub CLI attestation support is available:

```bash
gh attestation verify "$ASSET" -R AETHERXGLOBAL/execsurface
```

A valid build attestation binds an artifact to a build source/workflow. It does not establish that the artifact is safe.

### Rust-native alternative

If Rust/Cargo is already installed:

```bash
cargo install execsurface --version "=0.1.0-alpha.5" --locked
execsurface --version
execsurface doctor
```

If either public installation path fails, retain that failure rather than substituting an internal build.

## 2. Create an isolated evaluation workspace

```bash
rm -rf /tmp/execsurface-technical-evaluation
mkdir -p /tmp/execsurface-technical-evaluation
cd /tmp/execsurface-technical-evaluation
```

## 3. Learn the accepted baseline

Use the same `/bin/bash -lc` wrapper used by the public GitHub Action:

```bash
execsurface learn -- /bin/bash -lc 'true'
```

Expected artifact:

```text
execsurface.lock.json
```

Retain the printed baseline digest.

## 4. Prove the unchanged path

```bash
execsurface check \
  --json-output pass-report.json \
  --markdown-output pass-report.md \
  -- /bin/bash -lc 'true'
```

Expected verdict:

```text
ExecSurface: PASS
```

Expected process exit status: `0`.

If observation is incomplete, `ERROR`/non-PASS is the correct conservative outcome and should be reported.

## 5. Introduce deterministic controlled drift

```bash
set +e
execsurface check \
  --json-output drift-report.json \
  --markdown-output drift-report.md \
  -- /bin/bash -lc 'true; /bin/echo controlled-drift >/dev/null'
status=$?
set -e
printf 'ExecSurface exit status: %s\n' "$status"
```

With the built-in unmatched-drift review policy, the expected result in the supported evaluation environment is:

```text
ExecSurface: REVIEW
ExecSurface exit status: 10
```

`REVIEW` means the observed execution surface differs from the accepted baseline under the declared policy. It does not mean the command is malicious.

## 6. Evidence to retain

Preserve:

```text
execsurface.lock.json
pass-report.json
pass-report.md
drift-report.json
drift-report.md
```

Also capture:

```bash
execsurface --version
uname -a
sha256sum execsurface.lock.json pass-report.json drift-report.json
```

Record whether the evidence reports observation as complete or incomplete and retain any warning/error text.

## 7. What to inspect

Inspect at least:

- installed version identity;
- declared command identity;
- baseline digest;
- observer identity/capabilities/limitations;
- added / removed / changed observed effects;
- matched policy rules or default action;
- target exit status;
- observation completeness/health;
- usability friction and runtime overhead noticeable to the evaluator.

## 8. Current authority and limitation notes

The public alpha.4 reference backend is native `ptrace`.

Important boundaries:

- pathname copied at syscall entry is pathname access-attempt metadata, not kernel-object identity;
- known pathname-pointer TOCTOU remains an architectural limitation;
- shared-FD concurrency is conservatively fail-closed when attribution cannot be justified; this prevents the known false-completeness class but can create false incompleteness;
- this is not an exact shared-FD attribution repair;
- incomplete evidence cannot silently become PASS;
- BPF-LSM/kernel-hook work remains research/managed and non-default;
- no automatic hybrid selection or ptrace↔hybrid baseline interchangeability is authorized;
- Linux x86_64 is the current public support scope.

See `docs/architecture/PTRACE_VS_LSM_ARCHITECTURE_REVIEW.md` and `docs/releases/v0.1.0-alpha.5.md`.

## 9. What a successful evaluation establishes

A successful run supports only a bounded statement such as:

> The evaluator installed the exact public ExecSurface alpha.4 release, learned an accepted runtime surface, reproduced an unchanged PASS, introduced controlled runtime drift, and obtained a machine-readable REVIEW under the declared policy in the recorded environment.

It does **not** establish:

- that the changed command is malicious;
- that the unchanged command is safe;
- that all possible runtime behavior was observed;
- universal Linux/container compatibility;
- production readiness;
- third-party adoption;
- endorsement of AETHER X.

`RUNTIME DRIFT ≠ MALICIOUSNESS`

`NO OBSERVED DRIFT ≠ PROGRAM IS SAFE`

`OBSERVED BEHAVIOR ≠ ALL POSSIBLE BEHAVIOR`

`SELF-EVALUATION PASS ≠ INDEPENDENT ADOPTION`

## 10. Independent external evidence

If you run this evaluation outside AETHER X and are willing to share the result, preserve the environment, commands, verdict reports, failures/friction and enough reproduction detail for another developer to repeat it.

Independent external evidence is counted only when it originates from or is confirmed by an external evaluator. Stars, impressions, private praise and AETHER X self-tests do not count.

For the shortest controlled demo path, see [`QUICKSTART_5_MIN.md`](./QUICKSTART_5_MIN.md).
