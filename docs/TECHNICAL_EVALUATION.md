# ExecSurface — Technical Evaluation Pack v1

**Purpose:** let an external engineer independently reproduce one unchanged-runtime PASS and one controlled runtime-drift REVIEW in roughly 5–10 minutes, with machine-readable evidence.

**Current boundary:** Linux x86_64 Public Alpha. This evaluation is not a malware test, sandbox, EDR assessment, or proof of program safety.

## What this evaluation demonstrates

The evaluator will:

1. install the exact published crates.io evaluation release;
2. verify environment readiness;
3. learn an explicit tiny baseline;
4. re-run the same command and obtain `PASS`;
5. change runtime behavior while keeping the same shell wrapper shape and obtain `REVIEW`;
6. preserve JSON and Markdown verdict reports for inspection.

## 1. Install the exact public evaluation release

For a reproducible evaluation, pin the same public-alpha version whose zero-contact registry path is recorded in M7.1 evidence:

```bash
cargo install execsurface --version "=0.1.0-alpha.2" --locked
execsurface --version
execsurface doctor
```

Expected version:

```text
execsurface 0.1.0-alpha.2
```

The repository README may show the shorter normal-user install command. The evaluator path intentionally pins the version so that a later registry release cannot silently change the artifact under evaluation.

`doctor` is diagnostic only. It does not elevate privileges or weaken host security settings.

## 2. Create an isolated evaluation workspace

```bash
mkdir -p /tmp/execsurface-technical-evaluation
cd /tmp/execsurface-technical-evaluation
```

## 3. Learn the accepted baseline

Use the same `/bin/bash -lc` wrapper used by the GitHub Action:

```bash
execsurface learn -- \
  /bin/bash -lc 'true'
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

## 5. Introduce deterministic controlled drift

The executable and argument count remain comparable, but the shell now performs an additional execution effect:

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

With the built-in unmatched-drift review policy, the expected verdict is:

```text
ExecSurface: REVIEW
```

Expected process exit status: `10`.

`REVIEW` means the observed execution surface differs from the accepted baseline and requires review under the selected policy. It does not mean the command is malicious.

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

The JSON verdict reports are the primary machine-readable evaluation evidence. The Markdown files are human-readable summaries of the same bounded verdict surface.

## 7. What to inspect

An evaluator should inspect at least:

- the installed version identity;
- the baseline digest;
- the declared command identity;
- added / removed / changed effects;
- matched policy rules or default action;
- target exit status;
- observer identity, capabilities and limitations represented in the evidence surface.

## 8. What a successful evaluation establishes

A successful run supports only a bounded statement such as:

> The evaluator installed the exact public ExecSurface evaluation release, learned an accepted runtime surface, reproduced an unchanged PASS, introduced controlled runtime drift, and obtained a machine-readable REVIEW under the declared policy.

It does **not** establish:

- that the changed command is malicious;
- that the unchanged command is safe;
- that all possible runtime behavior was observed;
- production readiness;
- third-party adoption;
- endorsement of AETHER X.

`RUNTIME DRIFT ≠ MALICIOUSNESS`

`NO OBSERVED DRIFT ≠ PROGRAM IS SAFE`

`OBSERVED BEHAVIOR ≠ ALL POSSIBLE BEHAVIOR`

`SELF-EVALUATION PASS ≠ INDEPENDENT ADOPTION`

## 9. Independent external evidence

If you run this evaluation outside AETHER X and are willing to make the result public, preserve a link to the repository/CI run and the resulting evidence artifacts.

Independent adoption evidence is counted only when it originates from or is confirmed by an external project or evaluator. Stars, impressions, private praise and AETHER X self-tests do not count.

For the shortest existing demo path, also see [`QUICKSTART_5_MIN.md`](./QUICKSTART_5_MIN.md). For the prior verified zero-contact registry installation evidence, see [`M7_1_EVIDENCE.md`](./milestones/M7_1_EVIDENCE.md).

---

**Related tracking:** Issue #32 — Technical Evaluation Pack v1.