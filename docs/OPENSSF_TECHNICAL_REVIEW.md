# ExecSurface — OpenSSF Technical Review Pack

Tracking: #94
Current public release: `v0.1.0-alpha.4`
Current public source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
Audience: OpenSSF engineers/maintainers/researchers evaluating technical fit, overlap, reproducibility and architecture.

## Review intent

This is a request for **technical criticism and independent reproducibility**, not endorsement.

Negative evidence, no-fit conclusions, architecture criticism and overlap findings are explicitly useful outcomes.

## Q1 — What problem does ExecSurface solve today?

Source review and artifact provenance can show what was built or changed, but they do not directly show every effect that a particular command exhibited when executed.

ExecSurface addresses the narrower problem:

> Learn an accepted, canonical **observed runtime execution surface** for a declared command, then surface later observed execution behavior that appeared, disappeared or changed under a recorded observer and explicit policy.

Target use cases include CI jobs, dependency/tool execution, developer tools and AI-assisted workflows where runtime behavior matters and source/provenance evidence alone is incomplete.

## Q2 — How does it differ from adjacent mechanisms?

| Mechanism / project class | Primary job | Difference from ExecSurface |
|---|---|---|
| SELinux / AppArmor | Mandatory access-control policy mediation and enforcement through LSM | ExecSurface's public product is not MAC enforcement. It runs a command, records bounded evidence, canonicalizes it, compares to an accepted baseline and evaluates explicit drift policy. LSM hooks are relevant as potential stronger evidence anchors, not drop-in replacements for the current whole evidence pipeline. |
| seccomp | Syscall filtering / mediation | Installing seccomp changes the target execution policy. ExecSurface currently seeks an observer-backed evidence path rather than a syscall allow/deny sandbox. |
| auditd / Linux Audit | Host/system audit event generation and filtering | Audit is system-policy/configuration oriented; ExecSurface is command/session scoped, self-service oriented, and builds baseline/diff/policy evidence. Audit remains a useful independent cross-check candidate. |
| Falco-like tooling | Runtime detection/alerting over kernel/syscall event streams and rules | Falco is a broad runtime detection system. ExecSurface centers on an accepted per-command canonical baseline, reproducible diff and explicit evidence completeness; it does not ship threat/malware rules or infer maliciousness from drift. |
| Tetragon-like tooling | eBPF security observability and optional runtime enforcement | Tetragon deliberately hooks deep in the kernel and can enforce. This makes it strong prior art for kernel-authoritative observation. ExecSurface's current public path is lighter-weight ptrace self-service; its BPF-LSM/kernel-hook direction is research-only and should be challenged for duplication/interop opportunities. |
| tracing tools (strace/perf/bpftrace) | Raw tracing/debugging/measurement | ExecSurface turns bounded observations into a canonical versioned surface, accepted baseline, deterministic diff, policy verdict and machine-readable evaluation evidence. It intentionally avoids unrestricted payload/argv collection. |
| SLSA / in-toto / Sigstore | Build provenance, artifact identity/signing/attestation | These describe how artifacts were produced/identified. ExecSurface describes bounded behavior observed when a command runs. The open question is whether runtime evidence should be bound to/integrated with provenance rather than treated as a separate trust claim. |
| GUAC-like supply-chain graph | Ingest and relate software supply-chain metadata | ExecSurface could potentially be one evidence producer if a useful interoperable model exists. It should not invent a GUAC mapping without community review. |

## Q3 — What is ExecSurface?

Current classification:

- **Observer-backed evidence generator:** YES.
- **Canonical baseline / diff engine:** YES.
- **Explicit drift policy engine:** YES.
- **Detector of observed execution-surface drift:** YES, in the bounded sense above.
- **General threat/malware detector:** NO.
- **Kernel enforcement system:** NO for the public alpha.
- **Sandbox:** NO.
- **Formal verifier of software safety:** NO.

## Q4 — What is the current novel/value hypothesis?

The value hypothesis to falsify is not "Linux has no runtime monitoring".

It is:

