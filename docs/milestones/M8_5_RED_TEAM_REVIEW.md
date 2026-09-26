# M8.5 — Independent Parity Red-Team Review

Date: 2026-09-26
Status: **ACCEPTED — NARROW PARITY CLAIMS ONLY**
Tracking: #42
Reference backend: `linux-ptrace-metadata-v2`
Candidate backend: `linux-libbpf-metadata-experimental-v1`

## Red-team mandate

Attempt to falsify M8.5 by looking for ways that event-count similarity, runtime identifiers, syscall names, incomplete evidence, unresolved path identity, unsupported capability classes, fixture unsafety, or CLI integration could be misrepresented as ptrace/eBPF semantic equivalence.

This review is independent of the implementation role. Its job is to block closure if any candidate uncertainty can silently become equivalence or PASS authority.

## Attacks and findings

### 1. Raw event-count equality

**Attack:** produce the same number of events with a changed spawn mechanism.

**Result:** **REJECTED AS PARITY.** The differential harness classifies the selected class as `contradicted`. Event count is not accepted as semantic equivalence.

### 2. Syscall name mistaken for process-creation semantics

**Attack:** rely on the Linux syscall name `clone` for a libc `fork()` workload.

**Result:** **REAL COUNTEREXAMPLE FOUND AND FIXED.** The original eBPF candidate reported clone semantics while ptrace reported fork semantics. The collector now classifies classic clone exits using Linux clone flags / exit-signal semantics. The original failed evidence is preserved.

### 3. Process TGID mistaken for task parent identity

**Attack:** create descendants from a non-leader thread.

**Result:** **REAL COUNTEREXAMPLE FOUND AND FIXED.** TGID-based parent attribution flattened nested-thread lineage. Parent identity is now task/TID scoped for the proved classic-clone fixtures. The original contradiction is preserved.

### 4. clone3 overclaim

**Attack:** force the candidate to infer clone3 mechanism without defensible metadata.

**Result:** **FAIL-CLOSED RETAINED.** The attempted user-memory helper path conflicted with the Apache-2.0 license boundary. The license was not weakened. Unresolved clone3 mechanism remains explicit and does not become equivalence.

### 5. Incomplete/lost evidence

**Attack:** truncate selected evidence / introduce collection incompleteness.

**Result:** **BLOCKED.** Selected semantic classes become `blocked_incomplete`; incomplete evidence cannot prove parity.

### 6. Successful-open positive witness with unrelated unresolved opens

**Attack:** make at least one unrelated eBPF successful-open path unresolved while retaining a resolved `/dev/zero` successful-open witness.

**Result:** **POSITIVE EXISTENCE CLAIM SURVIVES NARROWLY.** A matching non-empty ptrace FD-attributed witness and eBPF successful-open witness for the exact focused path proves that the focused successful open occurred. Unrelated unresolved opens do not negate that existential fact.

This is not generalized to unused successful opens and is not an absence proof.

### 7. Failed-open / absence claim under variable resolution health

**Attack:** use a missing path and attempt to infer that no successful open occurred merely because no resolved focused successful-open witness exists.

**Result:** **FAIL-CLOSED / HEALTH-CONDITIONAL.** The missing path is never promoted into successful-open evidence. If any candidate successful-open identity is unresolved, the requested absence conclusion is `blocked_incomplete`. If all candidate successful-open identities are resolved and both focused witness sets are empty, the class remains `non_comparable`; absence is still not promoted to equivalence.

This distinction is required because userspace `/proc/<pid>/fd/<fd>` resolution can legitimately vary with scheduling between otherwise identical runners.

### 8. vfork fixture safety

**Attack:** accept parity evidence from Rust `libc::vfork()` even though the Rust/libc binding explicitly warns that `vfork` can corrupt memory.

**Result:** **RUST PATH REMOVED.** The vfork witness was moved to a native C fixture compiled with clang. The child performs only `execl` and `_exit` before exec, removing Rust runtime/ownership behavior from the vfork evidence path.

### 9. Unsupported capability promotion

**Attack:** infer full-surface equivalence from passing selected classes.

**Result:** **BLOCKED.** Unsupported classes remain `non_comparable`; `full_surface_comparable=false` remains explicit.

### 10. eBPF PASS authority

**Attack:** make experimental libbpf evidence appear complete/PASS-eligible through the CLI.

**Result:** **BLOCKED.** The CLI rejects an experimental report that claims `observation_complete=true`. The eBPF path remains observation-only and does not enter normal learn/check PASS authority.

## Final evidence reviewed

Final closure candidate at commit `a73cf1bd7ce485d0649268480cc3696113cb8a6c`:

- normal repository CI: **PASS** — workflow run `36256996309`;
- M8.5 semantic differential: **PASS** — workflow run `36256996339`;
- clean fork/exec class-local parity: **PASS**;
- focused `/dev/zero` successful-open witness: **equivalent** for the controlled positive existential proposition;
- failed missing-path open: **never promoted**, with verdict bound to actual resolution health (`blocked_incomplete` when unresolved identities exist, otherwise `non_comparable`);
- native C vfork semantic parity: **PASS**;
- clone3 unresolved semantics: **fail-closed**;
- same-count / different-semantics mutation: **contradicted**;
- incomplete selected evidence: **blocked_incomplete**.

Preserved negative evidence includes at least:

- workflow run `36251855360` — fork vs syscall-name clone semantic mismatch;
- workflow run `36252411516` — nested-thread parent attribution mismatch;
- kernel-verifier rejection of the GPL-restricted clone3 helper path under the Apache-2.0 boundary;
- workflow run `36255562056` — focused successful-open proof incorrectly blocked by unrelated unresolved opens;
- workflow run `36256075043` — positive focused proof passed while the first negative-test expectation exposed the need to bind absence conclusions to resolution health;
- workflow run `36256614065` — the same negative fixture produced `non_comparable` because all unrelated successful-open identities resolved on that runner, proving that resolution completeness is scheduler/runner dependent and must be tested semantically rather than by one hard-coded verdict.

## Red-team decision

**ACCEPT M8.5 for the explicitly proved semantic projections only.**

The following are permitted conclusions:

- controlled classic fork / nested classic-clone lineage witnesses can be semantically equivalent across ptrace and the experimental eBPF candidate after the recorded fixes;
- controlled process-exec occurrence witnesses can be semantically equivalent for the tested structural roles;
- a controlled native-C vfork witness reaches scoped parity for the tested vfork -> exec workload;
- a controlled used-FD positive successful-open witness can establish the same focused existential proposition across both backends;
- incomplete evidence and unresolved absence remain fail-closed.

The following remain prohibited conclusions:

- full-surface ptrace/eBPF equivalence;
- universal clone3 parity;
- parity for unsupported path/network/fd-lifecycle/rename/delete/causal-chain classes;
- production readiness;
- automatic backend interchangeability for learned baselines;
- eBPF PASS authority.

## Authority after M8.5

- ptrace correctness reference: **RETAINED**;
- eBPF full-surface comparability: **FALSE**;
- eBPF PASS authority: **NOT AUTHORIZED**;
- next gate: **M8.6 — Performance / Compatibility**.
