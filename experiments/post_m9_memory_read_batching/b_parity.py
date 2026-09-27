import copy
import json
import os
import re
import subprocess
import sys
from pathlib import Path

if len(sys.argv) != 5:
    raise SystemExit("usage: b_parity.py REF CAND B_HELPER EVIDENCE")

ref, cand, helper, evidence = sys.argv[1:]
out = Path(evidence) / "adversarial"
out.mkdir(parents=True, exist_ok=True)

CASES = [
    "string-short",
    "string-empty",
    "string-boundary",
    "string-span-pages",
    "string-unmapped-after",
    "string-max-no-nul",
    "fixed-openat2-normal",
    "fixed-openat2-cross",
    "fixed-openat2-unmapped",
    "connect-ipv4",
    "connect-ipv6",
    "connect-unix",
    "rename",
    "clone3",
]


def cleanup():
    for p in (
        "/tmp/execsurface-b-rename-a",
        "/tmp/execsurface-b-rename-b",
        "/tmp/execsurface-b-no-socket",
    ):
        try:
            os.unlink(p)
        except FileNotFoundError:
            pass


def tid_project(doc):
    d = copy.deepcopy(doc)
    mapping = {}
    next_id = 1

    def token(value):
        nonlocal next_id
        if value is None:
            return None
        key = int(value)
        if key not in mapping:
            mapping[key] = f"T{next_id}"
            next_id += 1
        return mapping[key]

    for event in d.get("events", []):
        if "tid" in event:
            event["tid"] = token(event["tid"])
        if event.get("event_type") == "process_spawn" and "child_tid" in event:
            event["child_tid"] = token(event["child_tid"])
    for warning in d.get("warnings", []):
        if warning.get("tid") is not None:
            warning["tid"] = token(warning["tid"])
    return d


def normalize_stderr(text):
    text = re.sub(r"\btid[ =:]\s*\d+\b", "tid=T", text)
    text = re.sub(r"\bpid[ =:]\s*\d+\b", "pid=T", text)
    return text


def run(binary, case, stem):
    cleanup()
    p = subprocess.run(
        [binary, "observe", "--", helper, case],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        timeout=30,
    )
    Path(stem + ".stdout").write_text(p.stdout)
    Path(stem + ".stderr").write_text(p.stderr)
    if p.returncode == 0:
        doc = json.loads(p.stdout)
        return {
            "kind": "json",
            "returncode": 0,
            "document": tid_project(doc),
        }
    return {
        "kind": "error",
        "returncode": p.returncode,
        "stderr": normalize_stderr(p.stderr),
        "stdout": p.stdout,
    }


summary = []
for case in CASES:
    r1 = run(ref, case, str(out / f"{case}.reference1"))
    r2 = run(ref, case, str(out / f"{case}.reference2"))
    deterministic = r1 == r2
    if not deterministic:
        (out / f"{case}.reference1.projection.json").write_text(
            json.dumps(r1, indent=2, sort_keys=True) + "\n"
        )
        (out / f"{case}.reference2.projection.json").write_text(
            json.dumps(r2, indent=2, sort_keys=True) + "\n"
        )
        summary.append({"case": case, "reference_deterministic": False})
        (out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
        raise SystemExit(f"reference/reference nondeterminism: {case}")

    c = run(cand, case, str(out / f"{case}.candidate"))
    equal = r1 == c
    item = {
        "case": case,
        "reference_deterministic": True,
        "candidate_equal": equal,
        "result_kind": r1["kind"],
        "returncode": r1["returncode"],
    }
    if r1["kind"] == "json":
        doc = r1["document"]
        item["complete"] = doc.get("complete")
        item["warnings"] = len(doc.get("warnings", []))
        item["events"] = len(doc.get("events", []))
        item["outcome"] = doc.get("outcome")
    summary.append(item)
    if not equal:
        (out / f"{case}.reference.projection.json").write_text(
            json.dumps(r1, indent=2, sort_keys=True) + "\n"
        )
        (out / f"{case}.candidate.projection.json").write_text(
            json.dumps(c, indent=2, sort_keys=True) + "\n"
        )
        (out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
        raise SystemExit(f"reference/Candidate-B mismatch: {case}")

(out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print(json.dumps(summary, indent=2, sort_keys=True))
