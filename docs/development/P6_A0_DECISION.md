# ExecSurface — P6-A0 Source / Capability Freeze Decision

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Protocol: `docs/development/P6_COMPETITIVE_FALSIFICATION_PROTOCOL.md`
Matrix: `docs/development/P6_A0_OVERLAP_MATRIX.md`

## Decision

**P6_A0_OVERLAP_MATRIX_FROZEN**

This decision freezes the initial factual comparison surface and executable source identities. It is not a competitive result and contains no product ranking.

## Accepted evidence

Successful A0 source/capability gate:

- source SHA: `f2174f08134d62ed3e19879b8ff54ff7e1a07aea`
- workflow: `.github/workflows/p6-a0-source-freeze.yml`
- run: `36769654116`
- conclusion: `success`
- evidence artifact ID: `11122743690`
- artifact name: `p6-a0-36769654116-1`
- artifact digest: `sha256:23fb182b3bc1fc269883059c2ccdd700a414033aeae5fb26264866fcf01f08f5`
- artifact size: `10287` bytes
- artifact expiry at capture: `2026-10-30T20:01:19Z`

The run independently verified:

1. P5 closeout ancestry at `1f4df8f45a115a1638c997107e069c6acd74c51b`;
2. immutable public `v0.1.0-alpha.4` and `v0.1` source `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
3. no public `crates/` changes after P5 closeout;
4. machine-readable manifest consistency and prohibition on aggregate scoring;
5. Harden-Runner `v2.21.1` -> `e14015d583714f6e62063499dc959a02595150a1`;
6. cicd-sensor-action `v0.0.38` -> `6511eb44c91d71b2b93d71193b1bf2cb18352f66`;
7. exact fetchability of all frozen external source commits;
8. pinned Harden-Runner action contract;
9. pinned cicd-sensor runtime-trace predicate contract;
10. pinned Tetragon process/network evidence schema availability;
11. runner preflight on Ubuntu 24.04 x64, kernel `6.17.0-1022-azure`, Docker `28.0.4`.

## Frozen external identities

| System | Frozen identity | A1 role |
|---|---|---|
| Harden-Runner | `e14015d583714f6e62063499dc959a02595150a1` | primary GitHub-hosted CI comparator |
| cicd-sensor source | `1f031a106e23edda1eb496b0bae51fb12e85d62d` | primary runtime-trace/source reference |
| cicd-sensor action | `6511eb44c91d71b2b93d71193b1bf2cb18352f66` | primary GitHub-hosted executable comparator |
| Tetragon | `666efe6f91e3605ad58683ad226d759d9cf970ca` | primary runtime comparator, exact-source execution only until an immutable image digest is frozen |
| Falco | `e12b1d43e47a2903c07e14479e034d74d523ab9d` | secondary overlap-only |
| Tracee | `2f9dc40c20b17c2ba27f6d92b25e62790bd48a62` | secondary overlap-only |

## Proposition applicability freeze

- **CP1 process execution**: ExecSurface, Harden-Runner, cicd-sensor, and Tetragon are eligible for executable observation checks. Falco/Tracee remain secondary until a narrower event fixture is frozen.
- **CP2 outbound destination**: same primary set; success/attempt semantics must be reported separately rather than inferred from event presence.
- **CP3 file mutation/write**: same primary set; path observation is not treated as object-identity equivalence.
- **CP4 run evidence/attestation**: ExecSurface and cicd-sensor have direct bounded overlap. Harden-Runner remains `A0_PARTIAL`; the pinned action declares no action outputs and P6 will not reinterpret logs/security-insights links as an attestation-equivalent artifact without evidence. Tetragon/Falco/Tracee remain `NOT_APPLICABLE/NO_ASSUMPTION` until a directly comparable run-level artifact is demonstrated.

## Tetragon execution boundary

A floating image is forbidden. P6-A1 may execute Tetragon only if one of these conditions is met before that lane starts:

1. the exact source commit above is built/executed reproducibly in the workflow; or
2. a container image digest is independently resolved and frozen in a separate preregistered evidence commit.

Until then, the Tetragon live lane remains fail-closed and its absence cannot be counted against it or in favor of ExecSurface.

## Privacy / data boundary

Raw evidence is limited to the deterministic P6 workload and public CI metadata. No production secrets, user data, private endpoints, or sensitive host paths may be introduced. A comparator's documented telemetry or external service behavior must be recorded as operational/privacy context rather than silently normalized away.

## Scientific boundary

A0 establishes only that P6 now has a frozen comparison surface and immutable source/action identities. It does NOT establish:

- that ExecSurface is superior to any comparator;
- that any two backends are semantically equivalent;
- that unsupported behavior is absent;
- that a missing run-level attestation is a product defect;
- any performance claim;
- any public release/promotion decision.

## Authorization for P6-A1

P6-A1 is authorized only under its own preregistered same-workload protocol. The live corpus must preserve the exact A0 pins, use deterministic scenarios, retain all failures, and publish factual per-proposition observations without an aggregate score.
