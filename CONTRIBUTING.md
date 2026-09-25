# Contributing

ExecSurface prioritizes precision, reproducibility and explicit limitations.

## Architecture-affecting changes

Open an issue/ADR before changing:
- observation semantics,
- canonical effect identity,
- normalization,
- lockfile semantics,
- policy semantics,
- verdict semantics,
- privacy/security boundaries.

## Evidence rules

- no feature without tests,
- no benchmark without reproduction details,
- no security claim from exit code alone,
- no hidden failure,
- preserve negative results.

Every normalization change requires an adversarial counterexample.

## Scope

Changes that turn ExecSurface into an EDR, antivirus, sandbox, generic observability platform or agent framework are out of scope unless governance explicitly changes.
