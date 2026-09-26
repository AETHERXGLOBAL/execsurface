# M8.3c — Integration Note

Date: 2026-09-26
Status: **REFERENCE ONLY**
Tracking: #37

The authoritative M8.3c implementation, evidence, capability boundary, closure decision, and M8.3c2 entry conditions are maintained in:

`docs/milestones/M8_3C_COLLECTOR.md`

This file is intentionally only a pointer so M8.3c cannot acquire two competing sources of truth.

The governing boundaries remain unchanged:

- ptrace is the default correctness reference;
- the libbpf observer is experimental and opt-in;
- automatic backend selection is not authorized;
- eBPF-derived PASS is not authorized;
- cross-backend parity remains open for M8.5.
