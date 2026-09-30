"""Research-only GitLab CI context adapter for ExecSurface P7-A1.

This module binds validated GitLab CI identity to a deterministic context digest.
It is deliberately incapable of selecting baselines, changing verdicts, selecting
observers, or upgrading behavioral authority.
"""

from __future__ import annotations

import hashlib
import json
import re
from dataclasses import dataclass
from typing import Mapping

SCHEMA = "execsurface.gitlab-ci-context.v1"
PROVIDER = "gitlab-ci"
AUTHORITY = "context_only"

REQUIRED_FIELDS = (
    "CI_PROJECT_PATH",
    "CI_COMMIT_SHA",
    "CI_PIPELINE_ID",
    "CI_JOB_ID",
    "CI_JOB_NAME",
    "CI_PIPELINE_SOURCE",
)

_PROJECT_SEGMENT = re.compile(r"^[A-Za-z0-9][A-Za-z0-9_.-]{0,127}$")
_SHA = re.compile(r"^(?:[0-9A-Fa-f]{40}|[0-9A-Fa-f]{64})$")
_SOURCE = re.compile(r"^[A-Za-z0-9_.:-]{1,64}$")
_DECIMAL_ID = re.compile(r"^[0-9]+$")


class ContextError(ValueError):
    """Fail-closed validation error for GitLab CI context."""


@dataclass(frozen=True)
class BoundContext:
    record: dict[str, object]
    canonical_json: str
    context_digest: str

    def output(self) -> dict[str, object]:
        out = dict(self.record)
        out["context_digest"] = self.context_digest
        return out


def _required(env: Mapping[str, str], name: str) -> str:
    value = env.get(name)
    if value is None or value == "":
        raise ContextError(f"missing_required_field:{name}")
    if not isinstance(value, str):
        raise ContextError(f"non_string_field:{name}")
    return value


def _reject_controls(value: str, field: str) -> str:
    if any(ord(ch) < 0x20 or ord(ch) == 0x7F for ch in value):
        raise ContextError(f"control_character:{field}")
    if value != value.strip():
        raise ContextError(f"surrounding_whitespace:{field}")
    return value


def _project_path(value: str) -> str:
    value = _reject_controls(value, "CI_PROJECT_PATH")
    if len(value) > 255 or "//" in value:
        raise ContextError("invalid_project_path")
    parts = value.split("/")
    if len(parts) < 2 or any(part in {"", ".", ".."} for part in parts):
        raise ContextError("invalid_project_path")
    if not all(_PROJECT_SEGMENT.fullmatch(part) for part in parts):
        raise ContextError("invalid_project_path")
    return value


def _commit_sha(value: str) -> str:
    value = _reject_controls(value, "CI_COMMIT_SHA")
    if not _SHA.fullmatch(value):
        raise ContextError("invalid_commit_sha")
    return value.lower()


def _decimal_id(value: str, field: str) -> str:
    value = _reject_controls(value, field)
    if not _DECIMAL_ID.fullmatch(value):
        raise ContextError(f"invalid_decimal_id:{field}")
    number = int(value, 10)
    if number <= 0:
        raise ContextError(f"invalid_decimal_id:{field}")
    return str(number)


def _job_name(value: str) -> str:
    value = _reject_controls(value, "CI_JOB_NAME")
    if not (1 <= len(value) <= 255):
        raise ContextError("invalid_job_name")
    return value


def _pipeline_source(value: str) -> str:
    value = _reject_controls(value, "CI_PIPELINE_SOURCE")
    if not _SOURCE.fullmatch(value):
        raise ContextError("invalid_pipeline_source")
    return value


def bind_gitlab_context(env: Mapping[str, str]) -> BoundContext:
    """Validate and deterministically bind the frozen P7-A1 GitLab context.

    Only the six preregistered predefined variables are consumed. Unknown
    variables are ignored and therefore cannot steer baseline/verdict/authority.
    """

    project_path = _project_path(_required(env, "CI_PROJECT_PATH"))
    commit_sha = _commit_sha(_required(env, "CI_COMMIT_SHA"))
    pipeline_id = _decimal_id(_required(env, "CI_PIPELINE_ID"), "CI_PIPELINE_ID")
    job_id = _decimal_id(_required(env, "CI_JOB_ID"), "CI_JOB_ID")
    job_name = _job_name(_required(env, "CI_JOB_NAME"))
    pipeline_source = _pipeline_source(_required(env, "CI_PIPELINE_SOURCE"))

    record: dict[str, object] = {
        "schema": SCHEMA,
        "provider": PROVIDER,
        "project_path": project_path,
        "commit_sha": commit_sha,
        "pipeline_id": pipeline_id,
        "job_id": job_id,
        "job_name": job_name,
        "pipeline_source": pipeline_source,
        "authority": AUTHORITY,
        "may_select_baseline": False,
        "may_change_verdict": False,
        "may_upgrade_observer_authority": False,
    }
    canonical = json.dumps(record, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    digest = hashlib.sha256(canonical.encode("utf-8")).hexdigest()
    return BoundContext(record=record, canonical_json=canonical, context_digest=digest)
