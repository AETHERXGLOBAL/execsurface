# M10 — Kernel-Hook Evidence Authority / Hybrid Observer Closeout

Date: 2026-09-28
Tracking: #88
Research lineage: `research/m10-hybrid-observer-*`
Public product authority at closeout: unchanged

## Final classification

`M10_BOUNDED_HYBRID_AUTHORITY_VALIDATED_PRODUCT_DEFAULT_DEFERRED`

## Executive decision

M10 began by attacking the assumption that the native ptrace observer should remain the strongest correctness authority for all ExecSurface evidence propositions.

That assumption did **not** survive intact.

M10 established two real ptrace counterexamples, then built and tested a bounded hybrid kernel-observation architecture using BPF LSM security hooks plus kernel lifecycle/syscall tracepoints. The hybrid path produced stronger object-grounded evidence for controlled file, exec, and connect propositions and demonstrated explicit fail-closed behavior under real producer loss.

However, the same program also proved that BPF LSM is not currently available as a drop-in default on the standard GitHub-hosted runner environments tested, and that the stronger path carries explicit kernel/privilege/deployment requirements.

Therefore M10 does **not** authorize replacing the current public ptrace backend with mandatory BPF LSM.

The next architecture must separate:

1. **portable observation** from
2. **kernel-object evidence authority** from
3. **evidence completeness/health** from
4. **product backend selection**.

## Fixed review roles

Throughout M10:

- **Innovative Systems Architect** searched for stronger mechanisms rather than defending the installed design.
- **Anti-Deviation / Skeptical Reviewer** rejected sunk-cost defenses, invalid harness evidence, semantic shortcuts, and premature product promotion.

Dynamic specialists changed by gate across Linux ptrace, LSM/BPF, VFS objects, exec lifecycle, networking, ring-buffer loss, privileges, containers, namespaces, QEMU/kernel configuration, CI/reproducibility, evidence semantics, and internal red team.

## Gate ledger

### M10.1 — PATH-TOCTOU-001

Classification:
`PTRACE_PATH_TOCTOU_COUNTEREXAMPLE_OBSERVED`

Accepted evidence:
- 500 complete, warning-free adversarial runs;
- 243 cases where entry-time `PathAccessIntent` differed from independently established kernel-opened object truth;
- 0 fd-read object mismatches in the same gate.

Decision:
The ptrace entry-copied pathname is **userspace argument metadata observed at syscall entry**, not universally kernel-authoritative object identity.

### M10.2 — FD-SHARE-RACE-001

Classification:
`PTRACE_FD_ATTRIBUTION_COUNTEREXAMPLE_OBSERVED`

Accepted evidence:
- target-proven successful relevant reads: 2851;
- emitted relevant fd-read events: 2018;
- missing successful relevant reads: 833;
- observer state still reported `complete=true` with zero warnings.

Decision:
Universal completeness of current ptrace fd read/write attribution under concurrent shared-fd close/reuse is **killed**.

This is not only a documentation limitation: the combination of omitted target-proven operations and `complete=true` is an evidence-authority gap that the next-version architecture must address.

### M10.3A / M10.3B — environment feasibility

Standard hosted matrix:
- GitHub-hosted Ubuntu 22.04 / 24.04 / 26.04 exposed BTF but did not expose `bpf` in the active LSM list;
- all reported `unprivileged_bpf_disabled=2`.

Boot-controlled guest:
- Ubuntu kernel `6.8.0-142-generic`;
- `CONFIG_BPF=y`, `CONFIG_BPF_SYSCALL=y`, `CONFIG_BPF_LSM=y`;
- booted with `lsm=bpf`;
- active list `lockdown,capability,bpf`;
- BTF readable.

Decision:
BPF LSM is technically feasible in a controlled compatible environment, but cannot be assumed on the current hosted Marketplace path.

### M10.3D — FILE authority

Classification:
`M10_3_OBJECT_IDENTITY_MATCH`

Protocol-faithful fixture:
- target-side `fstat(2)` truth `(dev=2, ino=21)`;
- unique accepted BPF-LSM `file_open` event `(dev=2, ino=21)`;
- exactly one controlled event;
- malformed=0, wrong_session=0, drops=0, target exit=0.

Decision:
For the bounded proposition, BPF-LSM supplied kernel-object-grounded file identity stronger than ptrace entry-copied pathname metadata.

### M10.4 — EXEC authority

Classification:
`M10_4_HYBRID_EXEC_MATCH`

Architecture:
- BPF LSM provides pre-commit executable object candidate;
- `sched_process_exec` provides independent successful-exec confirmation.

Accepted result:
- successful executable candidate and confirmation matched target-side object ground truth `(dev=2, ino=14)`;
- failed exec (`ENOENT`) generated no false success confirmation;
- malformed=0, wrong_session=0, unknown_role=0, drops=0.

Decision:
An exec attempt must not be equated with successful exec. The tested hybrid composition represented the distinction correctly.

### M10.5 — CONNECT authority

Classification:
`M10_5_HYBRID_CONNECT_MATCH`

