# Security Policy

ExecSurface is a security-adjacent developer tool.

## Security boundary

ExecSurface is not a sandbox, EDR, antivirus or malware detector.

In the current **Stable v1.0.0** Linux x86_64 / native-`ptrace` contract, an ExecSurface `PASS` means only that under the selected observer, normalization profile, baseline and policy, no policy-relevant observed execution-surface drift was identified. It does **not** prove that the wrapped target command exited successfully: target exit status/signal are recorded separately and are not verdict-bearing in v1.0.0.

It does not prove program safety, sandboxing, malware freedom, fair benchmarking, determinism or completeness of all unobserved behavior. Keep the wrapped command's own success/failure gate when its correctness matters.

## Sensitive data

Evidence and traces can be sensitive. Default product semantics prohibit collecting file contents, environment values, stdin, network payloads and full child argv values.

## Vulnerability reports

Use GitHub private vulnerability reporting when available. Avoid publishing exploitable details in a public issue before maintainers can assess them. For experimental external-trial tooling, the reported CI-output privacy boundary and its reviewed remediation are retained at [Issue #164](https://github.com/AETHERXGLOBAL/execsurface/issues/164); old pinned trial packet revisions do not receive fixes retroactively.
