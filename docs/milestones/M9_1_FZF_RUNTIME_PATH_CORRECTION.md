# M9.1 — Frozen fzf Runtime Path Correction

Date: 2026-09-26
Status: **FROZEN BEFORE CORRECTED EXECUTION**
Class: `ZERO_CONTACT_EXTERNAL_REPRO`
Parent workload freeze: `b020369a6a6861cc8b530fc9e63a7db428999466`

## Preserved prior evidence

The original frozen Batch B fzf runtime case is not rewritten or deleted.

- First attempt: run `36279671507`, artifact `10918905123`, evidence `M9-ZC-05-FZF-RUNTIME-001`, `PARTIAL` because the shallow external checkout omitted tags required by upstream `make` to derive version metadata.
- Second attempt: run `36279861164`, artifact `10918252014`, evidence `M9-ZC-05-FZF-RUNTIME-002`, `PARTIAL`. Full git metadata fixed the build and `make` succeeded, but the frozen runtime command referenced `./target/fzf`, which does not exist for the pinned upstream Makefile on x86_64 Linux.

## Upstream-grounded correction

At pinned revision `b1be3a8be1b833ce5b92fbbac11637643d60a046`, the upstream Makefile maps x86_64 to `BINARY64 = fzf-linux_amd64` and its default target is `target/$(BINARY)`.

Therefore the corrected frozen runtime command is:

```sh
printf 'alpha\nbeta\ngamma\n' | ./target/fzf-linux_amd64 --filter=beta --select-1 --exit-0 >/dev/null
```

The build command remains exactly:

```sh
make
```

No ExecSurface semantics, normalization, thresholds, privacy rules, project source, or acceptance criteria change. The only correction is the executable path, grounded in the pinned upstream Makefile and frozen before this corrected execution.

## Corrected evidence ID

`M9-ZC-05-FZF-RUNTIME-003`

Acceptance remains: direct PASS; doctor PASS; baseline PASS; two unchanged zero-finding PASS checks; complete observation; frozen 3+15 performance protocol; schema-valid evidence; failures preserved as PARTIAL.
