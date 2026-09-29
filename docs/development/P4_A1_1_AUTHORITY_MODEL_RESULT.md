# ExecSurface — P4-A1.1 Authority Model Result

Date: 2026-09-29
Tracking: #108 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED — P4_A1_1_AUTHORITY_MODEL_PASS_RESEARCH_ONLY**

## Canonical implementation

`experiments/p4-backend-authority`

The implementation directly reuses `execsurface_model::semantics_v3` proof-carrying observations and proof requirements. The previously started parallel proposition-adapter experiment is retired from the working tree; its history and failed CI remain retained as engineering evidence.

## Accepted execution

- source: `2d3afad62c1140c5bdf9ff77e7636dc6ed22fa95`
- workflow: `36581599017`
- job: `109450909102`
- evidence artifact: `11038744982`
- artifact SHA-256: `sha256:827b07fa39774cf25400a5e4aa9a6153fdbd3ea204da59f85552b269eca1594d`
- runner: GitHub-hosted Ubuntu 24.04
- Rust: 1.90.0

## Results

- A0/A1 frozen-boundary verification: PASS
- isolated dependency lock generation: PASS
- `rustfmt --check`: PASS
- `clippy -D warnings`: PASS
- canonical authority-model falsification tests: **8/8 PASS**
- Semantics-v3 model reproof: **5/5 PASS**
- public M11 shared-FD fail-closed regression: **6/6 PASS**

Authority tests prove only the declared type/model properties:
- unsupported authority cannot become complete;
- lost/ambiguous authority cannot become complete;
- pathname-attempt evidence cannot satisfy object-identity proof requirements;
- backend name cannot upgrade weak evidence;
- the same proposition can have asymmetric authority across backends;
- different propositions are never treated as equivalent;
- derived authority requires an explicit derivation identity;
- deterministic serialization remains stable.

## Retained negative engineering evidence

Canonical first run `36580119977` stopped at `rustfmt` before clippy/tests. It remains retained and is not relabeled.

A later duplicate experiment/workflow path also failed before scientific tests and has been retired by the canonicalization note. Its history remains retained; no result from it is used for A1 acceptance.

## Boundary

This result closes **A1.1 only**. It does not prove that current ptrace runtime evidence has been mapped correctly into every frozen proposition.

Next authorized stage:
**P4-A1.2 — map actual existing ptrace evidence into the canonical proposition/authority model, research-only, followed by A1.3 adversarial mapping tests and A1.4 public anti-drift regression.**

No eBPF/BPF-LSM or external backend implementation is authorized yet. Public alpha.4 remains unchanged.
