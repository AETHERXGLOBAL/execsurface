# M9.2 — Independent External Quickstart

This path is for a third-party evaluator who wants to create independently attributable ExecSurface evidence.

**Scope:** current public alpha, Linux x86_64, public `ptrace` backend.

## Independence rule

To count toward M9.2 independent adoption, you must initiate and execute the evaluation yourself. AETHER X may point you to this public document, but AETHER X must not run the commands, edit your workload, or materially debug the result for you.

If you need material execution help, the result can still be useful external evidence, but it will be classified as `COLLABORATIVE_EXTERNAL`, not `INDEPENDENT_USER`.

## 1. Use a non-secret test context

Run this only on a command/workspace whose metadata you are comfortable referencing publicly or sharing with explicit consent.

Do not put secrets, tokens, credentials, private file contents, sensitive environment-variable values, stdin contents, or network payloads into the evidence submission.

## 2. Record your project revision

From the project you are evaluating:

```bash
git rev-parse HEAD
```

Preserve the repository URL and exact revision.

## 3. Install the pinned public evaluation release

```bash
cargo install execsurface --version "=0.1.0-alpha.2" --locked
execsurface --version
execsurface doctor
```

Expected version:

```text
execsurface 0.1.0-alpha.2
```

On the supported scope, `doctor` should report Linux, x86_64, ptrace availability, a writable workspace, and Ready.

## 4. Choose one stable, non-secret command

Prefer a user-like runtime command whose build/setup is already complete. Full build/test graphs can legitimately contain temporary runtime paths and may be noisier than a compiled CLI invocation.

Keep the same `/bin/bash -lc` wrapper for baseline and reruns.

Set your command locally, for example:

```bash
EVAL_COMMAND='./target/release/my-tool --version >/dev/null'
```

The example is illustrative. Use a real command from your own project and do not copy a command that does not apply to it.

## 5. Learn the baseline

```bash
execsurface learn -- \
  /bin/bash -lc "$EVAL_COMMAND"
```

Review the generated `execsurface.lock.json` before sharing or committing it.

## 6. Run the unchanged check twice

```bash
execsurface check \
  --json-output pass-1.json \
  --markdown-output pass-1.md \
  -- /bin/bash -lc "$EVAL_COMMAND"

execsurface check \
  --json-output pass-2.json \
  --markdown-output pass-2.md \
  -- /bin/bash -lc "$EVAL_COMMAND"
```

For a stable comparable command, the desired unchanged result is:

```text
ExecSurface: PASS
```

A REVIEW, ERROR, timeout, or non-comparable result is still useful evidence. Do not rerun until you get a preferred result and hide the earlier outcome; preserve failures and friction.

## 7. Optional controlled drift

This step is optional for an individual submission, although at least one accepted M9.2 record must demonstrate drift before the milestone can close.

Use a non-secret file already safe for this evaluation and keep the same shell wrapper. For example:

```bash
SAFE_FILE='./README.md'
set +e
execsurface check \
  --json-output drift.json \
  --markdown-output drift.md \
  -- /bin/bash -lc "$EVAL_COMMAND; /usr/bin/sha256sum '$SAFE_FILE' >/dev/null"
DRIFT_STATUS=$?
set -e
printf 'drift exit status: %s\n' "$DRIFT_STATUS"
```

With the built-in unmatched-drift review behavior, a visible added `sha256sum` execution should normally produce `REVIEW` rather than silently becoming PASS. Do not use this example if `README.md` or another chosen file is sensitive; choose only a safe, non-secret local file.

Natural drift from your own project is also acceptable if you can explain it clearly and preserve the exact before/after context.

## 8. Preserve minimal evidence

Record:

```bash
execsurface --version > execsurface-version.txt
uname -a > uname.txt
sha256sum execsurface.lock.json pass-1.json pass-2.json > evidence-SHA256SUMS
```

If you ran controlled drift:

```bash
sha256sum drift.json >> evidence-SHA256SUMS
```

Preserve the JSON reports as the primary machine-readable verdict evidence. You may also keep the Markdown reports for human review.

Do not upload secret-bearing logs or unrestricted process arguments.

## 9. Submit the independent evidence

Open the repository's **M9.2 Independent Adoption Evidence** issue form and provide:

- your external repository/reference;
- pinned project revision;
- exact evaluated command;
- ExecSurface version/install path;
- OS/kernel/architecture facts;
- install, doctor, baseline, unchanged-check, and optional drift outcomes;
- artifact locations and SHA-256 values;
- any failures, friction, false-positive concerns, or suspected false negatives;
- your attestation that you initiated and executed the evaluation without material AETHER X assistance.

The issue is an intake record. It does not automatically become accepted evidence. Privacy, provenance, semantics, and artifact references are reviewed before a canonical M9 record is created.

## What a successful independent record means

It can establish that one identified external evaluator, on one declared host/project/revision/command, independently installed and used ExecSurface and produced the recorded result.

It does **not** establish that the program is safe, that ExecSurface observed all possible behavior, that the product is production-ready, or that all Linux workloads will behave the same way.
