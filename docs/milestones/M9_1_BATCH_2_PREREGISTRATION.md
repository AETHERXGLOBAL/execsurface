# M9.1 Batch 2 — Pre-Registration

Status: **FROZEN BEFORE EXECUTION**

Purpose: test whether the large unchanged-rerun noise seen in Batch 1 is primarily a property of ephemeral build/test workloads rather than ordinary user-like execution.

No Batch 2 outcome was observed before this protocol was committed.

## Governance

- Independence class: `ZERO_CONTACT_EXTERNAL_REPRO`.
- These cases do **not** count as independent adoption.
- External repositories are pinned and unmodified.
- Build/setup occurs before the observed command and is not part of the learned runtime surface.
- No normalization, ignore rules, baseline semantic changes, or issue-#59 candidate fixes are allowed in this batch.
- The current ptrace backend remains the authority surface.
- Existing M9 evidence schema and validator remain unchanged.
- Two unchanged checks are required after learning.
- A controlled drift command is required for any case that reaches two unchanged PASS results.
- Performance protocol remains exactly 3 warmups per mode and 15 alternating measured samples per mode.

## Frozen cases

### ZC-04 — casey/just user-like CLI

Repository: `casey/just`

Revision: `5d5742cbcc50f19c99c356bc7e085acaa5f4665d`

Setup only, outside observation:

```bash
cargo +1.90.0 build --locked --release
```

Observed command:

```bash
./target/release/just --list >/dev/null
```

Rationale: same pinned external project as Batch 1, but a normal read-mostly CLI operation instead of its full Cargo test suite.

Controlled drift, only if two unchanged checks PASS:

```bash
./target/release/just --list >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null
```

Expected controlled-drift requirement: added `sha256sum` process execution must be surfaced as REVIEW or stronger; it must not be silently accepted.

### ZC-05 — junegunn/fzf deterministic filter CLI

Repository: `junegunn/fzf`

Revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`

Setup only, outside observation:

```bash
go build -o "$RUNNER_TEMP/fzf" .
```

Observed command:

```bash
/bin/bash -lc "printf 'alpha\nbeta\ngamma\n' | '$RUNNER_TEMP/fzf' --filter beta >/dev/null"
```

Rationale: same pinned external project as Batch 1, but the compiled user-facing binary receives deterministic input rather than invoking the Go build/test graph during observation.

Controlled drift, only if two unchanged checks PASS:

```bash
/bin/bash -lc "printf 'alpha\nbeta\ngamma\n' | '$RUNNER_TEMP/fzf' --filter beta >/dev/null; /usr/bin/sha256sum '$RUNNER_TEMP/fzf' >/dev/null"
```

Expected controlled-drift requirement: added `sha256sum` process execution must remain visible.

### ZC-06 — BurntSushi/ripgrep user-like file listing

Repository: `BurntSushi/ripgrep`

Revision: `3fce3b5bb0236da2df6d99672afb8a719642eca7`

Toolchain for setup: Rust `1.96.0`, because Batch 1 direct-only evidence established that the pinned revision's dependency graph no longer accepts Rust 1.90.0.

Setup only, outside observation:

```bash
cargo +1.96.0 build --locked --release
```

Observed command:

```bash
./target/release/rg --files . >/dev/null
```

Rationale: recover the same pinned project under its actual toolchain requirement, while observing a normal read-only CLI operation rather than the build/test command.

Controlled drift, only if two unchanged checks PASS:

```bash
./target/release/rg --files . >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null
```

Expected controlled-drift requirement: added `sha256sum` execution remains visible.

### ZC-07 — sharkdp/fd stable comparator

Repository: `sharkdp/fd`

Revision: `ce97e473ebaec49697c07daa50a7bc2b32f713d2`

Setup only, outside observation:

```bash
cargo +1.90.0 build --locked --release --all-features
```

Observed command:

```bash
./target/release/fd --hidden --type f --exclude target . >/dev/null
```

Rationale: a pinned external comparator with prior M7 evidence, executed here under the same frozen M9.1 protocol as the other user-like cases.

Controlled drift, only if two unchanged checks PASS:

```bash
./target/release/fd --hidden --type f --exclude target . >/dev/null; /usr/bin/sha256sum Cargo.toml >/dev/null
```

Expected controlled-drift requirement: added `sha256sum` execution remains visible.

## Failure handling

- If the direct command fails, stop that case fail-closed and record `PARTIAL`.
- If observer execution fails, preserve the exact failure and do not substitute another backend.
- If unchanged checks return REVIEW/BLOCK, preserve the findings and do not tune the baseline after observation.
- If a controlled-drift run fails to surface the injected extra process, mark the case as a suspected false negative and stop promotion claims.
- If performance crosses M6.5, record it; do not silently authorize eBPF.

## Interpretation boundary

Batch 2 can test the narrower hypothesis that ordinary compiled CLI operations are more baseline-stable than build/test graphs. It cannot establish universal stability, adoption, or broad performance claims.
