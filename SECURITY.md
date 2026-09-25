# Security Policy

ExecSurface is a security-adjacent developer tool.

## Security boundary

ExecSurface is not a sandbox, EDR, antivirus or malware detector.

A future PASS verdict will mean only that under the recorded observer, normalization profile, baseline and policy, no disallowed execution-surface expansion was identified.

It will not prove that a program is safe or that unobserved behavior is impossible.

## Sensitive data

Evidence and traces can be sensitive. Default product semantics prohibit collecting file contents, environment values, stdin, network payloads and full child argv values.

## Vulnerability reports

Use GitHub private vulnerability reporting when available. Avoid publishing exploitable details in a public issue before maintainers can assess them.
