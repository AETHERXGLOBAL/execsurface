# M12.7 Failure 003 — Undisclosed Conservative Incompleteness

Date: 2026-09-28
Tracking: #90
Branch: `integration/m12-portable-clean`

## Classification

`M12_7_RELEASE_BLOCKED_CLAIMS`

This is a real release-documentation blocker, not a harness-only failure.

## Failed run

- Workflow: `M12.7 Independent Internal Release Red-Team`
- Run: `36468075481`
- Job: `109083080171`
- Candidate SHA: `064f32f140b01ecfe9aa99a0cd2e4b4a51499379`
- Conclusion: **failure**

## What survived before the blocker

Before the disclosure check failed, the candidate passed:

- public-line/provenance freeze;
- runtime-source freeze after the M12.6 semantic candidate;
- diff-scope kill check;
- adversarial fixture build;
- rustfmt and full-workspace Clippy `-D warnings`;
- shared-FD fail-closed semantics;
- full M12 adversarial replay;
- ptrace regressions;
- frozen baseline and CLI compatibility;
- full workspace regression;
- privilege / hybrid non-promotion sentinels;
- general claim-boundary sentinels.

## Finding

The portable M12 runtime deliberately treats observed clone-based concurrency as a shared-fd lifecycle ambiguity because raw observation v2 does not retain enough `CLONE_FILES` detail to prove exact fd-table sharing.

This is conservative: it prevents the known false-completeness class, but it can also mark some clone/thread concurrency incomplete even when exact fd-table sharing is not proven.

The README and prospective alpha.4 release notes did not disclose that false-incompleteness tradeoff explicitly enough for a public release.

## Why this blocks release

A developer can receive incomplete/non-PASS-eligible evidence in a concurrent program even though the observer has not established actual fd-table sharing. Shipping that behavior without an explicit public limitation would overstate the precision of the completeness classification.

The runtime behavior itself is intentional and already adversarially validated. The blocker is the missing public disclosure of its conservative blast radius.

## Authorized remediation

Documentation-only remediation:

- README must state that the portable ptrace guard may conservatively mark some clone/thread concurrency incomplete even when exact fd-table sharing is not proven;
- release notes must state the same and explain that this is a false-incompleteness tradeoff used to prevent a known false-completeness class;
- do not claim exact shared-FD repair;
- do not weaken the fail-closed runtime guard.

After remediation, rerun the complete M12.7 red-team from the start. The failed run remains part of the evidence record.