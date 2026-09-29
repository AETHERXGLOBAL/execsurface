# P3 V5-C C6 Working State

Date: 2026-09-29
Branch: `development/post-alpha4-behavioral-integrity`
Parent evidence: `V5C_CERTIFICATION_PATH_ESTABLISHED_BOUNDED`

C6 is executing as a separate certificate-aware legacy-guard integration gate.

Immutable rules during execution:
- public alpha.4/main/stable tag remain untouched;
- frozen V5 Stage L remains `P3_INCOMPLETE_EVIDENCE`;
- no old learning sample is replaced;
- raw-v2 `ProcessSpawn.mechanism` remains event-label based;
- internal certificate follows causally correlated clone/clone3 syscall origin;
- the generic `shared_fd_table_ambiguity` guard may be skipped only for a positively complete internal certificate;
- any absent/incomplete/contradictory certificate remains fail-closed;
- existing observer warnings are never cleared by certification;
- resource/event loss, lifecycle loss, file-identity failure, and other incompleteness remain independent blockers;
- no threshold, adversarial fixture, or regression test may be weakened to obtain a PASS.

Preserved C6 plumbing failures before semantic execution:
- `36564238281`: patch driver did not match the current ptrace handoff text; no semantic test executed.
- `36564356709`: handoff patch matched, but the negative-test insertion anchor was stale; no semantic test executed.

Both are tooling failures only and remain part of history. C6 scientific acceptance criteria were unchanged.
