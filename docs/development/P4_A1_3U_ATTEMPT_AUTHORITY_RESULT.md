# ExecSurface — P4-A1.3U Attempt-Scoped Authority Result

Date: 2026-09-29
Parent: #108 / #107 / #100
Branch: `development/post-alpha4-behavioral-integrity`
Protocol: `docs/development/P4_A1_3U_ATTEMPT_AUTHORITY_PROTOCOL.md`
Status: **CLOSED — PASS BOUNDED**

## Decision

`P4_A1_3U_ATTEMPT_AUTHORITY_PASS_BOUNDED`

The bounded correction establishes that `attempt_only` authority is usable for the explicit `P4.PATH.ACCESS_ATTEMPT` proposition while remaining structurally unable to satisfy stronger successful-object/effect requirements. The correction does not change public/raw-v2 bytes, the public collector, public learn/check behavior, baseline semantics, verdict semantics, or any release/tag/channel.

## Preserved falsification failure

The preregistered first A1.3U execution is retained as negative evidence:

- workflow: `36586076705`
- job: `109466636514`
- source: `b118e3ea6f8b3c732c6d0eae54da30918ce11125`
- result: **3/5 PASS, 2/5 FAIL**

The two failures were semantic, not formatting:

1. a complete exact `attempt_only` pathname-attempt record could not satisfy its own attempt-scoped requirement because `AdapterRecord::admissible_for()` admitted only `direct | derived_bounded`;
2. `AdapterRecord::validate()` did not reject `attempt_only` authority attached to a non-attempt proposition such as `ProcessExecSucceeded`.

The failure was not relabeled, discarded, or repaired by changing the preregistered assertions.

## Authorized bounded correction

Per the frozen A1.3U protocol, the model correction was restricted to the authority layer:

- `attempt_only` validation is permitted only for explicit attempt-shaped proposition variants:
  - `FilePathnameAttemptObserved`
  - `FileRenameAttemptObserved`
  - `NetworkConnectDestinationAttemptObserved`
- `attempt_only` is admissible only for the explicit product proposition ID `P4.PATH.ACCESS_ATTEMPT` paired with `FilePathnameAttemptObserved`;
- `proof.satisfies(requirement)` remains mandatory, so stronger success/object requirements cannot be satisfied by attempt evidence;
- rename/connect attempt records remain descriptive and do not gain successful `P4.FILE.RENAME_DELETE` or `P4.NET.CONNECT_DESTINATION` product authority;
- raw-v2 mapping and public runtime behavior are unchanged.

Correction commit:
`7dfdd3958ac7e6c38cb52859d883035fa6e9ace2`

## Accepted canonical reproof

Accepted source:
`ea3ab3ce57bdd3bdcc52b8150d53fcc461020d7b`

Workflow:
`36586951989`

Job:
`109469694231`

Artifact:
`11041628320`

Artifact SHA-256:
`sha256:6ae26bd38b5e54554c78ca8d513a5a1fc005b354a899cfbf580a39631093f008`

Accepted gates:

- rustfmt PASS;
- clippy `-D warnings` PASS;
- base authority-model tests **8/8 PASS**;
- A1.3U attempt-authority tests **5/5 PASS**;
- A1.3 adversarial + A1.2 integrated mapping tests **18/18 PASS**;
- separate A1.2 mapping tests **10/10 PASS**;
- Semantics v3 reproof **7/7 PASS**;
- public M11 shared-FD fail-closed contract **6/6 PASS**.

## Falsification interpretation

The accepted model now enforces both sides of the boundary:

- an explicit attempt proposition can carry useful attempt-scoped authority;
- attempt authority cannot be laundered into successful exec, successful-open object identity, FD success attribution, successful rename/delete, or successful connect authority.

This is a bounded authority-contract correction, not a new observer capability and not a completeness claim.

## Public / product boundary

No public integration or release is authorized by this result. `v0.1.0-alpha.4`, stable `@v0.1`, `main`, the public raw-v2 schema, default ptrace behavior, and public verdict behavior remain unchanged.

Because this accepted correction occurred after the earlier A1.4 anti-drift run, A1.4 must be re-executed from a source containing this result and correction before full A1 closure or A2 authorization.
