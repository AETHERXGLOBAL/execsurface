# ExecSurface Post-Alpha.4 Development Program

Tracking issue: #100
Development branch: `development/post-alpha4-behavioral-integrity`

## 1. Frozen public baseline

The currently published public release remains:

- release: `v0.1.0-alpha.4`
- immutable release source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- stable Action channel: `AETHERXGLOBAL/execsurface@v0.1`
- public/default correctness-reference observer: native `ptrace`
- support boundary: Linux x86_64

The post-alpha.4 development program MUST NOT silently change the semantics, artifacts, tags, stable Action channel, or published claims of alpha.4.

`main` is the public repository line. Documentation-only maintenance may continue on `main` when required for the published product, but experimental/runtime development from this program is isolated on the dedicated development branch until its promotion gates close.

## 2. Isolation rule

All substantive post-alpha.4 experimental development is performed on:

`development/post-alpha4-behavioral-integrity`

Rules:

1. no experimental runtime/observer/semantics work is committed directly to `main`;
2. no post-alpha.4 experiment may move or retag `v0.1.0-alpha.4`;
3. no post-alpha.4 experiment may move stable `@v0.1` until a separate release/promotion gate explicitly authorizes it;
4. no force-push or history rewrite is authorized for evidence-bearing work;
5. negative evidence, failed experiments, killed hypotheses and regressions are retained;
6. no merge from the development branch to `main` before the declared gate chain passes;
7. promotion is by reviewed PR with explicit evidence references, never by direct history replacement;
8. a future alpha version is created only after release criteria are met; development branch activity is not itself a release claim.

## 3. Product identity / north star

ExecSurface is developed as a **runtime behavioral integrity and verification layer**, not as a generic EDR, malware detector, Falco clone, Tetragon clone or broad host runtime-security platform.

Durable differentiation target:

`backend-agnostic evidence semantics + authority/completeness + canonical accepted surface + deterministic drift + explicit policy + attestable verification result`

Collectors are evidence backends. They are not the product moat.

## 4. Fixed team roles

The following roles remain present through every gate:

### 4.1 Innovation Scientist / Systems Architect

Responsibilities:
- search for non-obvious architectural leverage;
- prefer semantic/evidence advances over feature imitation;
- challenge local optimizations that do not improve strategic differentiation;
- propose integrations/reuse when rebuilding competitor infrastructure would be wasteful.

### 4.2 Anti-Drift / Scientific Integrity Reviewer

Responsibilities:
- prevent scope drift toward generic runtime-security feature races;
- block claim inflation;
- block post-hoc metric or threshold changes;
- enforce separation of evidence, interpretation and product claims;
- prevent reopening closed milestones without a new falsifying result.

### 4.3 Independent Falsifier / Red Team

Responsibilities:
- construct false-PASS, false-REVIEW and false-incompleteness cases;
- attack backend equivalence claims;
- attack lifecycle, concurrency, fd, pathname, network, provenance and attestation assumptions;
- preserve counterexamples as first-class evidence.

## 5. Dynamic specialist pool

Select only the strongest relevant specialists per gate:

- Linux kernel / ptrace / eBPF / BPF-LSM engineer
- runtime evidence semantics / PL / formal methods specialist
- filesystem/process/network identity specialist
- CI/CD supply-chain security architect
- in-toto / SLSA / Sigstore provenance specialist
- reproducibility / benchmarking / performance engineer
- nondeterminism / build-systems specialist
- privacy / data-minimization security engineer
- developer-experience / GitHub Actions / GitLab engineer
- OpenSSF / ecosystem interoperability lead

The team is dynamic; the fixed innovation, anti-drift and falsification roles are mandatory in every material decision.

## 6. Execution protocol

Every development gate follows:

`QUESTION -> HYPOTHESIS -> PREDECLARED TEST -> ADVERSARIAL TEST -> EVIDENCE -> DECISION`

Never:

`DESIRED FEATURE -> IMPLEMENTATION -> METRIC SELECTION -> SUCCESS CLAIM`

For each gate:

1. verify the real remote branch HEAD before work;
2. read all newer commits/issues/evidence before modifying anything;
3. freeze the question, scope, metrics, kill criteria and evidence format;
4. select the dynamic specialists for that gate;
5. run the innovation role and falsifier independently;
6. implement only after the hypothesis/test contract is frozen;
7. preserve raw artifacts and all failed runs;
8. classify result as PASS / FAIL / KILLED / INCOMPLETE / DEFERRED as appropriate;
9. update the tracking issue and evidence documents;
10. do not merge/promote unless the gate explicitly authorizes promotion.

## 7. Development gate chain

