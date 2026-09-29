# P4-A1 Canonicalization Note

Date: 2026-09-29
Branch: `development/post-alpha4-behavioral-integrity`

During A1 continuation, a second research crate/workflow path was started before the existing P4 authority-model experiment was rediscovered on the same branch.

Anti-drift decision:
- canonical P4 authority implementation remains `experiments/p4-backend-authority`;
- canonical workflow remains `.github/workflows/p4-a0-a1-authority.yml`;
- the later duplicate `experiments/p4-ptrace-proposition-adapter` path is retired and removed from the working tree;
- its commits/workflow history remain in Git/Actions as engineering evidence and are not rewritten;
- no scientific result from the duplicate path is used for P4 acceptance;
- all subsequent A1 work must extend the canonical experiment, which directly reuses `execsurface_model::semantics_v3` proof-carrying observations.

Reason: maintaining two parallel authority models would create semantic drift and an avoidable translation layer. The existing canonical experiment is stronger because it is already bound directly to the accepted Semantics-v3 proposition/proof model.
