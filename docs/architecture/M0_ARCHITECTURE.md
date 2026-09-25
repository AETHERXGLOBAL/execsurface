# M0 — Architecture Freeze

Status: **ACCEPTED**

Issue: #1

Date: 2026-09-25

## 1. Problem statement

Git/source/dependency diffs describe changes to inputs. They do not directly answer whether the resulting program began producing new externally observable runtime effects.

ExecSurface answers:

> Given a declared command, baseline context, normalization profile and independent policy, did a candidate execution introduce new externally observable effects, exactly what changed, and what CI verdict follows?

The target abstraction is not tracing itself:

```text
raw observation
    ↓
canonical execution surface
    ↓
content-addressed baseline
    +
independent policy
    ↓
deterministic drift classification
    ↓
verdict + evidence
```

## 2. Scope and system boundaries

### In scope, Linux-first

Process:
- descendant creation/relations,
- executable identity,
- executable family,
- parent/child relation,
- privacy-safe argv metadata.

Filesystem:
- open/access intent,
- reads when fd-to-path attribution is reliable,
- writes/creates/deletes/renames,
- path class: workspace, temp, home, credential-sensitive, system, outside-workspace, device/special, unknown.

Network:
- outbound connection attempts,
- address family,
- destination IP,
- destination port,
- protocol metadata when reliably observable.

Special resources:
- `/dev/*`,
- Unix sockets and special filesystem resources when the backend provides reliable identity.

### Explicitly out of scope

- antivirus or malware classification,
- EDR replacement,
- generic runtime anomaly detection,
- sandbox/enforcement claims,
- proof of safety,
- proof of absence of unobserved behavior,
- payload inspection,
- file-content capture,
- environment-value capture,
- AI classification in MVP,
- macOS/Windows before tested backends exist.

### Hostname boundary

A `connect(2)` observation normally establishes an address, not the application-level hostname that caused resolution.

Therefore the reliable MVP network key is:

`address-family + destination IP + destination port`.

Hostname attribution is optional evidence only when obtained through a documented metadata-safe mechanism. Reverse DNS must never be treated as proof of the hostname requested by the program.

## 3. Threat model

### Assets

- integrity of canonical effects,
- baseline integrity,
- policy integrity,
- completeness status,
- developer trust,
- confidentiality of secrets in argv/env/files/stdin/network payloads.

### Threats addressed

- dependency introduces a new executable,
- build/test path introduces a network endpoint,
- subprocess reaches a credential-sensitive path,
- new write occurs outside declared zones,
- ancestry introduces new external effects,
- normalization hides meaningful drift.

### Threats not solved

- compromised kernel/observer,
- trace-aware malicious program changing behavior,
- unexecuted code paths,
- covert channels,
- kernel-only behavior outside observer semantics,
- activity outside the traced process tree,
- sandbox escape prevention,
- malware verdicts.

If trace loss, parser loss, truncation or a completeness-fatal observer error occurs, verdict is **ERROR**, never PASS.

## 4. Privacy model

Rule: **observe metadata, not secrets**.

Default collection MUST NOT include:
- file contents,
- environment variable values,
- network payloads,
- stdin,
- token/secret contents,
- full child argv values.

Default argv metadata may include argument count and explicitly safe structural metadata.

Hashing is not automatic redaction: low-entropy secrets can be dictionary attacked. Secret-bearing values are not made safe merely by hashing them into a public lockfile.

## 5. Canonical event model

Raw records are backend-specific and may include PIDs, timestamps, syscall names, fd numbers and runner-specific paths.

Canonical effects are backend-independent.

Initial effect families:
- `process.spawn`
- `process.exec`
- `file.open`
- `file.read`
- `file.write`
- `file.create`
- `file.delete`
- `file.rename`
- `network.connect`
- `resource.special`

M1 may emit only effects it can substantiate. Unsupported semantics must be declared.

### Process identity

PIDs/TIDs are raw evidence, not baseline identity.

Canonical process context uses normalized executable identity plus bounded ancestry. Repeated instances may collapse for surface comparison while raw evidence retains multiplicity.

### File identity and symlinks

A path associated with an fd must be attributable through observer evidence.

Post-hoc `realpath()` is not automatically authoritative because resolution can race with filesystem changes.

Canonical file evidence therefore needs:
- observed lexical path,
- normalized path,
- path class,
- resolution status,
- optional inode/device evidence when useful.

Ambiguity is represented explicitly rather than guessed away.

### Ordering

Surface identity is primarily set/graph based, not timestamp-order based. Parent/child relations remain explicit. Canonical records are deterministically sorted.

## 6. Baseline / lock model

Default filename:

`execsurface.lock.json`

The lockfile means:

> accepted observed canonical execution surface for a declared comparison context.

It is not a raw trace, security proof, or policy.

Required conceptual fields:
- schema/version,
- tool version,
- command identity metadata,
- platform metadata,
- observer name/version,
- observer capability/limitation declaration,
- normalization profile/version,
- canonical effects,
- canonical process relations,
- baseline digest.

### Baseline vs policy

Baseline = what was observed and accepted as reference.

Policy = what is allowed.

