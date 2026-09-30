# P5-A5 rerun marker after Clippy-only correction

This marker triggers the unchanged frozen P5-A5 gate after the static-only `clone_on_copy` correction.

- Scientific counterexample run `36755962689` remains retained: 8/12, four surviving attacks.
- Static-only run `36757944902` remains retained; the frozen 12-test corpus did not execute.
- Clippy-only correction source: `e420fc4edef2876f2278c0cc431c974110669530`.
- No attack, threshold, public crate, alpha.4 semantic, or predecessor acceptance criterion is changed by this marker.
