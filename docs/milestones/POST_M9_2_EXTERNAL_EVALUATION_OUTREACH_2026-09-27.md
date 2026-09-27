# Post-M9.2 — Independent External Evaluation Outreach

Date: 2026-09-27
Status: **OUTREACH SENT — awaiting external action**
Public release: `v0.1.0-alpha.3`

## Classification

This outreach occurs **after** the immutable M9.2 closeout and does not rewrite, reopen, or retroactively change that result.

The historical M9.2 result remains:

`KILLED — NO ACCEPTED INDEPENDENT EVIDENCE IN DECLARED SEARCH SCOPE (2026-09-27)`

Any future third-party run resulting from this outreach is new prospective external evidence. It may be reviewed under the existing independence/provenance/privacy rules, but it must not be represented as evidence that existed at the time of the M9.2 closeout.

## Purpose

Invite technically relevant external organizations to independently evaluate the current public ExecSurface alpha on infrastructure and workloads they control.

The request explicitly states that:

- no endorsement, sponsorship, procurement, or commercial commitment is requested;
- AETHER X will not execute commands for the evaluator;
- AETHER X will not edit the evaluator's workload;
- AETHER X will not materially debug the run or tune outcomes;
- failures, REVIEW, ERROR, usability friction, and negative findings are first-class results;
- evaluators must not send secrets, credentials, private file contents, or sensitive logs;
- a public repository/CI reference or concise independently produced result is sufficient.

## Public evaluation surface

- Repository: `https://github.com/AETHERXGLOBAL/execsurface`
- Technical Evaluation Pack: `docs/TECHNICAL_EVALUATION.md`
- Five-Minute Start: `docs/QUICKSTART_5_MIN.md`
- Current public release: `v0.1.0-alpha.3`

Before outreach, the two public evaluation guides above were aligned to `v0.1.0-alpha.3` so evaluators are not directed to the superseded alpha.2 release.

## Outreach cohort

Ten individualized outbound emails were sent on 2026-09-27 through public organizational contact channels:

| # | Organization | Public contact channel | Relevance | State |
|---|---|---|---|---|
| 1 | Rust Foundation | `contact@rustfoundation.org` | Rust ecosystem / developer tooling | SENT |
| 2 | OpenSSF | `support@openssf.org` | Open-source software supply-chain security | SENT |
| 3 | Socket Labs | `labs@socket.dev` | Supply-chain security / program behavior research | SENT |
| 4 | Determinate Systems | `hello@determinate.systems` | Reproducible developer environments / Nix | SENT |
| 5 | Flox | `hello@flox.dev` | Deterministic developer environments | SENT |
| 6 | Oxide Computer Company | `sales@oxide.computer` | Systems / infrastructure engineering | SENT |
| 7 | Tweag | `sales@tweag.io` | Reproducibility / developer tooling / systems research | SENT |
| 8 | GitLab Open Source Program | `opensource@gitlab.com` | CI / developer tooling / open source | SENT |
| 9 | Cloud Native Computing Foundation (CNCF) | `info@cncf.io` | Cloud-native / CI / infrastructure ecosystem | SENT |
| 10 | Anchore | `sales@anchore.com` | Software supply-chain and container security | SENT |

Use of a general or sales inbox does not imply a commercial request; those messages explicitly request routing to the relevant engineering/open-source/security contact when necessary.

## Evidence handling rule

An outbound email is **not adoption evidence**.

A reply expressing interest is also not by itself independent execution evidence.

A future result may be considered for independent-evidence review only if the third party independently executes the public workflow on infrastructure they control or independently choose and AETHER X does not materially assist execution or repair.

If material assistance becomes necessary, classify the result as `COLLABORATIVE_EXTERNAL`, not `INDEPENDENT_USER`.

All failures and non-countable outcomes must remain preserved rather than silently dropped.

## Next action

Monitor inbound replies and public references. For each response:

1. preserve the original external reference;
2. classify `NO_RUN`, `CANDIDATE_INDEPENDENT`, `COLLABORATIVE_EXTERNAL`, `INDEPENDENT_USER`, or `KILLED` based on provenance;
3. perform privacy review before retaining artifacts;
4. do not provide material execution assistance if independence is to be preserved;
5. if a genuine independent run is produced, validate it prospectively under the existing M9 evidence validators and record its exact scope without generalizing to production readiness or universal adoption.
