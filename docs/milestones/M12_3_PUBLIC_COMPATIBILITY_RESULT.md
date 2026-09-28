# M12.3 — Public Install / Marketplace / Crates Regression — Result

Date: 2026-09-28
Tracking: #90
Clean integration PR: #92
Branch: `integration/m12-portable-clean`

## Team

Fixed roles retained:
- **Innovative Systems Architect**
- **Anti-Deviation / Skeptical Reviewer**

Dynamic specialists: crates/package engineering, GitHub Actions/Marketplace, release binary integrity, self-service Linux UX.

## Fresh evidence

Workflow: `M12 public install and Marketplace compatibility`
Run: `36458306831`
Conclusion: **success**

### Candidate crates/path path

Passed:
- `cargo package --locked -p execsurface --allow-dirty`;
- `cargo install --locked --path crates/execsurface-cli`;
- candidate `doctor`;
- candidate learn/check PASS path;
- controlled candidate REVIEW with exit code `10`.

### Current public release integrity

Passed:
- release pointer remained exactly `v0.1.0-alpha.3`;
- checksum-verified Linux x86_64 public binary download;
- published binary version check;
- doctor, PASS, and controlled REVIEW behavior.

### Stable Marketplace channel integrity

Passed:
- `AETHERXGLOBAL/execsurface@v0.1` PASS with output verdict `pass` and exit code `0`;
- controlled drift produced verdict `review` and exit code `10`;
- baseline was prepared using the current checksum-verified public release.

## Interpretation

The clean portable candidate remains compatible with the existing self-service packaging surface while the currently published release and stable Marketplace channel remain healthy and unchanged.

This does not publish the candidate or change the stable Action pointer.

No BPF-LSM privilege, daemon, managed kernel, hybrid default, or cross-backend baseline equivalence was introduced.

## M12.3 classification

**`M12_3_PUBLIC_PACKAGING_MARKETPLACE_PASS_CLEAN`**

M12.3 is closed.

Next authorized gate: **M12.4 — Adversarial regression**.
