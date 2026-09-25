# M2 — Canonicalization Test Plan

## Positive invariance fixture

Two raw observations representing the same logical behavior vary:

- PIDs/TIDs,
- raw sequence numbers,
- independent event interleaving,
- workspace physical root,
- home physical root,
- temp physical root,
- run-specific temp physical root,
- cache physical root.

Expected: exact equality of the canonical surface.

## Mandatory adversarial fixtures

- executable suffix difference survives run-temp normalization;
- credential-sensitive suffix survives home normalization;
- root component boundary prevents prefix confusion;
- relative path remains unresolved;
- parent traversal remains unresolved;
- duplicate effects deduplicate;
- remote destination port remains significant;
- Linux open read/write access modes remain distinct;
- incomplete observation is rejected;
- duplicate sequence is rejected;
- conflicting semantic roots are rejected.

## Evidence class

Passing CI is **COMPUTATIONAL_EVIDENCE** for implemented fixtures. It is not proof that all possible filesystem/runtime nondeterminism has been normalized correctly.