> A small, command-scoped, self-service pipeline that records privacy-bounded runtime evidence, canonicalizes it into a reproducible accepted surface, explicitly represents completeness/observer limits, and performs deterministic later drift comparison can be useful inside software-integrity workflows without becoming a full host runtime-security platform.

OpenSSF should explicitly test whether that is genuinely useful, interoperable and non-redundant.

## Q5 — Current limitations

- Linux x86_64 public alpha only.
- Native `ptrace` remains the public default/reference observer.
- ptrace stop/resume overhead is real and workload dependent.
- userspace pathname/sockaddr pointer reads at syscall entry can race with another thread; pathname metadata is not kernel-object identity.
- exact shared-FD attribution under concurrency is not claimed; alpha.4 conservatively fails closed when ambiguity cannot be justified and can therefore create false incompleteness.
- raw observation v2 does not retain enough detail to prove exact `CLONE_FILES` sharing in all relevant cases.
- mmap/io_uring and every possible Linux I/O path are not claimed as fully covered by the current fd-effect model.
- ptrace availability depends on kernel/security/container/namespace configuration; unsupported conditions must fail explicitly.
- the metadata boundary intentionally excludes file contents, environment values, stdin, network payloads and full child argv.
- BPF-LSM/kernel-hook work remains research/managed and non-default.
- no public ptrace↔hybrid baseline equivalence.
- observed behavior is not all possible behavior.

## Q6 — What specifically do we want OpenSSF to try to break?

1. The usefulness of a command-scoped runtime execution-surface baseline in real software-integrity workflows.
2. Reproducibility of the baseline/evidence/diff model from public documentation only.
3. Whether the observer capability/completeness representation is sufficient to prevent false confidence.
4. Whether ptrace is acceptable for the public portable scope or creates unacceptable blind spots/perturbation.
5. Whether our fail-closed shared-FD hardening is too conservative or still misses false-completeness cases.
6. Whether pathname/access-attempt semantics are clear enough and impossible to confuse with kernel-object identity.
7. Whether existing tools (Falco, Tetragon, audit, LSM ecosystems, tracing platforms) already provide the valuable part better.
8. Whether runtime evidence should integrate with ORBIT / Security Insights / Gemara / supply-chain metadata instead of remaining a product-specific schema.
9. Whether our 5–10 minute self-service evaluation is genuinely zero-assistance and reproducible.

## Current architecture

```text
Declared command
      |
      v
Public alpha.4 native ptrace observer
  - launched descendant scope
  - selected metadata only
  - explicit health/completeness
      |
      v
Raw typed observations + observer health
      |
      v
Normalize / canonicalize
      |
      +----------------------+
      |                      |
      v                      v
Accepted baseline        Current surface
      |                      |
      +----------+-----------+
                 v
        deterministic diff
                 |
                 v
          explicit policy
                 |
                 v
      PASS / REVIEW / BLOCK / ERROR
                 |
                 v
      JSON / Markdown evidence
```

Research-only direction, not public default:

```text
stable lifecycle tracepoints + BPF-LSM/kernel hooks
                |
                v
 proposition-scoped stronger kernel-object evidence
                |
        [promotion gates not closed for product default]
```

## Why ptrace remains public after Greg Kroah-Hartman's criticism

The criticism triggered a formal architecture review rather than a defense.

Current conclusion:

- Greg's criticism was technically substantive.
- SELinux/AppArmor are not direct replacements for the per-command evidence-generation pipeline.
- ptrace remains useful for launch scope, descendant following, syscall entry/exit, selected metadata, fd-lifecycle modeling and low-friction self-service.
- ptrace is **not** treated as strongest authority for every proposition.
- PATH-TOCTOU and shared-FD counterexamples were reproduced and retained.
- kernel-hook/hybrid architecture is the research direction for authority-sensitive propositions.
- public promotion remains blocked by proposition coverage, loss accounting, privilege/portability, packaging and compatibility requirements.

Reference: `docs/architecture/PTRACE_VS_LSM_ARCHITECTURE_REVIEW.md`.

