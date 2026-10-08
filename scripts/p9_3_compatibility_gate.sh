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

# Freeze the exact public Alpha.5 profile-3 baseline.
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
assert lock["payload"]["canonical_surface"]["normalization"]["profile_version"] == 3
PY
alpha5_before="$(sha256sum alpha5.lock.json | cut -d' ' -f1)"

# Stage-2 R2 cross-version counterexample.
cat > dup2_probe.py <<'PY'
import os
import sys

victim = sys.argv[1]
victim_fd = os.open(victim, os.O_WRONLY | os.O_CREAT | os.O_TRUNC, 0o600)
read_fd, write_fd = os.pipe()
os.dup2(write_fd, victim_fd)
assert os.write(victim_fd, b"x") == 1
os.close(read_fd)
os.close(write_fd)
os.close(victim_fd)
PY

victim="$work/stage2-r2-victim.txt"
: > "$victim"
"$alpha5_bin" learn --output alpha5-dup2.lock.json -- python3 dup2_probe.py "$victim"
test ! -s "$victim"
: > "$victim"
"$candidate_bin" learn --output candidate-dup2.lock.json -- python3 dup2_probe.py "$victim"
test ! -s "$victim"

python - "$victim" <<'PY'
import json
import sys
from pathlib import Path

victim_name = Path(sys.argv[1]).name
alpha = json.loads(Path("alpha5-dup2.lock.json").read_text())
candidate = json.loads(Path("candidate-dup2.lock.json").read_text())

def writes_victim(lock):
    out = []
    for effect in lock["payload"]["canonical_surface"]["effects"]:
        if effect.get("event_type") != "file_path_access":
            continue
        if effect.get("operation") != "write":
            continue
        target = effect.get("target", {})
        if target.get("resolution") != "kernel_fd_resolved":
            continue
        if str(target.get("value", "")).endswith(victim_name):
            out.append(effect)
    return out

assert alpha["payload"]["canonical_surface"]["normalization"]["profile_version"] == 3
assert candidate["payload"]["canonical_surface"]["normalization"]["profile_version"] == 4
assert writes_victim(alpha), "Alpha.5 did not reproduce the stale-destination phantom write"
assert not writes_victim(candidate), "candidate retained the stale-destination phantom write"
print("P9.3 Stage-2 R2 cross-version counterexample: PASS")
PY

# Profile 3 remains immutable/readable history but is explicitly incomparable
# with the corrected profile 4.
set +e
legacy_output="$("$candidate_bin" check --baseline alpha5.lock.json -- /bin/bash -lc true 2>&1)"
legacy_status=$?
set -e
test "$legacy_status" -eq 2
grep -F "ExecSurface: ERROR" <<<"$legacy_output"
grep -F "baseline and candidate are not comparable" <<<"$legacy_output"
grep -F "normalization.profile_version baseline=3 candidate=4" <<<"$legacy_output"
test "$alpha5_before" = "$(sha256sum alpha5.lock.json | cut -d' ' -f1)"

# Candidate profile-4 baselines remain stable under corrected semantics.
"$candidate_bin" learn --output candidate.lock.json -- /bin/bash -lc true
python - <<'PY'
import json
from pathlib import Path
lock = json.loads(Path("candidate.lock.json").read_text())
assert lock["schema_version"] == 2
assert lock["payload"]["digest_format_version"] == 2
assert lock["payload"]["canonical_surface"]["normalization"]["profile_version"] == 4
PY
candidate_before="$(sha256sum candidate.lock.json | cut -d' ' -f1)"

set +e
pass_output="$("$candidate_bin" check --baseline candidate.lock.json --json-output pass-report.json -- /bin/bash -lc true 2>&1)"
pass_status=$?
set -e
test "$pass_status" -eq 0
grep -F "ExecSurface: PASS" <<<"$pass_output"
python - <<'PY'
import json
from pathlib import Path
report = json.loads(Path("pass-report.json").read_text())
assert report["schema_version"] == 2
assert report["verdict"] == "pass"
PY
test "$candidate_before" = "$(sha256sum candidate.lock.json | cut -d' ' -f1)"

# Unsupported baseline schema must fail explicitly.
cp candidate.lock.json unsupported.lock.json
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
policy_output="$("$candidate_bin" check --baseline candidate.lock.json --policy unsupported-policy.json -- /bin/bash -lc true 2>&1)"
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
unknown_policy_output="$("$candidate_bin" check --baseline candidate.lock.json --policy unknown-field-policy.json -- /bin/bash -lc true 2>&1)"
unknown_policy_status=$?
set -e
test "$unknown_policy_status" -eq 2
grep -F "unknown field" <<<"$unknown_policy_output"

# Stable verdict/exit-code contract: REVIEW=10.
set +e
review_output="$("$candidate_bin" check --baseline candidate.lock.json -- /bin/bash -lc 'true; /bin/echo p9-review >/dev/null' 2>&1)"
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
block_output="$("$candidate_bin" check --baseline candidate.lock.json --policy block-policy.json -- /bin/bash -lc 'true; /bin/echo p9-block >/dev/null' 2>&1)"
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

# Exact Alpha.5 rollback remains valid and neither baseline is mutated.
set +e
rollback_output="$("$alpha5_bin" check --baseline alpha5.lock.json -- /bin/bash -lc true 2>&1)"
rollback_status=$?
set -e
test "$rollback_status" -eq 0
grep -F "ExecSurface: PASS" <<<"$rollback_output"
test "$alpha5_before" = "$(sha256sum alpha5.lock.json | cut -d' ' -f1)"
test "$candidate_before" = "$(sha256sum candidate.lock.json | cut -d' ' -f1)"
test "$policy_before" = "$(sha256sum block-policy.json | cut -d' ' -f1)"

printf 'P9.3 CLI compatibility harness: PASS\n'
