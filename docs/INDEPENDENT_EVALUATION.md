# Independent Evaluation — ExecSurface

This guide is for developers, maintainers, security engineers, CI owners, and researchers who want to evaluate ExecSurface independently, without contacting AETHER X first.

Current public version: `v0.1.0-alpha.3`

Supported public environment: Linux x86_64

Public correctness-reference backend: native `ptrace`

## What ExecSurface does

ExecSurface learns an accepted runtime execution surface for a command and later reports observed runtime drift, including selected process, file, and network effects.

It does not prove software safety and it is not an antivirus, EDR, malware detector, or sandbox.

## 1. Install

### crates.io

```bash
cargo install execsurface --locked
execsurface --version
```

### GitHub Release

You can also use the checksum-verified Linux x86_64 binary from the current GitHub Release.

## 2. Check your environment

```bash
execsurface doctor
```

Do not change host security settings just to make the tool pass. If `doctor` reports an unsupported environment, record that result as part of the evaluation.

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

Review the generated files before use.

## 5. Learn a baseline

To match the public GitHub Action execution wrapper:

```bash
execsurface learn -- \
  /bin/bash -lc 'cargo test --locked'
```

Review `execsurface.lock.json` before committing it.

## 6. Re-run unchanged

```bash
execsurface check \
  --policy execsurface-policy.json \
  -- /bin/bash -lc 'cargo test --locked'
```

Record the verdict and whether the evidence was complete.

## 7. Introduce one controlled runtime change

Change the command or fixture in a deliberate, reviewable way so one additional runtime effect occurs. For example, invoke a known extra executable in a temporary evaluation branch.

Run `execsurface check` again and verify that the additional behavior is surfaced rather than silently accepted.

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

Pin third-party Actions to immutable SHAs in security-sensitive repositories according to your own supply-chain policy.

## What to report

An independent report is useful whether the result is positive or negative. Please include:

- project or repository tested;
- operating system and architecture;
- ExecSurface version;
- command evaluated;
- whether `execsurface doctor` passed;
- baseline-learn result;
- first unchanged-check verdict and finding count;
- second unchanged-check verdict and finding count;
- controlled-drift result;
- completeness status;
- any false positive, false negative, usability problem, crash, or unexpected overhead;
- enough reproduction detail for another developer to repeat the result.

Do not include secrets, credentials, private source code, environment values, file contents, stdin, or network payloads.

## Submit your independent result

Open a GitHub issue in this repository using the **Independent Evaluation** issue template. You do not need prior approval or contact with AETHER X.

A report is considered `INDEPENDENT_USER` evidence only when the external evaluator independently initiates and executes the evaluation. AETHER X-run tests, even against public third-party projects, remain zero-contact external reproduction and are not counted as independent adoption.

Negative evidence is welcome and will not be relabeled as success.