They remain independent schemas and can disagree.

### Command identity privacy

A public lockfile must not require complete argv. Default command identity uses executable identity, argument count and/or a user-supplied logical command label. Exact argv capture requires explicit private-mode policy later.

### Content addressing

v1 intends SHA-256 over a versioned deterministic JSON representation, excluding self-referential digest fields. Exact serialization test vectors must be frozen before M3.

Incompatible schema, normalization profile or backend semantics make runs non-comparable and yield ERROR rather than silent PASS.

## 7. Normalization policy

Normalization is the main false-positive/false-negative boundary.

Rule:

> Normalize only variability demonstrated to be irrelevant to execution-surface meaning.

Do not suppress an effect merely because it is common.

### Noise taxonomy

| Source | M0 treatment |
|---|---|
| PID/TID | remove from canonical identity; retain in raw evidence |
| timestamps/durations | exclude from surface identity |
| local ephemeral ports | may be excluded when only client-side ephemeral identity |
| remote destination port | always remains target identity |
| random temp tokens | normalize only under declared temp roots and tested token rules |
| GitHub runner workspace prefix | replace only from explicitly declared workspace root |
| home prefix | replace declared home root with `$HOME`; suffix remains significant |
| package cache | map only declared cache roots |
| dynamic linker/library reads | retain; baseline can absorb stable behavior |
| process scheduling | do not use timestamp order as surface identity |
| concurrent children | preserve ancestry/effect association; deterministic sort |

Allowed semantic root tokens may include:
- `$WORKSPACE`
- `$HOME`
- `$TMP`
- `$CACHE:<name>`

### Required adversarial test for every normalization rule

1. rationale,
2. positive fixture,
3. adversarial counterexample,
4. statement of information removed,
5. assertion that distinct security-relevant targets do not collapse.

Example unsafe normalization:

`/tmp/plugin-AB12/curl` and `/tmp/plugin-CD34/ssh` must never collapse so far that the executable difference disappears.

Credential-sensitive `$HOME/.ssh/config` must not become generic `$HOME/config`.

Changing normalization semantics versions the normalization profile.

## 8. Classification and verdict semantics

Initial drift categories:
- NEW_EXEC
- NEW_NETWORK
- NEW_READ
- NEW_WRITE
- NEW_DELETE
- NEW_DEVICE
- NEW_PATH_CLASS

Finding severities:
- INFO
- LOW
- MEDIUM
- HIGH
- CRITICAL

Severity mapping is configurable and does not define baseline identity.

### PASS

Observation completed; candidate is comparable; no disallowed expansion and no configured review-only drift remains.

PASS does not mean safe.

### REVIEW

Observation completed and candidate is comparable, but new effects require review under policy.

### BLOCK

Comparable drift violates explicit policy.

Examples:
- credential-sensitive read,
- undeclared network under deny-by-default policy,
- forbidden executable class,
- write outside allowed zones.

### ERROR

A trustworthy verdict cannot be produced.

Examples:
- observer failure,
- trace truncation,
- parser loss,
- descendant-tracking failure,
- incompatible baseline/profile,
- canonicalization invariant failure.

The target command exit code and ExecSurface verdict are independent.

Removed effects are reported but do not BLOCK by default.

## 9. MVP definition

Target CLI:

```bash
execsurface learn -- COMMAND
execsurface check -- COMMAND
```

MVP must prove:
- descendant process execution observation,
- canonical executable identity,
- file effects with explicit attribution/completeness,
- outbound connection attempts,
- deterministic canonicalization,
- content-addressed baseline,
- added/removed effect diff,
- independent policy,
- PASS/REVIEW/BLOCK/ERROR,
- terminal/JSON/Markdown reports,
- SARIF only when semantics fit,
- evidence bundle.

### Mandatory fixtures

1. stable baseline command,
2. candidate adds child executable,
3. candidate adds outbound connection,
4. candidate accesses credential-sensitive fixture path,
5. candidate writes outside workspace,
6. same logical behavior with different PID/temp values canonicalizes identically,
7. observer failure → ERROR,
8. concurrent descendants do not lose effects,
9. symlink ambiguity is surfaced,
10. high event volume detects loss/truncation instead of passing.

### Independent reproduction gate

MVP is not accepted until a clean Linux CI environment reproduces controlled baseline → change → exact finding.

## 10. Performance discipline

No "low overhead" claim before reproducible measurement records:
- runner/machine,
- kernel,
- backend/version,
- command,
- repetitions,
- wall time,
- CPU,
- memory,
- event volume/raw size,
- canonicalization time.

## 11. M0 status labels

- raw vs canonical separation: **PROVED BY ARCHITECTURE**
- baseline vs policy separation: **PROVED BY ARCHITECTURE**
- observer failure → ERROR: **PROVED BY CONTRACT**
- privacy-safe default model: **PROVED BY DESIGN**
- concrete strace completeness: **OPEN M1**
- fd/path correctness: **OPEN M1/M2**
- temp-token algorithm: **OPEN M2**
- hostname attribution: **OPEN / DEFERRED**
- low-overhead claim: **OPEN**
- novelty/patentability: **OPEN**
