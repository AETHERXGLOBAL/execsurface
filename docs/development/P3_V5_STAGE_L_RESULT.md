# ExecSurface — P3 V5 Stage L Real-Workload Requalification Result

Date: 2026-09-29
Tracking: #103
Parent program: #100
Protocol: `docs/development/P3_V5_REAL_WORKLOAD_PROTOCOL.md`
Branch: `development/post-alpha4-behavioral-integrity`
Status: **CLOSED AT STAGE L — P3_INCOMPLETE_EVIDENCE**

## Frozen workload

- repository: `junegunn/fzf`
- revision: `b1be3a8be1b833ce5b92fbbac11637643d60a046`
- host: GitHub-hosted Ubuntu 24.04.5
- kernel: Linux 6.17.0-1022-azure x86_64
- Go: go1.24.13 linux/amd64
- Rust: rustc 1.90.0
- ExecSurface execution source: `9090fc01bd26ce66ddf6df5aaf0d5492a1b4b58b`

The three preregistered direct priming runs completed successfully.

## Stage L execution

Workflow run:
`36559896295`

Job:
`109377956718`

Artifact:
`11028623425`

Artifact digest:
`sha256:5bde2d52dd6c2d3c3662ec6cda7a972bf458248eaef06d4a0f09b7181a443f63`

Exactly six learning attempts were executed. No attempt was replaced or rerun.

| learning attempt | exit code | lock produced |
|---|---:|---|
| 1 | 2 | no |
| 2 | 2 | no |
| 3 | 2 | no |
| 4 | 2 | no |
| 5 | 2 | no |
| 6 | 2 | no |

Every stderr record reported:

`ExecSurface: ERROR`

`execsurface: raw observation is incomplete and cannot form a trusted canonical surface`

Therefore the frozen Stage L admission rule rejected the complete learning set.

## Formal classification

`P3_INCOMPLETE_EVIDENCE`

This is not a variance-model failure and not a variance-model value success. The real-workload value question is inadmissible because trusted multi-run learning evidence could not be formed under the current public v2 completeness contract.

Per protocol:
- Stage A explicit acceptance was not entered;
- Stage C six-check comparison was not entered;
- no learning run was replaced;
- no completeness rule was weakened;
- no broad cache/module/stdlib normalization was introduced;
- no positive value claim is authorized.

## Scientific interpretation

The result independently reproduces the architectural ordering exposed by E4: real concurrent FZF/Go workloads can be blocked before downstream performance/variance analysis because the current public v2 observer contract treats an incomplete raw observation as ineligible for trusted canonicalization.

The current alpha.4 shared-FD guard is deliberately conservative: clone-based concurrency can make the v2 observation incomplete because raw v2 does not retain the relationship evidence needed to certify fd-table lifecycle attribution. P2/S5 showed that the ptrace state machine can observe enough clone information to distinguish important shared/known-independent cases in a bounded research model, but that evidence is not part of the public v2 contract.

The exact warning class for this V5 run must be confirmed by a separate diagnostic record before attributing all six failures specifically to `shared_fd_table_ambiguity`; the CLI error alone proves incompleteness, not its internal warning code.

## Next action

Do not rerun V5 and do not replace samples.

Open a separate **P3 V5-D completeness attribution diagnostic** on the same pinned workload using the existing observer API to preserve raw warnings and completeness classification. This diagnostic may identify the blocking proposition but cannot convert the failed V5 learning set into admissible evidence.

If the blocker is the expected conservative shared-FD ambiguity, the next scientific step is a separately gated research-only runtime bridge that retains P2/S5 exact fd-table relation evidence; it must not weaken or reinterpret public alpha.4 v2.
