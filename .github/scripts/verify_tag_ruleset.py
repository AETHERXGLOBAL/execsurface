#!/usr/bin/env python3
"""Verify immutable-tag governance for a candidate stable v1 release tag."""

from __future__ import annotations

import argparse
import fnmatch
import json
import sys
from pathlib import Path


def load_jsonl(path: Path) -> list[dict]:
    rows: list[dict] = []
    try:
        for raw in path.read_text(encoding="utf-8").splitlines():
            raw = raw.strip()
            if raw:
                rows.append(json.loads(raw))
    except (OSError, json.JSONDecodeError) as exc:
        raise ValueError(f"cannot read ruleset evidence: {exc}") from exc
    return rows


def protects(ruleset: dict, tag: str) -> bool:
    if ruleset.get("target") != "tag":
        return False
    if ruleset.get("enforcement") != "active":
        return False

    full_ref = f"refs/tags/{tag}"
    ref_name = (ruleset.get("conditions") or {}).get("ref_name") or {}
    includes = ref_name.get("include") or []
    excludes = ref_name.get("exclude") or []

    if not any(fnmatch.fnmatchcase(full_ref, pattern) for pattern in includes):
        return False
    if any(fnmatch.fnmatchcase(full_ref, pattern) for pattern in excludes):
        return False

    rule_types = {r.get("type") for r in (ruleset.get("rules") or [])}
    if not {"deletion", "update"}.issubset(rule_types):
        return False

    if ruleset.get("bypass_actors"):
        return False
    if ruleset.get("current_user_can_bypass") not in (None, "never"):
        return False

    return True


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--rulesets-jsonl", required=True)
    p.add_argument("--tag", required=True)
    args = p.parse_args()

    if not args.tag.startswith("v1."):
        print("tag-governance error: verifier is only for immutable v1 release tags", file=sys.stderr)
        return 2

    try:
        rulesets = load_jsonl(Path(args.rulesets_jsonl))
    except ValueError as exc:
        print(f"tag-governance error: {exc}", file=sys.stderr)
        return 2

    for ruleset in rulesets:
        if protects(ruleset, args.tag):
            print(
                json.dumps(
                    {
                        "tag": args.tag,
                        "ruleset_id": ruleset.get("id"),
                        "ruleset_name": ruleset.get("name"),
                        "protected": True,
                    },
                    sort_keys=True,
                )
            )
            return 0

    print(
        f"tag-governance error: no active no-bypass deletion+update tag ruleset protects {args.tag}",
        file=sys.stderr,
    )
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