Architecture:
- BPF LSM `socket_connect` supplies kernel-mediated destination candidate;
- `sys_exit_connect` supplies independent success/failure completion.

Accepted result:
- successful IPv4/TCP loopback candidate matched target `getpeername()` and server-side `accept()` truth;
- successful syscall exit returned 0;
- controlled failed connect returned `ECONNREFUSED` / kernel exit `-111` and produced zero success confirmations;
- malformed=0, wrong_session=0, unknown_role=0, drops=0.

Decision:
A connect attempt must not be treated as proof of successful connection. The tested hybrid composition preserved that distinction.

### M10.6 — LOSS-AUTHORITY-001

Classification:
`M10_6_REAL_LOSS_DETECTED_FAIL_CLOSED`

Frozen real-loss result:
- producer attempts: 200000;
- accepted events: 127;
- actual `bpf_ringbuf_reserve()` failures: 199873;
- exact accounting: `127 + 199873 = 200000`;
- malformed=0, wrong_session=0, target exit=0;
- `evidence_complete=false`;
- `pass_authority=false`.

Decision:
Real transport/producer loss can be made a first-class evidence-health fact and can fail closed rather than masquerade as no drift.

### M10.7 — PRIVILEGE-MATRIX-001

Classification:
`M10_7_PRIVILEGE_MATRIX_CONFIRMED`

Accepted matrix:

| Context | Active BPF LSM | Minimal BPF map create |
|---|---:|---:|
| GitHub-hosted Ubuntu 24.04 host root | No | Success |
| Default local container | Host lacks BPF LSM | Denied `EPERM` |
| Privileged local container | Host lacks BPF LSM | Success |
| Boot-controlled guest root | Yes | Success |
| Boot-controlled guest UID/GID 65534 | Yes | Denied `EPERM` |
| Boot-controlled guest new user namespace | Yes | Denied `EPERM` in tested configuration |

Decision:
Generic BPF capability, container root, and active BPF LSM are separate deployment facts. Stronger kernel-hook evidence is viable in managed/compatible environments but is not a universal self-service default.

## Architectural conclusion

### 1. ptrace remains useful, but its authority must be bounded

Ptrace still provides valuable properties for the current self-service product:

- launch-scoped observation;
- descendant following;
- syscall entry/exit correlation;
- selected metadata reads;
- fd lifecycle modeling;
- no persistent privileged daemon in the normal public path.

But M10 rejects any architecture that implicitly treats all ptrace-derived metadata as kernel-object truth or treats current `complete=true` as sufficient under the demonstrated shared-fd race.

### 2. BPF LSM is not a drop-in ptrace replacement

BPF LSM is stronger for selected kernel-object propositions but does not automatically replace:

- launch/session lifecycle;
- success confirmation;
- transport loss accounting;
- portable hosted deployment;
- baseline compatibility semantics.

The validated direction is therefore **hybrid**, not substitution-by-hook-name.

### 3. Evidence authority must become explicit and versioned

The next-version internal evidence contract should represent, at minimum:

- observer/backend identity and version;
- proposition/event class;
- evidence source/authority class;
- session membership;
- capability fingerprint;
- collection-health/completeness state;
- loss accounting where applicable;
- comparability rules.

A user-observed syscall argument, a kernel-object identity, and a success-confirmed operation must not be silently collapsed into one undifferentiated fact.

### 4. Backend selection and evidence authority are separate decisions

A stronger backend is not automatically the default backend.

Product selection must account for:

- privileges;
- kernel configuration;
- CI/container availability;
- packaging;
- performance;
- user workflow;
- backward compatibility;
- baseline comparability.

## Required next-version work

M10 authorizes a new isolated integration/hardening program, but not release promotion.

The next program must:

1. introduce an authority-aware, capability-versioned internal evidence model without silently changing existing baselines;
2. correct the ptrace pathname claim boundary so entry-copied paths are not represented as universally kernel-authoritative;
3. eliminate or fail-close the demonstrated shared-fd false-completeness condition;
4. retain ptrace as the portable path until a stronger backend is available under explicit capability requirements;
5. integrate the hybrid kernel-authority path only as an explicit capability, initially non-default;
6. prove proposition-level comparability before any cross-backend baseline reuse;
7. preserve metadata-only privacy boundaries;
8. prove fail-closed behavior for every new authority-bearing event class;
9. run backward-compatibility and Marketplace regression gates before any next public release.

## Public/current release decision

At M10 closeout:

- `main` remains unchanged from the pre-development freeze;
- public release remains `v0.1.0-alpha.3`;
- Marketplace behavior remains unchanged;
- existing baselines remain unchanged;
- ptrace remains the current public backend under its bounded semantics;
- no BPF-LSM public PASS/`learn`/`check` authority is granted;
- no cross-backend baseline interchangeability is granted;
- no automatic backend selection is granted;
- no privileged daemon/service is authorized.

## Final M10 decision

**CLOSE M10.**

The external criticism uncovered a genuine architectural gap. The research program then validated a stronger bounded hybrid evidence architecture, while also proving why that architecture cannot simply replace the current public backend in every environment.

The correct next step is an isolated next-version integration/hardening program, not immediate modification of the published alpha.
