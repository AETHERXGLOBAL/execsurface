# Active GitHub Workflows

Only durable operational workflows live in this directory. Completed milestone, research, one-shot release, diagnostic and historical validation workflows are preserved under `.github/workflow-archive/` so their evidence is retained without cluttering the active Actions surface.

## Active surface

- `ci.yml` — formatting, linting, tests and lockfile integrity on current development.
- `adversarial-regression.yml` — bounded destructive regression for observer ambiguity, TOCTOU and shared-FD failure classes.
- `ptrace-lifecycle-regression.yml` — bounded ptrace lifecycle regression on the public reference observer.
- `public-consumer-smoke.yml` — validates the currently published binary and stable `v0.1` Action as a real consumer would.
- `technical-evaluation.yml` — reproducible public evaluator path using the current registry release.
- `registry-packaging.yml` — crates.io packaging and dependency-boundary checks.
- `publish-crates.yml` — explicit registry publication of an immutable release tag.
- `release.yml` — immutable-tag release build, attestation and GitHub publication.
- `promote-release.yml` — verifies an explicit release request before immutable tag creation.

Historical workflow source and GitHub Actions run history are retained. Archival is repository hygiene, not evidence deletion or history rewriting.
