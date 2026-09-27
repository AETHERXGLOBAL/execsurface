# GitHub Marketplace Listing Copy — ExecSurface

Use this copy when publishing the existing public GitHub Action in GitHub Marketplace.

## Name

ExecSurface — Runtime Drift Detection

## Short description

Detect runtime execution-surface drift in CI and developer workflows using an explicit baseline and policy.

## Primary category

Security

## Secondary category

Utilities

## Long description

ExecSurface learns an accepted runtime execution surface for a command and later reports observed behavior that appeared, disappeared, or changed.

It is designed for software repositories, CI pipelines, dependency updates, developer tools, and AI tooling where source review alone does not show every runtime effect.

The public alpha supports Linux x86_64 and uses the native ptrace observer as its correctness-reference backend.

Typical workflow:

1. install ExecSurface;
2. run `execsurface doctor`;
3. generate starter policy / GitHub Action files with `execsurface init`;
4. explicitly learn a reviewed baseline;
5. run later checks and inspect PASS / REVIEW / BLOCK / ERROR evidence.

No signup, API key, meeting, or prior approval from AETHER X is required.

ExecSurface does not prove software safety and is not an antivirus, EDR, malware detector, or sandbox.

## Stable Action reference

```yaml
uses: AETHERXGLOBAL/execsurface@v0.1
```

## Evaluation link

`docs/INDEPENDENT_EVALUATION.md`
