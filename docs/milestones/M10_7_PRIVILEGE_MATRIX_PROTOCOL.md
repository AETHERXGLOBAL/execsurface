# M10.7 — PRIVILEGE-MATRIX-001 Protocol

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m107`
Status: **PREREGISTERED — NO RESULT YET**

## Objective

Determine the practical privilege/deployment boundary of the candidate hybrid BPF/LSM observation architecture across hosted CI, containers, user identities, and user namespaces, without confusing semantic strength with deployability.

## Fixed roles

- **Innovative Systems Architect** — identify deployment models that preserve stronger kernel evidence without silently imposing unacceptable privilege or service requirements.
- **Anti-Deviation / Skeptical Reviewer** — block promotion to default/public architecture if required privilege, kernel configuration, container policy, or hosted-CI support is absent.

## Dynamic specialists

Linux capabilities/BPF privilege, LSM activation, containers/seccomp, user namespaces, GitHub Actions runners, QEMU boot configuration, Rust/Linux packaging implications, threat modeling, evidence semantics, and internal red team.

## Prior accepted evidence incorporated

This gate does not repeat closed tests unnecessarily.

Accepted prior facts:

1. M10.3A: standard GitHub-hosted `ubuntu-22.04`, `ubuntu-24.04`, and `ubuntu-26.04` exposed readable BTF but did **not** expose `bpf` in the active LSM list; all reported `unprivileged_bpf_disabled=2`.
2. M10.3B: a boot-controlled Ubuntu `6.8.0-142-generic` guest with `CONFIG_BPF_LSM=y` booted successfully with `lsm=bpf`, active list `lockdown,capability,bpf`, and readable BTF.
3. M10.3D/M10.4/M10.5: real BPF-LSM programs loaded/attached successfully in that controlled root guest.

## New executable matrix

A static privilege probe will attempt a minimal `BPF_MAP_CREATE` operation and record return/errno under the following contexts.

### P1 — GitHub-hosted runner host

Record:
- active LSM list when readable;
- `/proc/sys/kernel/unprivileged_bpf_disabled`;
- root-context minimal BPF map-create result.

This does not grant BPF-LSM capability if `bpf` is absent from the active LSM list.

### P2 — Default container on GitHub-hosted runner

Create a local image from a minimal scratch rootfs containing only the static probe, avoiding registry/image-network dependence. Run without added capabilities/privilege and record BPF map-create return/errno.

### P3 — Privileged container on the same host

Run the same local image with `--privileged` and record BPF map-create return/errno.

Even if generic BPF map creation succeeds, BPF-LSM is still unavailable when the host kernel's active LSM list lacks `bpf`.

### P4 — Boot-controlled guest root

Boot the accepted generic Ubuntu kernel with `lsm=bpf`; record active LSM list and root minimal BPF map-create result.

### P5 — Boot-controlled guest unprivileged UID

From the same guest, drop real/effective/saved UID/GID to `65534` and attempt the identical BPF map create. Record return/errno.

### P6 — Boot-controlled guest user namespace

Create a new user namespace and attempt the identical BPF map create from the resulting namespace context. Record whether namespace creation succeeds and the BPF return/errno. No assumption is made before execution about the exact errno.

## Frozen interpretation rules

The candidate hybrid backend **cannot be a drop-in default for the current Marketplace/GitHub-hosted path** if standard hosted runners lack active BPF LSM, regardless of whether a privileged custom environment can support it.

A boot-controlled/root-only success may establish technical deployability in managed/self-hosted environments but not universal self-service portability.

Default-container denial is a deployment constraint, not a semantic failure.

## Frozen classifications

- `M10_7_PRIVILEGE_MATRIX_CONFIRMED` — standard hosted BPF-LSM remains unavailable; boot-controlled root BPF operation succeeds with active BPF LSM; unprivileged guest BPF operation is denied; all matrix records are captured without harness ambiguity.
- `M10_7_PRIVILEGE_MATRIX_SURPRISE` — an observed privilege boundary materially contradicts the preregistered model and requires architecture review before closeout.
- `M10_7_MATRIX_INCOMPLETE` — one or more mandatory matrix contexts lack interpretable evidence.
- `M10_7_INFRA_FAILURE` — harness failure prevents matrix execution.

Privileged-container and user-namespace outcomes are recorded as required facts but are not independently required to equal a predeclared errno/value for `M10_7_PRIVILEGE_MATRIX_CONFIRMED`.

## Product decision implication

If the matrix confirms that BPF-LSM requires a boot/kernel/privilege environment unavailable on standard GitHub-hosted runners, M10 closeout must separate:

- **evidence architecture authority** — whether hybrid kernel hooks are stronger for selected propositions; from
- **default product backend** — whether the mechanism is deployable for current users without new privilege/infrastructure requirements.

## Non-authorizations

This gate does not authorize a privileged daemon, silent elevation, host policy changes, public BPF PASS/`learn/check`, baseline interchangeability, backend auto-selection, ptrace replacement, or changes to `main`, Marketplace, or `v0.1.0-alpha.3`.
