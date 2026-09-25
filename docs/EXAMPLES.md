# ExecSurface Command Examples

These are **illustrative command recipes**, not claims that ExecSurface has separately certified every ecosystem.

ExecSurface observes a command on supported Linux x86_64. The project command itself remains your responsibility.

The GitHub Action wraps its command input in `/bin/bash -lc`, so local learn/check examples below use the same wrapper.

## Rust

```bash
execsurface init --command "cargo test --locked" --github-actions

execsurface learn --   /bin/bash -lc 'cargo test --locked'

execsurface check   --policy execsurface-policy.json   -- /bin/bash -lc 'cargo test --locked'
```

The ExecSurface repository's own Rust workflow is continuously tested.

## Python

Illustrative project command:

```bash
execsurface init --command "python -m pytest" --github-actions

execsurface learn --   /bin/bash -lc 'python -m pytest'

execsurface check   --policy execsurface-policy.json   -- /bin/bash -lc 'python -m pytest'
```

This does not imply Python-specific observer semantics. ExecSurface observes the resulting Linux process/file/network effects.

## Node

Illustrative project command:

```bash
execsurface init --command "npm test" --github-actions

execsurface learn --   /bin/bash -lc 'npm test'

execsurface check   --policy execsurface-policy.json   -- /bin/bash -lc 'npm test'
```

This does not imply Node-specific observer semantics.

## CI advice

Choose a command that is stable enough to baseline and narrow enough that drift is reviewable.

If a test command intentionally includes highly nondeterministic network/process behavior, split the workflow or choose a more specific command instead of normalizing meaningful differences away.
