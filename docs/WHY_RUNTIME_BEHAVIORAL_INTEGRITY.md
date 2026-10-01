# Why provenance is not the same as runtime behavioral integrity

Software supply-chain security has made major progress on questions such as:

- Where did this artifact come from?
- Which workflow built it?
- Was it signed by an expected identity?
- Can the build provenance be verified?

Those questions matter. They are not the same question as:

> What did this software actually do when it ran here, and did that observed execution surface change from what was explicitly accepted before?

ExecSurface is built around that second question.

## Source identity and runtime behavior are different evidence

A source diff may be empty while runtime behavior changes because of:

- a dependency resolving differently;
- a build/test tool invoking an additional executable;
- a plugin or generated tool path changing;
- an external binary or script changing independently of the repository;
- environment-specific runtime behavior;
- a new network destination or file interaction appearing at execution time.

Conversely, a source diff can be large while the observed execution surface relevant to a particular command remains unchanged.

Neither fact proves safety. They answer different questions.

## The ExecSurface model

ExecSurface treats the accepted runtime surface and the policy as separate objects.

A simplified flow is:

```text
command
  |
  v
observe runtime effects
  |
  v
canonical execution surface
  |
  +---- accepted surface
  |
  v
explicit drift
  |
  v
explicit policy
  |
  v
PASS / REVIEW / BLOCK / ERROR
```

The key constraints are deliberate:

- frequency does not create authorization;
- a new baseline is not automatic approval;
- incomplete or ambiguous evidence cannot silently become PASS;
- backend identity does not automatically create authority;
- observed behavior is not all possible behavior;
- no observed drift does not prove that software is safe.

## A small example

Suppose a CI test command was learned with an accepted surface that includes the expected compiler, test binary and file accesses.

A later run unexpectedly introduces:

```text
+ EXEC     /usr/bin/curl
+ NETWORK  telemetry.example.com:443
```

ExecSurface does not label that behavior malware. It reports the observed change and evaluates the explicit policy.

That may result in `REVIEW` or `BLOCK`, depending on the policy selected by the user.

The important point is that the decision is based on explicit drift from an accepted execution surface, not on an opaque anomaly score.

## How this complements provenance systems

Build provenance and artifact signing answer identity and production-chain questions. Runtime behavioral integrity can consume that context without replacing it.

A composed path can look like:

```text
source / artifact / workflow identity
                |
                v
        provenance / signing
                |
                v
        observed execution
                |
                v
        accepted vs current
                |
                v
       deterministic drift
                |
                v
        explicit policy
                |
                v
        attestable result
```

This is why ExecSurface does not position itself as a replacement for SLSA, Sigstore, Falco or Tetragon. Those systems address adjacent or broader problems. See [Ecosystem Positioning](ECOSYSTEM_POSITIONING.md).

## Why evidence completeness matters

A runtime verifier is only as strong as the propositions its observation backend can actually support.

ExecSurface therefore keeps observer health, incompleteness and unsupported states explicit. Missing evidence is not converted into a clean result merely because no suspicious event was seen.

Alpha.5 uses native `ptrace` as its bounded public reference observer on Linux x86_64. Research on other evidence backends does not grant them public verdict authority until proposition-level semantics, completeness and loss behavior are proved.

## Try to falsify it

The most useful external result is not praise. It is evidence.

We want reports such as:

- a false PASS;
- an unchanged workload that produces unstable drift;
- a reproduction or install failure;
- an observer-completeness problem;
- a real workload where the model creates unacceptable noise;
- an interoperability limitation;
- evidence that an adjacent system already solves the same problem more directly;
- a counterexample to the claimed product boundary.

Current public release: `v0.1.0-alpha.5`

Install:

```bash
cargo install execsurface --version "=0.1.0-alpha.5" --locked
execsurface --version
execsurface doctor
```

Repository: https://github.com/AETHERXGLOBAL/execsurface

Public Alpha.5 review hub: https://github.com/AETHERXGLOBAL/execsurface/issues/118

Negative, partial and no-fit findings are explicitly welcome.

## Boundary

ExecSurface detects observed execution-surface drift under its recorded observer and policy.

It is not antivirus, EDR, SIEM, malware detection, a sandbox, or proof that software is safe. ARM64 support and independent external validation are not claimed for Alpha.5.
