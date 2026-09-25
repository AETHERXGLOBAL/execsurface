# M6.5 — Schema Migration

M6.5 intentionally changes execution-surface meaning and therefore versions the affected contracts instead of silently rewriting v1.

## Versions

- raw observation schema: v2
- canonical surface schema: v2
- normalization profile: v2
- baseline lock schema: v2
- baseline digest format: v2
- diff report schema: v2
- policy schema: v2 (v1 remains accepted for v1-compatible matchers)
- verdict report schema: v2

## Baseline migration

Existing lock schema v1 files are **not reinterpreted** as v2.

Relearn the baseline with the hardened observer:

```bash
execsurface learn -- COMMAND [ARGS...]
```

This is deliberate because v2 adds actual fd-attributed read/write effects, execution chains, openat2 resolve metadata and kernel-fd resolution state.

## Policy migration

Policy v1 remains accepted when it uses only v1 effect kinds.

To match actual fd-attributed reads or writes, use policy schema v2 and:

- `file_read`
- `file_write`

Policy v1 that attempts to use these v2 matchers is rejected explicitly.

## Frozen schemas

Published v1 schema files remain unchanged:

- `schemas/execsurface-lock-v1.schema.json`
- `schemas/execsurface-policy-v1.schema.json`

New contracts are published as:

- `schemas/execsurface-lock-v2.schema.json`
- `schemas/execsurface-policy-v2.schema.json`
