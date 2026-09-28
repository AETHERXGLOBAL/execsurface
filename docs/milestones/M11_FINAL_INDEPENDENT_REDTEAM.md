# M11 — Final Independent Internal Red-Team

Date: 2026-09-28
Tracking: #89
Branch: `research/m11-authority-aware-integration`
Review type: **independent internal role-based release red-team**
Status: **COMPLETE**

## Purpose

Attempt to invalidate the strongest positive M11 release argument after all implementation gates, public-compatibility checks, and preregistered performance measurements were available.

This is not an external review. It is a deliberately separated internal skeptical review whose job is to reject promotion if the evidence does not support it.

## Review roles

Fixed roles retained:

- **Innovative Systems Architect** — challenged whether the architecture could preserve portable self-service while exposing stronger authority only where the environment supports it.
- **Anti-Deviation / Skeptical Reviewer** — attempted to reject the release argument using known counterexamples, incomplete measurements, hidden compatibility changes, authority inflation, silent fallback, or performance overclaim.

Additional specialists for this final gate:

- Linux ptrace / process-lifecycle semantics
- BPF-LSM and kernel evidence authority
- baseline and cross-backend comparability
- Rust integration / test review
- GitHub Actions and Marketplace compatibility
- benchmark methodology / shared-runner noise
- privacy / data-minimization review

## Promotion argument under attack

Candidate argument:

> The isolated M11 portable hardening work is suitable to advance to a separate merge/release review, while the hybrid backend/default remains deferred.

The red-team did **not** test or endorse a stronger statement such as production readiness, universal Linux coverage, hybrid default readiness, or end-to-end BPF performance readiness.

## Attack 1 — Did M11 erase the ptrace counterexamples?

**Result: NO.**

The M10 counterexamples remain architectural constraints:

- syscall-entry pathname is not treated as kernel-object identity;
- shared-FD lifecycle ambiguity can invalidate ptrace completeness.

M11 hardening responds by lowering authority or failing closed rather than redefining the counterexamples away.

**Red-team disposition:** no blocker for the bounded portable candidate; counterexamples remain blockers against stronger authority claims.

## Attack 2 — Can ptrace pathname evidence still masquerade as kernel-object truth?

**Result: NO in the tested M11 authority model.**

M11.2 explicitly separates pathname argument evidence from kernel-object identity authority. Exact proposition requirements prevent a userspace pointer/path observation from satisfying stronger object-identity propositions.

**Red-team disposition:** pass.

## Attack 3 — Can shared-FD ambiguity still produce false `complete=true` / PASS eligibility?

**Result: NOT in the tested M11.3 path.**

Clone-based concurrency for which raw-v2 cannot certify FD-sharing lifecycle is marked incomplete through `shared_fd_table_ambiguity` / `IncompleteAmbiguity`, and that classification is not PASS-eligible.

This is deliberately conservative: it may reduce PASS eligibility rather than pretend the observer saw more than it can certify.

**Red-team disposition:** pass for fail-closed behavior; no claim of perfect FD reconstruction.

## Attack 4 — Can hybrid candidate evidence be promoted into success evidence?

**Result: NO in the explicit M11 adapter contract.**

Candidate evidence and completion/success evidence remain distinct propositions. Missing exec-success or connect-completion confirmation prevents the explicit hybrid contract from being accepted for those success propositions.

**Red-team disposition:** pass.

## Attack 5 — Can producer loss, malformed records, wrong-session records, or unknown-role records remain PASS-authoritative?

**Result: NO in the tested M11.6 health mapping.**

The tested cases are non-complete / non-PASS. Producer-loss accounting remains mandatory for the explicit hybrid contract.

**Red-team disposition:** pass, bounded to tested loss/session paths.

## Attack 6 — Did M11 silently make ptrace and hybrid baselines interchangeable?

**Result: NO.**

Cross-backend baseline reuse remains rejected unless exact proposition-level semantic equivalence is separately established. M11.5 did not infer equivalence from similar event names or similar outputs.

Frozen baseline-v2 compatibility remained green.

**Red-team disposition:** pass.

## Attack 7 — Did M11 add hidden BPF privilege or hybrid auto-selection to the public user path?

**Result: NO.**

M11.7 proved that:

- the Marketplace surface files stayed identical to frozen public `main`;
- the normal M11 CLI path worked on a standard GitHub-hosted runner without BPF-LSM setup;
- the published release path remained usable;
- `AETHERXGLOBAL/execsurface@v0.1` preserved PASS/REVIEW and exit-code behavior;
- explicit hybrid prerequisites did not become public defaults.

