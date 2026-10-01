# Repository Archive

This directory stores superseded operational state, prerelease decisions, engagement snapshots and historically important material that should remain reproducible but should not be mistaken for current product state.

## Rules

- Archive movement never rewrites Git history.
- Negative evidence and failed results are preserved.
- Current product facts belong in `docs/STATUS.md`, `README.md` and the latest release document.
- Historical documents retain the wording and assumptions that were true when written.
- Archived evidence remains evidence; archive location only means it is no longer the authoritative current-state surface.

Completed one-shot GitHub workflow definitions may be moved to `.github/workflow-archive/` so they remain inspectable without cluttering the active Actions surface.
