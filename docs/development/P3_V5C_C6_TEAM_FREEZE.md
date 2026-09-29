# P3 V5-C C6 Team Freeze

Date: 2026-09-29
Branch: `development/post-alpha4-behavioral-integrity`

Fixed independent roles:
1. Innovation Scientist / Systems Architect — seek the smallest proof-bearing integration and alternative designs; cannot waive falsification gates.
2. Anti-Drift / Scientific Integrity Reviewer — owns frozen evidence, compatibility boundary, and claim discipline; can veto gate skipping.
3. Independent Falsifier / Red Team — owns false-completeness attacks and counterexamples; does not optimize implementation metrics.
4. Independent Critical-Milestone Reviewer — reviews C6 evidence only after implementation/falsification results exist and may classify the gate PASS_BOUNDED / FAIL / INCOMPLETE.

Dynamic implementation/review specialists:
- Linux ptrace child-creation semantics (`clone`, `clone3`, fork/vfork routing)
- fd-table lifecycle (`CLONE_FILES`, exec/CLOEXEC, dup/close/reuse)
- Rust systems/state-machine correctness
- observer completeness and proof-carrying evidence
- Go build concurrency / real-workload reproducibility
- compatibility/regression engineering

Separation rule:
- implementers may not relax a red-team fixture;
- red team may not redefine acceptance after seeing results;
- anti-drift reviewer blocks public-v2 reinterpretation;
- critical reviewer bases closure only on preregistered evidence and retained failures.
