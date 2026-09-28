# M10.3B — Attempt 2 Parser Review

Date: 2026-09-28
Tracking: #88
Branch: `research/m10-hybrid-observer-m103b`
Workflow run: `36428146126`
Source SHA: `18d3be87dbad0fa28264eeaa1cbb368ab18b67a0`
Artifact: `10972132687`
Artifact digest: `sha256:31dbde1025e881e204d7f66dea60c65259f0ff4f4236398a96ebc4c3b2bc74ab`
Status: **RAW CAPABILITY OBSERVED / FROZEN CLASSIFIER OUTPUT INVALIDATED BY PARSER ARTIFACT**

## Raw evaluator output

The evaluator emitted:

`M10_3B_LSM_ACTIVATION_BLOCKED`

This output is not accepted as the scientific classification.

## Raw serial evidence

The repaired harness successfully booted the intentionally installed kernel:

`6.8.0-142-generic`

The guest reached both frozen completion markers and exited normally. Its serial output included:

- `M10_3B_CMDLINE=console=ttyS0 panic=-1 lsm=bpf`
- `M10_3B_LSM_LIST=lockdown,capability,bpfM10_3B_BTF_READABLE=true`
- `M10_3B_PROBE_COMPLETE`

The concatenation occurred because `/sys/kernel/security/lsm` did not supply a trailing newline and the init script used `echo -n` before `cat`. The following BTF marker was therefore appended to the same line.

## Interpretation

The raw serial transcript contains `bpf` in the active LSM value and proves that the boot-controlled approach reached a guest with BPF LSM active. However, the preregistered machine-readable classifier did not parse that transcript correctly, so M10.3B is not closed from attempt 2.

This is a **parser/harness artifact**, not evidence of LSM activation failure.

## Corrective action allowed

A third attempt may only repair record framing:

- read the LSM list into a shell variable;
- emit it with a terminating newline as one marker line;
- retain the same kernel prerequisites, QEMU/TCG boot model, `lsm=bpf` command line, classification rules and product non-authorizations.

No scientific success criterion may change.
