# ExecSurface v0.1.0-alpha.5 — release decision

Status: **INTERNALLY RELEASE-READY — PUBLIC RELEASE BLOCKED BY P8 EXTERNAL-EVIDENCE GATE**

This decision is evidence-bound. It does not authorize publication, tag creation, crates.io publication, stable Action movement, or any claim of independent external validation.

## Frozen product source

- Frozen candidate SHA: `5067200452c174da6bc8d9d7ecf6957ee379f0a2`
- Current public release remains: `v0.1.0-alpha.4`
- Immutable public alpha.4 source: `48e0b9a0707553349e75e97a9dfa096d13f9ab5d`
- Stable Action `AETHERXGLOBAL/execsurface@v0.1` remains on the alpha.4 source.
- No `v0.1.0-alpha.5` public tag is authorized by this decision.

Later governance/workflow commits on `integration/post-alpha4-promotion-candidate` are release-driver evidence only. They do not replace or mutate the frozen product source above.

## Fresh Alpha.5 candidate gates

### A1 — Semantics v3 repaired-admission reproof

- Run: `36898076576` — PASS
- Candidate gate SHA: `067d2e6f6ff1191ccab9bc3ed9ff0a8fe12d4ee6`
- Artifact: `11180725985`
- Artifact digest: `sha256:61e7bfe667c4bd2292b1bbea66c8a1ad1d538d9488a7969327a5c687b13ef95b`

Reproved explicit v2/v3 schema separation, repaired Semantics v3 adversarial corpora, model/baseline/diff/policy regressions, M11 shared-FD fail-closed behavior, formatting, and Clippy `-D warnings`.

### A2 — P4 proposition/backend authority reproof

- Run: `36898309801` — PASS
- Candidate gate SHA: `7a586ae87c0670ee5bdda19bd7b4af2721d7ca28`
- Artifact: `11181360577`
- Artifact digest: `sha256:4934a99d5f93ade6b6620efb9d5e67aee3d7b31f7eeba5e3343306b1ec8ecd3d`

Reproved proposition-scoped authority, S01–S10 sensitivity, cross-proposition rejection, bounded open/rename-delete/connect authority, live ptrace attacks, pinned Tetragon import identity, Semantics v3, and M11.

### A3 — bounded legitimate variance / anti-poisoning

- Run: `36898477456` — PASS
- Candidate gate SHA: `917e7e2e902e87b91d299b590cff1cf7d0c15d81`
- Artifact: `11181076354`
- Artifact digest: `sha256:5f334ed81ee0321b9dd910a79ce98d228835e27ea5f1a4f44c110b61924c82a8`

Passed the exact 12-test legitimate-variance corpus, GCC candidate tests, poisoning/false-PASS falsifier, and semantic-authority reproof. Frequency and similarity remain non-authoritative.

### A4 — attestation/provenance authority

Retained harness failure:
- Run `36898676440` — FAILURE before test jobs because `id-token: read` was invalid workflow permission syntax.

Minimal harness-only correction:
- Commit `1ad4590ea75733d5e35a883fd88731297a69d3fe`
- Scientific assertions and acceptance rules unchanged.

Accepted run:
- Run `36898814464` — PASS
- Artifact: `11181191355`
- Artifact digest: `sha256:820f6ba23882122a343536d46e2ae82cf1a3b72d8a4aa3032bf162812ac1d6fb`

Passed Sigstore reverification, 12/12 semantic-authority attacks, P5-A5 12/12 cross-attestation red-team, pinned SLSA fixture, standards-composition replay, Semantics v3, P4 and M11. Cryptographic validity, signature presence, provenance presence, signer identity and verifier identity do not independently create semantic authority.

### A5 — product usability, packaging and reproducibility

Retained harness failure:
- Run `36899085032` — FAILURE after package/install succeeded because the harness assumed `doctor` must appear in top-level help text.

Minimal harness-only correction:
- Commit `79955e3d85ac23dda1239d5efbdcb2f4761dcc96`
- Static help-text assumption removed; functional `execsurface doctor` remained mandatory.

Accepted run:
- Run `36899408076` — PASS
- Product artifact: `11180418313`
- Product artifact digest: `sha256:05a49a2b90557b7db92523b01615dca3e1d656818ad46c4fc39392033c7b2017`
- Reproducible-binary artifact: `11180364179`
- Reproducible-binary artifact digest: `sha256:a36ef6dae90395cff57b8cea7a0c90f624b01f833750aef77f6c217b7cc43bf3`

Passed full workspace fmt/clippy/tests/docs/release build, clean package/path install, functional doctor, CLI PASS/REVIEW/ERROR contracts, local Action PASS/REVIEW/ERROR, checksum-verified alpha.4 regression, and isolated deterministic release builds.

### A6 — final integrated destructive assault

Retained harness failure:
- Run `36899686411` — FAILURE only in the final governance locator after all scientific/destructive jobs passed. The failing locator used a broad `grep | head` pipeline under `pipefail`.

