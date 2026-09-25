# Support

ExecSurface is currently a **public alpha** for Linux x86_64.

## Before opening an issue

Run:

```bash
execsurface doctor
```

Then read:

- [Five-Minute Start](docs/QUICKSTART_5_MIN.md)
- [Troubleshooting](docs/TROUBLESHOOTING.md)
- [Security boundary](SECURITY.md)

## Where to ask

Use GitHub Issues for:

- reproducible installation failures;
- supported-environment observer failures;
- documentation gaps;
- Action integration problems;
- adoption/compatibility reports.

Use the **Adoption / Integration** issue template for real project evaluation.

## What to include

Include:

- ExecSurface version;
- Linux distribution/kernel;
- x86_64 confirmation;
- command shape with secrets removed;
- CI provider/runner;
- `execsurface doctor` result;
- minimal error/evidence needed to reproduce.

## Do not post

Do not publish:

- tokens;
- passwords;
- private keys;
- private repository contents;
- confidential paths if their names are sensitive;
- file contents collected outside ExecSurface.

For a security vulnerability, follow [SECURITY.md](SECURITY.md) and use private vulnerability reporting when available.

## Support expectations

There is no public-alpha SLA.

A PASS verdict is not a safety certification, and support discussions will not reinterpret incomplete evidence as PASS.
