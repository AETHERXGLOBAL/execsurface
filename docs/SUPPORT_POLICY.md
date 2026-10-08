# ExecSurface Stable Support and Deprecation Policy

Status: **V1 STABLE OPERATING POLICY — ACTIVE**  
Applies when a stable `v1.x` line is published.

This policy defines the operational lifecycle for the bounded stable surface. It does not claim that v1.0 is already released.

## Supported stable line

After v1.0 exists:

- exact releases such as `v1.0.0`, `v1.0.1` and later `v1.x.y` are immutable release identities;
- `AETHERXGLOBAL/execsurface@v1` is the moving stable GitHub Action channel;
- the current supported stable release is the newest release that has passed the full stable release gate and to which `@v1` points;
- older exact v1 releases remain available for rollback, reproduction and compatibility diagnosis.

No fixed calendar maintenance period or response-time SLA is promised by this document.

## Compatibility policy inside v1

Stable v1.x changes must be backward compatible with the frozen stable contract unless an explicit correctness exception is documented and separately qualified.

Within v1.x, do not silently:

- reassign PASS / ERROR / REVIEW / BLOCK exit codes;
- reinterpret old baseline bytes under different canonical semantics;
- relax incomplete evidence into PASS eligibility;
- change a stable policy matcher/action meaning;
- remove or semantically reassign a stable CLI command/argument;
- remove or semantically reassign a stable GitHub Action input/output.

When meaning cannot be preserved safely, explicit rejection is preferred to silent acceptance.

## Deprecation policy

A stable surface may be marked deprecated within v1.x when a compatible replacement exists.

Normal deprecation requires:

1. public documentation of the deprecated surface;
2. a documented compatible replacement or migration path;
3. continued compatibility for the remainder of the v1 major line unless keeping the behavior would violate a separately proven semantic-correctness invariant.

Removal of a stable v1 surface normally requires the next major version.

Research/experimental surfaces are not covered by this promise unless explicitly promoted into the stable contract.

## GitHub Action channel

`@v1` is a deliberately movable channel, not an immutable release identity.

It may move only after the target exact release has passed the stable release, artifact, consumer, compatibility and rollback gates.

Exact `v1.x.y` Action tags remain immutable.

## Bad-release rollback

A bad stable release must be recoverable without rewriting user baseline or policy history.

The rollback procedure is:

1. identify the last qualified immutable v1 release;
2. preserve the bad release/tag and its failure evidence;
3. move the `@v1` channel back to the last qualified immutable v1 release through the authorized release-control path;
4. never rewrite user baselines or policies as part of rollback;
5. document the affected semantic/operational boundary;
6. where appropriate, mark or yank the bad registry version without deleting historical evidence.

Rollback does not turn an incompatible baseline into a compatible one.

## Release correction policy

A correctness fix may change behavior observed in an earlier stable release only when retaining the old behavior would violate the frozen semantic contract or a separately proven invariant.

Such a correction requires:

- exact affected promise;
- counterexample/evidence;
- compatibility impact;
- upgrade/rollback behavior;
- adversarial regression;
- release notes.

A bug is not preserved merely because it shipped.

## Platform support

Stable platform support is installation-route-specific and evidence-gated.

The v1 contract is defined in `docs/COMPATIBILITY.md`. No statement in this policy expands platform, kernel, libc, container or backend support beyond that contract.

## No feature-count maturity rule

Stable support does not require Windows, ARM64, eBPF/BPF-LSM authority, backend auto-selection or a general variance feature.

Those remain separate research/expansion decisions unless later promoted with explicit evidence.
