# M1 Privacy Gate

Date: 2026-09-25

Verdict: **STRACE PRODUCTION INGESTION REJECTED; NATIVE PTRACE AUTHORIZED**

## Red-team question

Can the selected M1 observer satisfy the repository rule "observe metadata, not secrets" at collection time?

## Result

The original strace-ingestion proposal failed the gate because normal decoded trace output can include string syscall arguments, including child argv.

Sanitizing after observation is not an adequate default privacy boundary.

## Corrective action

- Preserve ADR-0001 as historical evidence.
- Add ADR-0002; do not rewrite M0 history.
- Implement the narrow M1 observer directly on Linux ptrace.
- Read only explicit metadata fields required by the event contract.
- Add a secret-sentinel non-capture integration test.

## Scope guard

This correction does not authorize M2 canonicalization, M3 baseline, M4 diff, M5 policy, eBPF, sandboxing, EDR behavior, or malware detection.

## Status

- negative result preserved: **PROVED**
- architecture deviation prevented: **PROVED**
- corrected implementation: **OPEN**
