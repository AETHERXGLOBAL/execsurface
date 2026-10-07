#!/usr/bin/env bash
set -euo pipefail

candidate_bin="${1:?usage: p9_3_compatibility_gate.sh CANDIDATE_BIN ALPHA5_BIN}"
alpha5_bin="${2:?usage: p9_3_compatibility_gate.sh CANDIDATE_BIN ALPHA5_BIN}"

candidate_bin="$(realpath "$candidate_bin")"
alpha5_bin="$(realpath "$alpha5_bin")"

test -x "$candidate_bin"
test -x "$alpha5_bin"
test "$("$alpha5_bin" --version)" = "execsurface 0.1.0-alpha.5"

work="$(mktemp -d "${RUNNER_TEMP:-/tmp}/execsurface-p9-3.XXXXXX")"
trap 'rm -rf "$work"' EXIT
cd "$work"

printf 'candidate=%s\n' "$("$candidate_bin" --version)"
printf 'compatibility_source=%s\n' "$("$alpha5_bin" --version)"

# Produce the compatibility input with the exact published Alpha.5 binary.
"$alpha5_bin" learn --output alpha5.lock.json -- /bin/bash -lc true
python - <<'PY'
import json
from pathlib import Path
lock = json.loads(Path("alpha5.lock.json").read_text())
assert lock["schema_version"] == 2
assert lock["payload"]["digest_format_version"] == 2
assert lock["payload"]["tool"]["name"] == "execsurface"
assert lock["payload"]["tool"]["version"] == "0.1.0-alpha.5"
assert lock["payload"]["platform"]["os"] == "linux"
assert lock["payload"]["platform"]["architecture"] == "x86_64"
PY

baseline_before="$(sha256sum alpha5.lock.json | cut -d' ' -f1)"

# Candidate must consume the public Alpha.5 baseline without rewriting it.
set +e
pass_output="$("$candidate_bin" check --baseline alpha5.lock.json --json-output pass-report.json -- /bin/bash -lc true 2>&1)"
pass_status=$?
set -e
if [ "$pass_status" -ne 0 ]; then
  printf 'candidate_alpha5_pass_status=%s\n' "$pass_status"
  printf '%s\n' "$pass_output"

  "$alpha5_bin" observe -- /bin/bash -lc true > alpha5-observe.json
  "$candidate_bin" observe -- /bin/bash -lc true > candidate-observe.json
  python - alpha5-observe.json candidate-observe.json <<'PY'
import json
import sys

for label, path in zip(("alpha5", "candidate"), sys.argv[1:]):
    data = json.load(open(path, encoding="utf-8"))
    events = data.get("events", [])
    exec_by_tid = {}
    relevant = []
    for event in events:
        tid = event.get("tid")
        kind = event.get("event_type")
        if kind == "process_exec":
            exec_by_tid[tid] = event.get("path")
        elif kind == "file_descriptor_access":
            target = event.get("path", "")
            if target == "/dev/null" or target.startswith(("pipe:[", "socket:[", "anon_inode:")):
                relevant.append({
                    "sequence": event.get("sequence"),
                    "tid": tid,
                    "actor": exec_by_tid.get(tid),
                    "operation": event.get("operation"),
                    "fd": event.get("fd"),
                    "path": target,
                })
    print(f"{label}_fd_identity_diagnostic={json.dumps(relevant, sort_keys=True)}")
PY
fi
test "$pass_status" -eq 0
grep -F "ExecSurface: PASS" <<<"$pass_output"
python - <<'PY'
import json
from pathlib import Path
report = json.loads(Path("pass-report.json").read_text())
assert report["schema_version"] == 2
assert report["verdict"] == "pass"
PY

test "$baseline_before" = "$(sha256sum alpha5.lock.json | cut -d' ' -f1)"

# Unsupported baseline schema must fail explicitly, never migrate or reinterpret.
cp alpha5.lock.json unsupported.lock.json
python - <<'PY'
import json
from pathlib import Path
p = Path("unsupported.lock.json")
data = json.loads(p.read_text())
data["schema_version"] = 999
p.write_text(json.dumps(data, indent=2) + "\n")
PY
set +e
unsupported_output="$("$candidate_bin" check --baseline unsupported.lock.json -- /bin/bash -lc true 2>&1)"
unsupported_status=$?
set -e
test "$unsupported_status" -eq 2
grep -F "unsupported lock schema version: 999" <<<"$unsupported_output"

# Policy schema/shape errors must be ERROR, not permissive fallback.
cat > unsupported-policy.json <<'JSON'
{
  "schema_version": 999,
  "default_action": "review",
  "rules": []
}
JSON
set +e
policy_output="$("$candidate_bin" check --baseline alpha5.lock.json --policy unsupported-policy.json -- /bin/bash -lc true 2>&1)"
policy_status=$?
set -e
test "$policy_status" -eq 2
grep -F "unsupported policy schema version: 999" <<<"$policy_output"

cat > unknown-field-policy.json <<'JSON'
{
  "schema_version": 2,
  "default_action": "review",
  "rules": [],
  "silently_allow_everything": true
}
JSON
set +e
unknown_policy_output="$("$candidate_bin" check --baseline alpha5.lock.json --policy unknown-field-policy.json -- /bin/bash -lc true 2>&1)"
unknown_policy_status=$?
set -e
test "$unknown_policy_status" -eq 2
grep -F "unknown field" <<<"$unknown_policy_output"

# Stable verdict/exit-code contract: REVIEW=10.
set +e
review_output="$("$candidate_bin" check --baseline alpha5.lock.json -- /bin/bash -lc 'true; /bin/echo p9-review >/dev/null' 2>&1)"
review_status=$?
set -e
test "$review_status" -eq 10
grep -F "ExecSurface: REVIEW" <<<"$review_output"

# Stable verdict/exit-code contract: BLOCK=20.
cat > block-policy.json <<'JSON'
{
  "schema_version": 2,
  "default_action": "review",
  "rules": [
    {
      "id": "block-added-exec",
      "action": "block",
      "match": {
        "change": "added",
        "effect": "process_exec"
      }
    }
  ]
}
JSON
policy_before="$(sha256sum block-policy.json | cut -d' ' -f1)"
set +e
block_output="$("$candidate_bin" check --baseline alpha5.lock.json --policy block-policy.json -- /bin/bash -lc 'true; /bin/echo p9-block >/dev/null' 2>&1)"
block_status=$?
set -e
test "$block_status" -eq 20
grep -F "ExecSurface: BLOCK" <<<"$block_output"
test "$policy_before" = "$(sha256sum block-policy.json | cut -d' ' -f1)"

# Stable verdict/exit-code contract: ERROR=2.
set +e
error_output="$("$candidate_bin" check --baseline missing.lock.json -- /bin/bash -lc true 2>&1)"
error_status=$?
set -e
test "$error_status" -eq 2
grep -F "ExecSurface: ERROR" <<<"$error_output"

# Rollback rehearsal: the exact prior public Alpha.5 path must still consume the
# original baseline after candidate use, with no baseline or policy mutation.
set +e
rollback_output="$("$alpha5_bin" check --baseline alpha5.lock.json -- /bin/bash -lc true 2>&1)"
rollback_status=$?
set -e
test "$rollback_status" -eq 0
grep -F "ExecSurface: PASS" <<<"$rollback_output"
test "$baseline_before" = "$(sha256sum alpha5.lock.json | cut -d' ' -f1)"
test "$policy_before" = "$(sha256sum block-policy.json | cut -d' ' -f1)"

printf 'P9.3 CLI compatibility harness: PASS\n'
