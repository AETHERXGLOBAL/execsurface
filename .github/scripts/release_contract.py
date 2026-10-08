#!/usr/bin/env python3
"""Bounded ExecSurface release-contract validator.

Supported by V1-R1:
- 0.1.0-alpha.N -> immutable v0.1.0-alpha.N, moving channel v0.1, prerelease
- 1.X.Y final   -> immutable v1.X.Y, moving channel v1, stable

Everything else fails closed until separately designed and qualified.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ALPHA_RE = re.compile(r"^0\.1\.0-alpha\.(0|[1-9][0-9]*)$")
V1_STABLE_RE = re.compile(r"^1\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$")


class ContractError(ValueError):
    pass


def classify(version: str, tag: str) -> dict[str, object]:
    expected_tag = f"v{version}"
    if tag != expected_tag:
        raise ContractError(f"tag/version mismatch: expected {expected_tag}, got {tag}")

    if ALPHA_RE.fullmatch(version):
        return {
            "version": version,
            "tag": tag,
            "stable_channel": "v0.1",
            "release_kind": "prerelease",
            "is_prerelease": True,
        }

    if V1_STABLE_RE.fullmatch(version):
        return {
            "version": version,
            "tag": tag,
            "stable_channel": "v1",
            "release_kind": "stable",
            "is_prerelease": False,
        }

    raise ContractError(
        "unsupported release version; V1-R1 accepts only 0.1.0-alpha.N or final 1.X.Y"
    )


def validate_request(
    request_path: Path, package_version: str, release_tag_file: str
) -> dict[str, object]:
    try:
        request = json.loads(request_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise ContractError(f"cannot read release request: {exc}") from exc

    for key in ("version", "tag", "stable_channel"):
        if key not in request or not isinstance(request[key], str) or not request[key]:
            raise ContractError(f"release request requires non-empty string field: {key}")

    version = request["version"]
    tag = request["tag"]
    classified = classify(version, tag)

    if package_version != version:
        raise ContractError(
            f"package/request version mismatch: package={package_version} request={version}"
        )
    if release_tag_file.strip() != tag:
        raise ContractError(
            "action release-tag/request mismatch: "
            f"action={release_tag_file.strip()} request={tag}"
        )
    if request["stable_channel"] != classified["stable_channel"]:
        raise ContractError(
            "stable channel mismatch: "
            f"expected {classified['stable_channel']} got {request['stable_channel']}"
        )

    revision = request.get("request_revision", 1)
    if not isinstance(revision, int) or revision < 1:
        raise ContractError("request_revision must be a positive integer")

    classified["request_revision"] = revision
    return classified


def emit(result: dict[str, object], output_format: str) -> None:
    if output_format == "json":
        print(json.dumps(result, indent=2, sort_keys=True))
        return

    if output_format == "github":
        for key in (
            "version",
            "tag",
            "stable_channel",
            "release_kind",
            "is_prerelease",
            "request_revision",
        ):
            if key not in result:
                continue
            value = result[key]
            if isinstance(value, bool):
                value = "true" if value else "false"
            print(f"{key.replace('_', '-')}={value}")
        return

    raise AssertionError(output_format)


def parser() -> argparse.ArgumentParser:
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="command", required=True)

    validate = sub.add_parser("validate-request")
    validate.add_argument("--request", required=True)
    validate.add_argument("--package-version", required=True)
    validate.add_argument("--release-tag-file", required=True)
    validate.add_argument("--output-format", choices=("json", "github"), default="json")

    classify_cmd = sub.add_parser("classify")
    classify_cmd.add_argument("--version", required=True)
    classify_cmd.add_argument("--tag", required=True)
    classify_cmd.add_argument("--output-format", choices=("json", "github"), default="json")

    return p


def main() -> int:
    args = parser().parse_args()
    try:
        if args.command == "validate-request":
            result = validate_request(
                Path(args.request), args.package_version, args.release_tag_file
            )
        else:
            result = classify(args.version, args.tag)
        emit(result, args.output_format)
        return 0
    except ContractError as exc:
        print(f"release-contract error: {exc}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