Primary M11.7 run: `36450566559` — success.
A later compatibility rerun after the hybrid adapter benchmark code was added also succeeded: `36451185462`.

**Red-team disposition:** pass for the tested public paths.

## Attack 8 — Is the portable path hiding a material performance regression?

**Result: NO gate-blocking regression in the preregistered paired run.**

M11.8 compared frozen `main` and the research branch on the same GitHub-hosted runner. The threshold was preregistered before results: for each nontrivial workload, a material signal required both >1.25x observed-median ratio and >=1,000 microseconds absolute increase; two nontrivial material signals were required to block on performance.

Primary run: `36451329198` — success.
Evaluator classification: `PORTABLE_NO_GATE_BLOCKING_REGRESSION`.
Material regression signal count: `0`.

Observed medians:

| workload | main | M11 | M11/main | delta | material signal |
| --- | ---: | ---: | ---: | ---: | --- |
| `/bin/true` (descriptive only) | 1,998 us | 2,003 us | 1.0025x | +5 us | no |
| descendant spawn | 6,002 us | 5,950 us | 0.9913x | -52 us | no |
| burst-128 | 77,168 us | 73,825 us | 0.9567x | -3,343 us | no |

The burst workload contained a large `main` p95 outlier on the shared runner. Therefore the review explicitly rejects the statement that M11 is universally faster. The supported conclusion is only **no gate-blocking regression in this paired run**.

**Red-team disposition:** pass for the preregistered bounded release gate; broad performance claims rejected.

## Attack 9 — Does the hybrid adapter benchmark prove hybrid runtime performance?

**Result: NO. This is a promotion blocker for hybrid default status.**

The bounded adapter-only benchmark measured:

- 21 batches × 10,000 iterations;
- median: 213 ns/iteration;
- p95: 217 ns/iteration;
- min: 212 ns;
- max: 222 ns.

But the measured code is only the authority/capability adapter. It does **not** include a live BPF-LSM collector, transport, success correlation, session isolation, producer-loss path, normalization, or end-to-end product execution.

Therefore these numbers cannot authorize hybrid default or product promotion.

**Red-team disposition:** hybrid default remains blocked/deferred until a real end-to-end managed BPF-LSM implementation and benchmark exist.

## Attack 10 — Did M11 mutate the public product before review closure?

**Result: NO.**

Frozen public `main` remained `db11761e2d75ebca7d4458dc39094f4aed6a26bb` through the final measurement. No public release, tag, Marketplace publication, or default backend was changed by M11.

**Red-team disposition:** pass.

## Evidence integrity

M11.8 primary artifacts:

- portable artifact: `m11-8-portable-performance-36451329198-1`
  - artifact id: `10984116733`
  - SHA-256: `4a4b89c00c915cd8a3a1e05789e0d998104a73fa8d753b18c4a87130005f7f44`
- hybrid adapter artifact: `m11-8-hybrid-adapter-36451329198-1`
  - artifact id: `10983073482`
  - SHA-256: `bcad63bc19c1758e3692d70627146548b2bd8d86ac8d666819d988773613f0be`

Latest semantic authority CI covering the hybrid adapter code before the performance-workflow-only commit:

- run `36451185374`
- conclusion: success
- tested SHA: `1ce71ff06d7030631aa68e9710f402b33888fd72`

The M11.8 workflow commit changed performance CI methodology only and its own run completed successfully at SHA `8c84fbd6f8b4283816126cd15b359e851894ad69`.

## Final skeptical judgment

The red-team **could not invalidate** the bounded statement that the portable M11 hardening branch is eligible to advance to a separate merge/release review.

The red-team **did invalidate any attempt** to use M11 as evidence that:

- hybrid is ready to become the default backend;
- the live hybrid collector has acceptable end-to-end performance;
- ExecSurface has universal kernel-event completeness;
- the product is production-ready;
- M11 is universally faster than the current public release.

## Red-team classification

**`M11_RELEASE_ARGUMENT_SURVIVES_WITH_HYBRID_PROMOTION_BLOCKED`**

This supports the M11.8 final classification `M11_PORTABLE_RELEASE_CANDIDATE_HYBRID_DEFAULT_DEFERRED`, subject to a separate merge/release review. It does not authorize merging or publishing by itself.
