#!/usr/bin/env python3
"""Research-only self-hosted CI packaging contract for ExecSurface P7-A2.

This module never changes ExecSurface behavioral authority or verdict semantics.
It validates a pre-approved baseline/configuration, records descriptive runner
context, and propagates the native ExecSurface check exit code unchanged.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import Mapping, Sequence

PACKAGE_VERSION = "v0.1.0-alpha.4"
PACKAGE_SHA256 = "55776cd130784d1c03451a14ab05ed113ffae4fa714c986666cbcadf242d8d1d"
APPROVAL_ORIGIN = "external_preapproved"
CONTEXT_AUTHORITY = "execution_environment_only"
SCHEMA = "execsurface.self-hosted-ci-package.v1"
HEX64 = re.compile(r"^[0-9a-f]{64}$")


class ContractError(ValueError):
    pass


@dataclass(frozen=True)
class PackageConfig:
    baseline_path: str
    baseline_sha256: str
    approval_origin: str
    package_version: str
    package_sha256: str

    def output(self) -> dict[str, str]:
        return {
            "approval_origin": self.approval_origin,
            "baseline_path": self.baseline_path,
            "baseline_sha256": self.baseline_sha256,
            "package_sha256": self.package_sha256,
            "package_version": self.package_version,
            "schema": SCHEMA,
        }

    @property
    def canonical_json(self) -> str:
        return json.dumps(self.output(), sort_keys=True, separators=(",", ":"))

    @property
    def config_digest(self) -> str:
        return hashlib.sha256(self.canonical_json.encode()).hexdigest()


@dataclass(frozen=True)
class EnvironmentContext:
    os_name: str
    architecture: str
    kernel: str
    ci_provider: str
    runner_name: str
    runner_labels: str

    def output(self) -> dict[str, object]:
        return {
            "architecture": self.architecture,
            "authority": CONTEXT_AUTHORITY,
            "ci_provider": self.ci_provider,
            "kernel": self.kernel,
            "may_change_verdict": False,
            "may_select_baseline": False,
            "may_upgrade_observer_authority": False,
            "os": self.os_name,
            "runner_labels": self.runner_labels,
            "runner_name": self.runner_name,
            "schema": "execsurface.self-hosted-ci-environment.v1",
        }

    @property
    def canonical_json(self) -> str:
        return json.dumps(self.output(), sort_keys=True, separators=(",", ":"))

    @property
    def context_digest(self) -> str:
        return hashlib.sha256(self.canonical_json.encode()).hexdigest()


def _clean_text(name: str, value: str, *, max_len: int = 256) -> str:
    if not isinstance(value, str):
        raise ContractError(f"{name}: expected string")
    if len(value) > max_len:
        raise ContractError(f"{name}: too long")
    if any(ord(ch) < 32 or ord(ch) == 127 for ch in value):
        raise ContractError(f"{name}: control character")
    return value


def _validate_relative_baseline(value: str) -> str:
    value = _clean_text("baseline_path", value, max_len=512)
    path = PurePosixPath(value)
    if not value or path.is_absolute() or value in (".", ".."):
        raise ContractError("baseline_path: must be a non-empty relative path")
    if ".." in path.parts or "" in path.parts:
        raise ContractError("baseline_path: traversal is forbidden")
    return value


def validate_config(data: Mapping[str, object]) -> PackageConfig:
    expected = {
        "baseline_path",
        "baseline_sha256",
        "approval_origin",
        "package_version",
        "package_sha256",
    }
    if set(data) != expected:
        missing = sorted(expected - set(data))
        extra = sorted(set(data) - expected)
        raise ContractError(f"config keys mismatch: missing={missing} extra={extra}")

    baseline_path = _validate_relative_baseline(str(data["baseline_path"]))
    baseline_sha = str(data["baseline_sha256"]).lower()
    package_sha = str(data["package_sha256"]).lower()
    if not HEX64.fullmatch(baseline_sha):
        raise ContractError("baseline_sha256: expected 64 lowercase hex")
    if str(data["approval_origin"]) != APPROVAL_ORIGIN:
        raise ContractError("approval_origin: must be external_preapproved")
    if str(data["package_version"]) != PACKAGE_VERSION:
        raise ContractError("package_version: frozen version mismatch")
    if package_sha != PACKAGE_SHA256:
        raise ContractError("package_sha256: frozen package digest mismatch")

    return PackageConfig(
        baseline_path=baseline_path,
        baseline_sha256=baseline_sha,
        approval_origin=APPROVAL_ORIGIN,
        package_version=PACKAGE_VERSION,
        package_sha256=PACKAGE_SHA256,
    )


def bind_environment(env: Mapping[str, str], *, kernel: str | None = None) -> EnvironmentContext:
    os_name = env.get("RUNNER_OS") or env.get("CI_RUNNER_OS") or ""
    arch_raw = env.get("RUNNER_ARCH") or env.get("CI_RUNNER_ARCH") or ""
    if os_name.lower() != "linux":
        raise ContractError("unsupported_os: alpha.4 package contract requires Linux")
    arch_norm = arch_raw.lower().replace("-", "_")
    if arch_norm not in {"x64", "x86_64", "amd64"}:
        raise ContractError("unsupported_architecture: alpha.4 package contract requires x86_64")

    kernel_value = kernel if kernel is not None else env.get("RUNNER_KERNEL", "unknown")
    return EnvironmentContext(
        os_name="Linux",
        architecture="x86_64",
        kernel=_clean_text("kernel", kernel_value, max_len=128),
        ci_provider=_clean_text("ci_provider", env.get("CI_PROVIDER", "unspecified")),
        runner_name=_clean_text("runner_name", env.get("RUNNER_NAME", "unspecified")),
        runner_labels=_clean_text("runner_labels", env.get("RUNNER_LABELS", "unspecified"), max_len=512),
    )


def load_config(path: Path) -> PackageConfig:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise ContractError(f"cannot load config: {exc}") from exc
    if not isinstance(data, dict):
        raise ContractError("config root must be an object")
    return validate_config(data)


def verify_baseline(workspace: Path, config: PackageConfig) -> Path:
    root = workspace.resolve(strict=True)
    candidate = (root / config.baseline_path).resolve(strict=True)
    try:
        candidate.relative_to(root)
    except ValueError as exc:
        raise ContractError("baseline_path resolves outside workspace") from exc
    if not candidate.is_file():
        raise ContractError("baseline_path is not a regular file")
    digest = hashlib.sha256(candidate.read_bytes()).hexdigest()
    if digest != config.baseline_sha256:
        raise ContractError("baseline_sha256 mismatch")
    return candidate


def run_check(
    *,
    binary: Path,
    workspace: Path,
    config: PackageConfig,
    reports_dir: Path,
    command: Sequence[str],
    env: Mapping[str, str],
    kernel: str | None = None,
) -> int:
    if not command:
        raise ContractError("command must not be empty")
    context = bind_environment(env, kernel=kernel)
    baseline = verify_baseline(workspace, config)
    if not binary.is_file():
        raise ContractError("execsurface binary missing")

    reports_dir.mkdir(parents=True, exist_ok=True)
    (reports_dir / "package-config.json").write_text(config.canonical_json + "\n", encoding="utf-8")
    (reports_dir / "environment-context.json").write_text(context.canonical_json + "\n", encoding="utf-8")
    (reports_dir / "binding.txt").write_text(
        f"config_digest=sha256:{config.config_digest}\n"
        f"environment_digest=sha256:{context.context_digest}\n"
        "authority=execution_environment_only\n"
        "native_exit_code_preserved=true\n",
        encoding="utf-8",
    )

    completed = subprocess.run(
        [
            str(binary),
            "check",
            "--baseline",
            str(baseline),
            "--json-output",
            str(reports_dir / "report.json"),
            "--markdown-output",
            str(reports_dir / "report.md"),
            "--",
            *command,
        ],
        cwd=workspace,
        check=False,
    )
    return completed.returncode


def _main(argv: Sequence[str]) -> int:
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest="action", required=True)

    validate = sub.add_parser("validate")
    validate.add_argument("--config", required=True, type=Path)
    validate.add_argument("--workspace", required=True, type=Path)

    check = sub.add_parser("check")
    check.add_argument("--config", required=True, type=Path)
    check.add_argument("--workspace", required=True, type=Path)
    check.add_argument("--binary", required=True, type=Path)
    check.add_argument("--reports-dir", required=True, type=Path)
    check.add_argument("command", nargs=argparse.REMAINDER)

    args = parser.parse_args(argv)
    try:
        config = load_config(args.config)
        if args.action == "validate":
            context = bind_environment(os.environ, kernel=os.uname().release)
            baseline = verify_baseline(args.workspace, config)
            print(
                json.dumps(
                    {
                        "baseline": str(baseline),
                        "config_digest": f"sha256:{config.config_digest}",
                        "environment": context.output(),
                        "environment_digest": f"sha256:{context.context_digest}",
                    },
                    sort_keys=True,
                    separators=(",", ":"),
                )
            )
            return 0

        command = list(args.command)
        if command and command[0] == "--":
            command = command[1:]
        return run_check(
            binary=args.binary,
            workspace=args.workspace,
            config=config,
            reports_dir=args.reports_dir,
            command=command,
            env=os.environ,
            kernel=os.uname().release,
        )
    except ContractError as exc:
        print(f"self-hosted contract error: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(_main(sys.argv[1:]))
