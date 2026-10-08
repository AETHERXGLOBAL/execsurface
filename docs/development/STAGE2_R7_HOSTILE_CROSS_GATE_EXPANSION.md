# ExecSurface — Stage-2 R7 Hostile Destruction Expansion Record

Date: 2026-10-08  
Parent: `STAGE2_R7_HOSTILE_CROSS_GATE_PROTOCOL.md`  
Parent remediation: #165 / PR #166  
Classification: **IN-GATE DESTRUCTION EXPANSION — NOT PART OF THE ORIGINAL PREREGISTERED FIVE**

## Integrity note

The original R7 protocol froze five output/custody collision attacks.

During execution, the independent destruction team identified two stronger timing/object-identity variants. They were added while R7 was still open. They are recorded here as expansions rather than back-edited into the original preregistration.

No claim is made that these two variants were preregistered.

## R7-X1 — post-target symlink rebinding

A verdict output path can be distinct at preflight, then the target can create that path as a symlink to the already verified baseline before report materialization.

Required invariant:

- preflight may legitimately allow the initially absent/distinct output path;
- target execution may occur;
- post-target identity must be revalidated;
- report materialization must return ERROR rather than follow the new alias;
- the verified baseline bytes must remain intact.

This expansion was added before the first semantic R7 implementation and participated in the retained 0/6 fail-first result.

## R7-X2 — trusted-object rename rebinding

After the first R7 implementation passed the six-case corpus, the destruction team attacked the remaining pathname assumption:

1. baseline is verified at its original path;
2. target renames/moves the verified baseline object itself onto the selected report-output path;
3. original baseline pathname disappears;
4. a postflight check that only re-resolves the original baseline pathname loses the trusted inode;
5. report materialization can then overwrite the same verified object at its new pathname.

Fail-first evidence:

- source: `437896a4e25de3644ed9ad2225000b561db938e5`
- CI run: `37715601621`
- job: `113111234007`
- R7 corpus result: **6 PASS / 1 FAIL**
- failing test: `r7_target_cannot_move_verified_baseline_onto_output_path`

The existing six attacks remained green; only the new trusted-object move attack broke the first implementation.

## Correction boundary

The correction snapshots existing protected object identity before target execution on the supported Linux/Unix boundary using filesystem object identity (device + inode), while retaining path-based preflight and postflight checks.

After target execution, output materialization is rejected if an output resolves to a protected preflight object identity even when that object has moved away from its original pathname.

This does not claim immutability of the target workspace. It only prevents ExecSurface's own report writer from overwriting an object that it previously consumed as a protected baseline/policy input.

## Anti-drift

- Original R7 preregistration remains unchanged.
- Both expansions remain explicit additions discovered during destruction.
- No Alpha.5 history is rewritten.
- No observer or policy semantics were expanded.
