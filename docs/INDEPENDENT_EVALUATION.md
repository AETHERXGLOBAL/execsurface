# Independent Evaluation — ExecSurface

This guide is for developers, maintainers, security engineers, CI owners, and researchers who want to evaluate ExecSurface independently, without contacting AETHER X first.

Current public version: `v0.1.0-alpha.6`

Supported public environment: Linux x86_64

Public default/reference observer: native `ptrace`

## What ExecSurface does

ExecSurface learns an accepted runtime execution surface for a command and later reports observed runtime drift, including selected process, file and network effects under the recorded observer and explicit policy.

It does not prove software safety and it is not antivirus, EDR, malware detection, a sandbox, or a general Linux enforcement framework.

## 1. Install

### Recommended exact public-release path

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
```

### crates.io alternative

```bash
cargo install execsurface --version "=0.1.0-alpha.6" --locked
execsurface --version
```

For independent evaluation, do not silently fall back to an internal source build if a public installation path fails. Record the failure.

## 2. Check your environment

```bash
execsurface doctor
```

Do not change host security settings just to make the tool pass. If `doctor` reports an unsupported environment, preserve that result as part of the evaluation.

## 3. Choose a real command

Use a command that is meaningful in your own project, for example:

```bash
cargo test --locked
```

or

```bash
npm test
```

or another repeatable build, test, lint, packaging, developer-tool, or CI command.

For the cleanest first evaluation, prefer a command whose ordinary runtime behavior is reasonably stable across two consecutive runs.

## 4. Create starter files

```bash
execsurface init --command "cargo test --locked" --github-actions
```

Review the generated files before use. `init` does not execute your command and does not automatically accept a baseline.

## 5. Learn a baseline

To match the public GitHub Action execution wrapper:

```bash
execsurface learn -- /bin/bash -lc 'cargo test --locked'
```

Review `execsurface.lock.json` before committing it.

## 6. Re-run unchanged

```bash
execsurface check \
  --policy execsurface-policy.json \
  -- /bin/bash -lc 'cargo test --locked'
```

Record the verdict and observation completeness. If evidence is incomplete, a non-PASS outcome is correct and should not be bypassed.

## 7. Introduce one controlled runtime change

Change the command or fixture in a deliberate, reviewable way so one additional runtime effect occurs. For example, invoke a known extra executable in a temporary evaluation branch.

Run `execsurface check` again and verify that the additional observed behavior is surfaced rather than silently accepted.

Do not weaken the policy or edit the baseline merely to obtain PASS.

## 8. Optional GitHub Actions use

```yaml
permissions:
  contents: read

steps:
  - uses: actions/checkout@v4

  - name: ExecSurface
    uses: AETHERXGLOBAL/execsurface@v0.1
    with:
      command: cargo test --locked
      baseline: execsurface.lock.json
      policy: execsurface-policy.json
```

For immutable evaluation of this release, pin `AETHERXGLOBAL/execsurface@v0.1.0-alpha.5`.

Pin other third-party Actions to immutable SHAs in security-sensitive repositories according to your own supply-chain policy.

## Current limitations that should be challenged

External evaluators are explicitly invited to test and criticize these boundaries:

- ptrace syscall-entry pathname metadata is not kernel-object identity and is subject to userspace pointer TOCTOU;
- shared-FD / clone concurrency can be conservatively marked incomplete rather than claiming exact attribution;
- conservative false incompleteness is possible;
- memory-mapped and io_uring behavior are not claimed as fully covered by the current fd-attribution model;
- support is Linux x86_64 public alpha, not universal Linux/container support;
- BPF-LSM/kernel-hook work remains research/managed and non-default;
- incomplete evidence cannot silently become PASS.

## What to report

An independent report is useful whether the result is positive or negative. Please include:

- project or repository tested;
- operating system, kernel and architecture;
- ExecSurface version and installation path;
- command evaluated;
- whether `execsurface doctor` passed;
- baseline-learn result;
- first unchanged-check verdict and finding count;
- second unchanged-check verdict and finding count where practical;
- controlled-drift result;
- completeness/health status;
- any false positive, false negative, false incompleteness, usability problem, crash, unexpected overhead, permission friction, or undocumented assumption;
- enough reproduction detail for another developer to repeat the result.

Do not include secrets, credentials, private source code, environment values, file contents, stdin, or network payloads.

## Submit your independent result

Open a GitHub issue in this repository using the **Independent Evaluation** issue template. You do not need prior approval or contact with AETHER X.

A report is considered independent evidence only when the external evaluator independently initiates and executes the evaluation. AETHER X-run tests, even against public third-party projects, are not independent adoption.

Negative evidence is welcome and will not be relabeled as success.
