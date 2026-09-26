# M9 ptrace ESRCH hardening closeout

Status: PROVED candidate for merge, subject to pull-request CI and final main rerun.

## Counterexample
Public `0.1.0-alpha.2` failed closed on pinned `casey/just` commit
`5d5742cbcc50f19c99c356bc7e085acaa5f4665d` with ptrace `ESRCH`.

## Root cause and bounded handling
Hardening localized lifecycle races across Linux ptrace thread groups, including
exec TID collapse, pre-registration ordering, exit-event lifecycle, and a
`PTRACE_GET_SYSCALL_INFO` `ESRCH` window during an explicitly observed sibling
`exit_group` teardown. The product candidate handles only lifecycle states backed
by explicit tracker state; it does not introduce blanket `ESRCH` suppression,
sleeps, retry loops, eBPF substitution, or relaxed completeness semantics.

## Accepted H1.9 evidence
- H1.9 run: `36277418787`
- exact production source SHA-256:
  `3dc656358a3675a0d78135bb83046bc45c33dca2718c8bd8b68fa090e18eac8b`
- bounded diagnostic reproducer: 240/240 complete, 37 exact bounded
  `PTRACE_GET_SYSCALL_INFO` `ESRCH` recoveries observed
- instrumentation removed and exact production source restored
- production-clean bounded stress: 240/240 complete
- unchanged observer regression suite: PASS
- pinned `casey/just`: 5/5 complete with exit 0
- artifact ID: `10917113831`
- artifact ZIP SHA-256:
  `f70954c54e210f4c3de0ee7cdb70f5179a4105021bfd502858bb8acb3ffcbff3`

## Discriminator result
The earlier infinite-syscall stress fixture was retained as negative diagnostic
evidence but rejected as a lifecycle acceptance gate after snapshots showed
ptrace-stop starvation could occur before the tracer processed `exit_group`.
Finite bounded fixtures removed that confounder while still reproducing the exact
`GET_SYSCALL_INFO` lifecycle race.

## Authority boundary
Baseline, diff, policy, report, event-budget, privacy, and eBPF authorization
semantics are unchanged. The ptrace backend remains the correctness reference.