Minimal locator-only correction:
- Frozen candidate SHA: `5067200452c174da6bc8d9d7ecf6957ee379f0a2`
- Exact P8 and arm64 retained markers used; no scientific test or acceptance boundary changed.

Accepted run:
- Run `36899958419` — PASS
- Final artifact: `11181645256`
- Final artifact digest: `sha256:16f08ae9faa2d15d539fcf32d7fd6848c7199acf9ed522e93a4faf18e2fc04b7`
- Ubuntu 22.04 M12 artifact: `11181297414`
- Digest: `sha256:153c2539df7c67ba0dca559d0d6c39c9c77f99687ece7e92cf1ba12ae06bfdb2`
- Ubuntu 24.04 M12 artifact: `11181645212`
- Digest: `sha256:62225d026707404546d2ff27b6975f92ecaed160c2a14f593760613acce38677`

Passed the frozen 16-invariant A7 governance falsifier, retained-negative-evidence checks, exact arm64/P8 boundaries, Semantics/P4 live attacks, P5 cross-layer attacks, P3/P7 attacks, M12 PATH-TOCTOU/shared-FD suites on Ubuntu 22.04 and 24.04, ptrace/M11 regressions, Action PASS/REVIEW/BLOCK/ERROR, and privacy sentinel.

## Independent frozen-source red-team

The independent red-team checked out and attacked the exact frozen product SHA, not the moving release-driver branch.

- Frozen SHA: `5067200452c174da6bc8d9d7ecf6957ee379f0a2`
- Run: `36900214264` — PASS
- Closeout artifact: `11181895481`
- Digest: `sha256:87eff6e53f77fe514d8785a98960471092c074eeaef842fcb44ac8de9224bb85`
- Ubuntu 22.04 500-run race artifact: `11180579168`
- Digest: `sha256:895c34d558cb69520d224737d10240caf84a813db360a6bbd83f6d5cf678f188`
- Ubuntu 24.04 500-run race artifact: `11180973633`
- Digest: `sha256:61872256ec78a856f1441e6331072ab78c0a105edd6f80529f8abdb8e72bc4c0`

Passed evidence/source binding, proposition ambiguity/default-bypass/migration attacks, P4 cross-proposition/live-object attacks, attestation substitution/verifier-identity laundering, frequency/similarity poisoning, package/install/deterministic-build/tamper checks, and increased 500-run PATH-TOCTOU/shared-FD adversarial replay on both Ubuntu versions.

Bounded result: `ALPHA5_FROZEN_INDEPENDENT_REDTEAM_SURVIVES_BOUNDED_INTERNAL`.

## Release dry-run

A publication-free rehearsal staged version `0.1.0-alpha.5` from the exact frozen source without mutating public refs.

- Run: `36900721300` — PASS
- Artifact: `11180749812`
- Artifact digest: `sha256:beb3cdf48073edef464817f9be14f22275c79ea2acc32c3af6336b1803832267`

Passed:
- isolated alpha.5 version and exact internal dependency migration;
- package topology audit;
- full staged fmt/clippy/workspace tests/docs;
- package file-set audit;
- registry-front package and publish dry-run;
- explicit downstream publication-order constraint;
- two deterministic staged release binaries;
- exact-format `v0.1.0-alpha.5` x86_64 Linux bundle and checksum;
- staged in-toto Statement v1 / SLSA provenance binding to the frozen source;
- post-rehearsal proof that alpha.4 and stable `v0.1` remained unchanged and no alpha.5 public tag was created.

Bounded result: `ALPHA5_RELEASE_DRY_RUN_PASS`.

## P8 external-validation decision

Live review on 2026-10-01 found:
- Issue #114 remains OPEN;
- the issue itself states that substantive current external response is still pending;
- outbound focused requests to William Woodruff and Santiago Torres exist, but no reply from either was present in Gmail at this review;
- currently observed Linux Foundation/OpenSSF, Rust Foundation and Cloudsmith responses are routing/referral/support-routing records, not qualifying technical evidence under P8 by themselves.

P8 cannot close from internal tests, internal red-team, release rehearsal, routing guidance, acknowledgement, praise, or participation.

Therefore the qualifying current external-evidence minimum remains unsatisfied.

## Decision

`ALPHA5_INTERNALLY_RELEASE_READY_BOUNDED — PUBLIC_RELEASE_BLOCKED_P8_EXTERNAL_EVIDENCE`

Consequences:
1. Do **not** create or publish `v0.1.0-alpha.5` yet.
2. Do **not** move stable `v0.1`.
3. Do **not** publish alpha.5 crates yet.
4. Preserve alpha.4 as the current public version.
5. Preserve all failed/harness-negative evidence listed above.
6. Keep P8 open until the preregistered independent external-evidence minimum is actually met.
7. When P8 becomes eligible, revalidate the frozen source/evidence identities before the final public release transaction and then perform post-release verification on the actual public artifacts.

## Scope of confidence

The frozen candidate has passed the strongest bounded internal evidence assembled in this program, including independent-style frozen-source destructive replay. This is not a claim that the software is universally bug-free, that all environments are supported, or that independent external validation has occurred.
