# M0 — Architecture Gate

Issue: #1

Date: 2026-09-25

Verdict: **M0 ACCEPTED**

M1 is architecturally authorized after this gate, but M1 implementation is not part of this commit.

## Dynamic M0 team

Task-specific review roles:
- Linux Systems / ptrace reviewer
- Rust systems architecture reviewer
- Software Supply Chain Security reviewer
- DevSecOps / CI reviewer
- Reproducibility engineer
- Threat-model specialist
- Dynamic-analysis researcher
- Developer Experience reviewer
- independent adversarial security reviewer

Fixed:
- **Innovation Scientist / Architect**
- **Deviation Prevention / Scientific Integrity**

## Gate checks

### Is ExecSurface merely a tracer?

**NO — PROVED BY ARCHITECTURE.**

Tracing is backend input. Product semantics are normalize/canonicalize/lock/diff/policy/verdict/evidence.

### Are baseline and policy distinct?

**YES — PROVED BY ARCHITECTURE.**

### Can observer failure pass?

**NO — PROVED BY VERDICT CONTRACT.**

Completeness-fatal failure yields ERROR.

### Is M1 backend evidence-based?

**PARTIAL EVIDENCE / ACCEPTED.**

strace is selected as reference implementation to minimize observer implementation risk while validating the higher-level abstraction. Completeness/performance remain empirical.

### Can normalization silently hide meaningful drift?

**NO BY CONTRACT; IMPLEMENTATION OPEN.**

Every rule requires rationale, positive fixture and adversarial counterexample.

### Does the default model require secrets?

**NO — PROVED BY DESIGN.**

## Open carry-forward

- minimum supported strace version: **OPEN M1**
- parser completeness matrix: **OPEN M1**
- fd-to-path attribution: **OPEN M1/M2**
- symlink race semantics: **OPEN M1/M2**
- hostname attribution: **OPEN / deferred**
- temp-token algorithm: **OPEN M2**
- deterministic JSON test vectors: **OPEN before M3**
- performance overhead: **OPEN**
- novelty/patentability: **OPEN**

## Killed approaches

- procfs as sole observer: **KILLED**
- eBPF-first M1: **KILLED FOR M1 ONLY**
- AI classifier in MVP: **KILLED**
- baseline/policy merger: **KILLED**
- "no drift = safe": **KILLED**

## M1 hard boundary

M1 may implement only a minimal Linux observer.

M1 may not:
- implement baseline learning,
- implement policy verdict logic,
- claim trace completeness,
- claim low overhead,
- claim malware detection,
- add macOS/Windows,
- add eBPF without a separate evidence-backed ADR.
