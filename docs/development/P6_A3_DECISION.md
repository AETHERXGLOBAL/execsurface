# ExecSurface — P6-A3 Reproducibility / Privacy / CI-Friction Decision

Date: 2026-09-30
Parent program: #100
P6 issue: #111
Protocol: `docs/development/P6_A3_REPRODUCIBILITY_PRIVACY_FRICTION_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`

## Decision

**P6_A3_REPRODUCIBILITY_CONTEXT_COMPLETE_BOUNDED**

This is a bounded reproducibility and operational-context result for the frozen A3 lanes only. It is not a product ranking, a privacy score, a performance claim, a global telemetry claim, or a semantic-equivalence claim.

## Frozen identity

Both independent repetitions used:

- workload blob: `c5986f1acd8bc79a3945e613acc128d5c8a9d168`;
- A3 workflow blob: `d1ea125ebd8f0230b49d10b48c716d6743706d08`;
- ExecSurface public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`;
- Harden-Runner action: `e14015d583714f6e62063499dc959a02595150a1`;
- cicd-sensor action: `6511eb44c91d71b2b93d71193b1bf2cb18352f66`, bundled sensor `v0.0.45`;
- runner: GitHub-hosted `ubuntu-24.04`, Linux X64;
- observed kernel in both repetitions: `6.17.0-1022-azure`.

Tetragon remained `NOT_TESTABLE_YET` because no immutable executable identity had been frozen. It is not counted as a negative comparative result.

## Independent repetitions

### Repetition 1

- source SHA: `bba3f34eae5402719fe968c885f2a245c582268d`;
- run: `36772847999`;
- conclusion: `success`.

Retained artifacts:

- freeze: `11123723826`, `sha256:535794738a955da315256a229688fca3cf287c27f1369efbcc420d62df9e754c`;
- ExecSurface: `11124298232`, `sha256:1279f96568615cddf823f8d137d6e2926175a08115559b90dcafda9af51e08ed`;
- Harden-Runner context: `11124258258`, `sha256:f81c11a82575a76109e1e2688beca1520271b93c9a3699cacd0614a6d6bcdcf3`;
- cicd-sensor workload context: `11123608854`, `sha256:163f62cbf502c04aff45a151fe55d3cb3422490b5ba0eb26a25a013c388d8fd5`;
- cicd-sensor Runtime Trace predicate: `11123648959`, `sha256:b84cde3c01745a36df627cf812386da5d9ab05f2325ee26ad1cc0d7a31de4c91`;
- cicd-sensor debug: `11123424074`, `sha256:ec3b52dc5c3938b06f707f987386ad3dbdbbced423f366783babc27b10927fc7`.

### Repetition 2

- source SHA: `792026731db32547dae319a7ca584138aa42a417`;
- run: `36772939746`;
- conclusion: `success`.

The second source changed only the dedicated A3 repetition marker and explicitly recorded `criteria_change=none`; the frozen workload and workflow blobs remained identical.

Retained artifacts:

- freeze: `11124546776`, `sha256:92b9562bacce2f989357344aadd4f987131627aca3a5b02a3016623b2930ab1f`;
- ExecSurface: `11123633983`, `sha256:a0250bed9b6b95f8cc25906e86a23cb27b84278703e6f89d727751ec24b1ab49`;
- Harden-Runner context: `11124343319`, `sha256:c7461df700faa7f1b7e22959fe5243740b5b67cfcb2dd8752df31d926ec56650`;
- cicd-sensor workload context: `11124103698`, `sha256:897c763a462ea1a83492adfc7f2fb82328f5eefa607d0184ffcbd7880e65f3a7`;
- cicd-sensor Runtime Trace predicate: `11124746671`, `sha256:c60217f6a4a440f1b59d573767446b7c779756e223dc1e1c8e8a401ebde6e2ad`;
- cicd-sensor debug: `11124766613`, `sha256:593b89305da9a67178c22da2ff0f6705399a11e8503bac1737d02e908a1c811c`.

## Reproducibility findings

### ExecSurface alpha.4

Both repetitions produced the same native outcome vector:

- S0 control: exit `0` / `PASS`;
- S1 child-process expansion: exit `10` / `REVIEW`;
- S2 network destination expansion: exit `2` / fail-closed `ERROR/INCOMPLETE`;
- S3 file-write expansion: exit `10` / `REVIEW`.

No S1-S3 scenario native-PASSed in either repetition.

Both independent baselines produced:

- baseline digest: `sha256:36256b582d59622b3e795fd2470a68cbbc1382e6b7c92ca2a2caeafc98e6d2f1`;
- effects: `113`.

S1 retained explicit `process_exec` drift. S3 retained explicit `file_open` / `file_write` evidence for `$TMP/execsurface-p6/s3.txt`. S2 retained the known fail-closed incompleteness result rather than laundering it into PASS.

### Harden-Runner

The pinned action initialized successfully and S0-S3 executed in both repetitions under `egress-policy: audit`.

The retained operational context was stable across both runs:

- file monitoring remained at its default enabled configuration;
- telemetry was not disabled by this lane;
- native detail surface was job log / Security Insights context;
- runner OS, architecture and kernel were unchanged.

DNS resolution varied naturally between repetitions (`172.66.147.243` vs `104.20.23.154` in the retained per-run context). This variation is retained rather than normalized away.

A3 does not reinterpret the A1 evidence boundary: exact local S1 child lineage and exact S3 sandbox-file evidence were not demonstrated in the retained A1 local output, and A3 does not turn that absence into a product-level inability claim.

### cicd-sensor

The pinned action initialized and finalized successfully in both repetitions. Both runs retained:

- a Runtime Trace predicate;
- debug evidence;
- workload operational context;
- GitHub run/job/workflow identity.

Both predicates contained `example.com` domain evidence and their own distinct GitHub run identity. Debug evidence in both repetitions bound `example.com` to `getent` and `curl` processes with bash / GitHub Runner ancestry.

The predicate's native `passed` value remains cicd-sensor native semantics only and is not mapped to ExecSurface `PASS`.

The A3 debug result-log did not surface the exact S3 `s3.txt` path or the exact S1 child command in the inspected result summary. This is recorded as an A3 retained-evidence observation only. It does not overwrite the richer A1 event evidence and is not generalized into product inability.

## Operational / data-flow facts

### ExecSurface lane

- exact alpha.4 release downloaded and checksum-verified;
- baseline learned once per independent runner;
- baseline/reports retained as local files and GitHub Actions artifacts;
- no explicit `sudo` in the A3 ExecSurface workflow lane;
- no external ExecSurface telemetry endpoint configured by the A3 lane.

The last point is scoped to this lane; it is not a universal telemetry-absence claim.

### Harden-Runner lane

- one pinned action integration step plus the workload steps;
- egress policy configured as `audit`;
- file monitoring remained enabled by default;
- telemetry remained at the action default because `disable-telemetry` was not set;
- job/Security Insights correlation is part of the observed native context;
- no attestation-equivalent machine artifact was established by the A1/A3 bounded evidence.

No privacy quality judgment is assigned to the external service/data flow.

### cicd-sensor lane

- one pinned action integration step plus the workload steps;
- bundled sensor version `v0.0.45`;
- attestation artifact enabled;
- debug artifact enabled;
- HTML report disabled;
- runtime setup created agent/proxy service evidence in the retained debug logs;
- GitHub artifact upload retained predicate/debug evidence.

No claim of non-GitHub endpoint absence is made from silence.

## Discrepancies retained

The following differences are intentionally preserved:

- DNS-selected IPs varied between repetitions;
- cicd-sensor predicates carried different run/job identities as expected;
- cicd-sensor run 2 observed an additional GitHub-hosted service domain not present in run 1;
- artifact digests differ across runs because run identity, timestamps and native evidence vary;
- ExecSurface S2 remained fail-closed incomplete in both repetitions.

None of these differences changed the frozen acceptance criteria.

## Acceptance review

A3 acceptance conditions are satisfied for the authorized lanes:

1. identical workload and workflow blobs across two independent runs — PASS;
2. S0-S3 executed in both runs for ExecSurface, Harden-Runner and cicd-sensor — PASS;
3. ExecSurface false PASS count under S1-S3 across both repetitions — `0`;
4. Harden-Runner and cicd-sensor retained evidence independently inspected in both repetitions — PASS;
5. setup/privilege/data-flow/artifact facts recorded without global generalization — PASS;
6. discrepancies retained rather than normalized away — PASS.

## Scientific boundary

A3 establishes bounded structural reproducibility and operational context only. It does not establish:

- product superiority;
- equivalent observation authority;
- privacy superiority;
- equivalent telemetry behavior;
- event-count superiority;
- performance superiority;
- Tetragon comparative behavior;
- public release readiness.

## Next gate

P6-A4 is optional. It may run only if a proposition-aligned timing methodology can compare equivalent measured boundaries without mixing setup cost, telemetry, canonicalization, and different semantic work into one misleading number. Otherwise P6 must record `NO_VALID_PERFORMANCE_COMPARISON` and proceed to bounded closeout.
