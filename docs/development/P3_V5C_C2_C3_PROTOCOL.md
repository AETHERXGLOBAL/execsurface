# ExecSurface — P3 V5-C C2/C3 Certificate Refinement & Adversarial Protocol

Date: 2026-09-29
Tracking: #104
Parent result: `docs/development/P3_V5C_C1_RESULT.md`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **PREREGISTERED — CERTIFICATE MUST FOLLOW SYSCALL ORIGIN**

## Objective

Refine the research-only internal clone/fd-table certificate so its truth follows the originating child-creation syscall evidence, not the ptrace event label, then attempt to falsify it with Linux event-routing counterexamples.

Public `v0.1.0-alpha.4`, public raw/canonical/baseline v2 bytes and the public `shared_fd_table_ambiguity` guard remain unchanged.

## Team

Fixed:
1. Innovation Scientist / Systems Architect
2. Anti-Drift / Scientific Integrity Reviewer
3. Independent Falsifier / Red Team

Dynamic C2/C3 specialists:
- Linux ptrace child-creation event semantics
- clone/clone3 + exit-signal semantics
- fd-table / `CLONE_FILES` lifecycle
- exec/de-threading and close-on-exec semantics
- Rust state-machine correctness
- adversarial concurrency / reproducibility

## C2 certificate truth rule

At every `PTRACE_EVENT_FORK`, `PTRACE_EVENT_VFORK`, or `PTRACE_EVENT_CLONE`:

1. inspect the parent's pending syscall-entry record;
2. if the pending origin is clone/clone3 with readable flags, classify the fd-table relation from those flags regardless of ptrace event label;
3. if the event is `PTRACE_EVENT_CLONE` but no clone/clone3 origin flags are available, certificate becomes ambiguous/fail-closed;
4. true fork/vfork origins remain independent-copy semantics;
5. clone/clone3 with unreadable flags remains non-certifiable even if the event label looks like fork/vfork;
6. `CLONE_THREAD` must never substitute for `CLONE_FILES`;
7. public v2 fail-closed guard remains applied after internal certification and is not relaxed by C2.

The research internal `ProcessSpawn` mechanism may reflect proven syscall origin on the development branch, but no public schema migration is authorized.

## C3 mandatory attacks

### C3-A — ordinary clone, private fd table
Known clone origin, `CLONE_FILES` clear. Must certify `IndependentCopy` exactly once.

### C3-B — ordinary clone, shared fd table
Known clone origin with `CLONE_FILES`. Must certify `Shared` exactly once.

### C3-C — clone origin routed as FORK-style ptrace event
Use raw `clone()` with `SIGCHLD` exit signal and `CLONE_FILES`. If Linux reports `PTRACE_EVENT_FORK`, the collector must still classify the origin as clone and the fd relation as `Shared`; it must not silently downgrade to fork semantics.

### C3-D — clone origin routed as VFORK-style ptrace event
Use a valid `clone()` combination including `CLONE_VFORK` (and required VM semantics) with an explicit fd relation. If Linux reports `PTRACE_EVENT_VFORK`, classification must still follow clone flags. If the platform does not route this fixture as VFORK, preserve that negative observation; do not force the event.

### C3-E — CLONE_THREAD without CLONE_FILES
Must remain known-independent for fd-table relation even though it is a thread-group transition.

### C3-F — missing clone flags
Any child-creation event that requires clone-origin flags but lacks them must invalidate certification.

### C3-G — clone3 unreadable flags
Must remain incomplete and non-certifiable.

### C3-H — exec/de-threading from a shared table
Inherited identity must follow the declared exec/cloexec semantics; no stale shared table authority.

### C3-I — close/reopen/dup/reuse while shared
Replacement path must be tracked without stale identity.

### C3-J — resource/event loss
Any relevant loss keeps certification non-pass-eligible.

## Acceptance

`C2C3_CERTIFICATE_SURVIVES_BOUNDED` only if:
- fmt/clippy/tests pass;
- live C3-C demonstrates correct syscall-origin handling if the kernel produces a FORK-style event from clone;
- any observed VFORK-style clone routing is handled from syscall origin;
- no missing/unreadable origin becomes certified;
- existing M11/M12 fail-closed tests remain green;
- workspace regression remains green;
- public guard semantics are unchanged.

If a live counterexample causes false certification, record `C2C3_FALSE_CERTIFICATION_FOUND` and do not proceed to C4 until corrected and independently retested.

## Preserved boundary

C2/C3 success proves only a bounded internal certificate path. It does not make the frozen V5 learning set admissible and does not authorize public alpha.4 changes or a release.