### P1 — Finish Issue #85 E4

Issue #85 remains the active eBPF research/value gate and is not bypassed.

Required decision:
- `VALUE_SIGNAL_ESTABLISHED_BOUNDED`, or
- `NO_VALUE_SIGNAL`, or
- `INCOMPLETE_FOR_VALUE_CLAIM`.

No eBPF public authority follows automatically from an E4 result.

### P2 — Execution Surface Semantics v3

Primary moat work:
- proposition-scoped evidence types;
- authority level per proposition;
- completeness/loss per proposition;
- causal lineage contract;
- canonical equivalence rules;
- explicit ambiguous/unsupported states;
- schema/baseline compatibility rules.

Innovation target: **proof-carrying observation records** where every canonical effect states what proposition it supports, which observer capability supports it and at what authority/completeness level.

No cross-backend equivalence without proposition-by-proposition proof.

### P3 — Legitimate variance / nondeterminism model

Build a variance model that is separate from baseline and policy.

Targets:
- temp roots;
- generated paths;
- caches;
- randomized test directories;
- parallel/interleaving-only variance;
- ephemeral identifiers;
- genuine expansion/contraction separation.

Variance learning must never silently authorize new behavior.

### P4 — Backend adapter / authority architecture

Priority:
1. ptrace reference adapter;
2. proposition-scoped native kernel/BPF-LSM research where ptrace authority is known weak;
3. external trace import experiments from mature ecosystems when technically/licensing-wise appropriate;
4. other collectors only for a concrete integrity proposition.

Avoid building a broad host collector merely for parity with competitors.

### P5 — Runtime attestation / provenance interoperability

Prefer standards reuse:
- in-toto Runtime Trace mapping;
- verification-result attestation;
- SLSA provenance binding when available;
- optional Sigstore/GitHub attestation signing/verification.

Bind at minimum:
- command identity;
- source/artifact/workflow identity when available;
- baseline digest;
- current surface digest;
- observer/backend identity;
- capability/completeness state;
- policy digest;
- verdict;
- evidence digest.

### P6 — Competitive falsification harness

Compare factually against relevant competitor classes using pinned scenarios:
- Harden-Runner;
- cicd-sensor;
- Tetragon;
- Falco/Tracee only where propositions materially overlap.

Measure separately:
- setup/privilege friction;
- declared proposition coverage;
- authority/completeness;
- deterministic reproducibility;
- false PASS;
- false REVIEW / false incompleteness;
- overhead;
- privacy/data capture;
- CI integration friction;
- attestation/provenance output;
- failure transparency.

No composite winner score and no marketing ranking.

### P7 — Platform/CI expansion

Only after semantic gates:
1. Linux arm64 if parity is proved;
2. GitLab CI adapter;
3. self-hosted CI packaging;
4. other OS families under separate evidence contracts.

### P8 — External validation

Continue OpenSSF ORBIT engagement under Issue #94.

Count only externally generated evidence as external evidence:
- zero-assistance reproduction;
- substantive architecture criticism;
- no-fit/redundancy finding;
- external counterexample;
- standards/integration guidance;
- independent real-workload report.

No endorsement/adoption claim beyond exact evidence.

## 8. Explicit anti-imitation rules

Do not prioritize:
- threat-intelligence feeds;
- malware signatures;
- generic EDR dashboards;
- broad kernel event collection merely for feature parity;
- opaque ML anomaly scoring before deterministic semantics mature;
- automatic baseline poisoning/approval;
- payload/file-content/secret collection by default;
- superficial platform-count expansion that weakens evidence contracts.

## 9. Promotion and merge policy

The development branch is not merged to `main` as a whole merely because development progressed.

Promotion uses narrowly reviewed PRs with:
- exact source SHA;
- referenced gate result;
- reproducible tests;
- negative evidence retained;
- explicit claims boundary;
- compatibility/migration statement;
- independent red-team review.

A feature may be promoted only if its own gate authorizes it. Failed or research-only work remains isolated.

## 10. Next substantive public-alpha release gate

A release after alpha.4 requires, at minimum:

1. Issue #85 E4 formally closed;
2. Semantics v3 accepted or explicitly deferred with evidence-based rationale;
3. at least one proved improvement in false-review/nondeterminism or proposition authority;
4. no regression of fail-closed semantics;
5. executable attestation/interoperability prototype;
6. zero-assistance public rehearsal PASS;
7. bounded independent red-team review;
8. release-source freeze followed by reproducible build/checksum/provenance verification;
9. only after those gates may stable tags/channels be considered for promotion.

Until then:

`v0.1.0-alpha.4` remains the published public release and post-alpha.4 work remains development/research evidence only.