## 5–10 minute independent reproduction

Supported public environment: Linux x86_64.

### Install exact alpha.4 public binary

```bash
VERSION=v0.1.0-alpha.4
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

Expected version: `execsurface 0.1.0-alpha.4`.

Do not change host security settings if `doctor` fails. Report the environment and failure.

### Learn accepted surface

```bash
rm -rf /tmp/execsurface-openssf-review
mkdir -p /tmp/execsurface-openssf-review
cd /tmp/execsurface-openssf-review
execsurface learn -- /bin/bash -lc 'true'
```

### Unchanged check

```bash
execsurface check \
  --json-output pass-report.json \
  --markdown-output pass-report.md \
  -- /bin/bash -lc 'true'
```

Expected in a supported complete run: `PASS`, exit `0`.

### Controlled drift

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

Expected in the controlled supported case: `REVIEW`, exit `10`.

Retain any different outcome as evidence.

## Evidence to return

Please report:

- OS, kernel, architecture and container/VM context;
- install path and exact ExecSurface version;
- `doctor` result;
- baseline digest;
- PASS/REVIEW/ERROR/BLOCK results and exit codes;
- JSON evidence files;
- completeness/health state;
- elapsed time / noticeable overhead;
- undocumented assumptions or permission friction;
- false positive / false negative / false incompleteness suspicion;
- anything that prevented zero-assistance reproduction.

Do not send secrets, private source, environment values, file contents, stdin or network payloads.

## Technical review questions

1. Is runtime execution-surface drift useful in actual software-integrity workflows?
2. Is the baseline/evidence model clear and reproducible?
3. Is the backend architecture appropriate to the declared purpose?
4. What syscall/process/object blind spots invalidate or weaken current propositions?
5. Can a third party install and run the evaluation without AETHER X assistance?
6. Which existing tools overlap materially, and where is ExecSurface redundant?
7. Does the architecture add value or merely repackage existing tracing/runtime detection?
8. Should this evidence integrate with supply-chain standards / attestations / OpenSSF metadata ecosystems?
9. What should change for this to become useful to the open-source security ecosystem?

## Prior-art / ecosystem review — initial hypothesis, not a competitive claim

### Falco

Falco is a mature runtime detection/alerting system using kernel/syscall events and rules. It overlaps in observing process/file/network runtime behavior but has a different product center: continuous runtime detection and alerts. ExecSurface should not claim category novelty against Falco.

### Tetragon

Tetragon is strong eBPF-based security observability and enforcement and explicitly emphasizes deep kernel hook visibility that avoids common syscall-tracing/user-kernel boundary problems. It is especially important prior art for our kernel-hook research direction. OpenSSF/community reviewers should tell us whether integration/reuse makes more sense than developing overlapping kernel collection infrastructure.

### SELinux / AppArmor / seccomp / auditd

These are fundamental Linux policy/audit primitives with different scopes. ExecSurface's current differentiation is the command-scoped evidence/baseline/diff pipeline, not superior Linux enforcement.

### SLSA / in-toto / Sigstore / GUAC

These are complementary supply-chain identity/provenance/metadata systems. The key open question is whether ExecSurface runtime evidence should be bound to artifact/workflow provenance and represented in an existing ecosystem model instead of inventing an isolated trust layer.

## Claims boundary

Do not describe the outcome of this review as:

- "OpenSSF validated ExecSurface";
- "Linux Foundation approved ExecSurface";
- "OpenSSF partner";
- "endorsed by OpenSSF".

A Working Group discussion, independent run, issue comment, criticism or contribution must be described exactly as what occurred.

## Negative evidence request

Please prioritize findings that falsify our assumptions:

- installation failure;
- unsupported environment;
- unclear baseline meaning;
- false PASS;
- false REVIEW;
- false incompleteness;
- missing runtime effect;
- observer perturbation;
- excessive cost;
- privilege friction;
- evidence ambiguity;
- overlap/redundancy with existing tools;
- no credible ecosystem fit.

A documented `NO_FIT` is a valid successful review outcome.
