import copy
import json
import os
import subprocess
import sys
from pathlib import Path

if len(sys.argv) != 6:
    raise SystemExit("usage: a2_raw_parity_v5.py REF CAND HELPER FIXTURE EVIDENCE")

ref, cand, helper, fixture, evidence = sys.argv[1:]
out = Path(evidence) / "raw-lane"
out.mkdir(parents=True, exist_ok=True)
fixed = "/tmp/execsurface-a2-fixed"

# Lane A only: workloads whose reference was raw-deterministic in v4, plus
# fixture-spawn which v4 did not reach. Concurrent thread-read is intentionally
# excluded from raw-order equality and is handled by the production semantic
# lane frozen in the v5 protocol.
cases = [
    ("irrelevant", [helper, "getpid", "5000"], None),
    ("fileio", [helper, "fileio", fixed], b"fixture"),
    ("failed-open", [helper, "failed-open"], None),
    ("signal", [helper, "signal"], None),
    ("restart", [helper, "restart"], None),
    ("restart-single", [helper, "restart-single"], None),
    ("fork", [helper, "fork", fixed], b"fixture"),
    ("vfork", [helper, "vfork"], None),
    ("exec", [helper, "exec"], None),
    ("failed-exec", [helper, "failed-exec"], None),
    ("thread-exec", [helper, "thread-exec"], None),
    ("exit-group", [helper, "exit-group"], None),
    ("fixture-spawn", [fixture, "spawn", "phase-aware-a2"], None),
]


def reset_file(content):
    try:
        os.unlink(fixed)
    except FileNotFoundError:
        pass
    if content is not None:
        Path(fixed).write_bytes(content)


def run(binary, args, path):
    with path.open("w") as f:
        p = subprocess.run(
            [binary, "observe", "--", *args],
            stdout=f,
            stderr=subprocess.PIPE,
            text=True,
            timeout=30,
        )
    Path(str(path) + ".stderr").write_text(p.stderr)
    if p.returncode != 0:
        raise RuntimeError(f"{binary} {args} rc={p.returncode}: {p.stderr}")
    return json.loads(path.read_text())


def projection(doc):
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
        if "tid" not in event or "event_type" not in event:
            raise RuntimeError("raw event missing required flattened identity fields")
        event["tid"] = token(event["tid"])
        if event["event_type"] == "process_spawn":
            if "child_tid" not in event:
                raise RuntimeError("process_spawn event missing child_tid")
            event["child_tid"] = token(event["child_tid"])
    for warning in d.get("warnings", []):
        if warning.get("tid") is not None:
            warning["tid"] = token(warning["tid"])
    return d


def projection_self_test():
    sample = {
        "events": [
            {"sequence": 1, "tid": 100, "event_type": "process_spawn", "child_tid": 200, "mechanism": "fork"},
            {"sequence": 2, "tid": 200, "event_type": "process_exec", "path": "/bin/true"},
        ],
        "warnings": [{"code": "sentinel", "tid": 200, "message": "sentinel"}],
    }
    projected = projection(sample)
    assert projected["events"][0]["tid"] == "T1"
    assert projected["events"][0]["child_tid"] == "T2"
    assert projected["events"][1]["tid"] == "T2"
    assert projected["warnings"][0]["tid"] == "T2"
    assert projected["events"][0]["sequence"] == 1


projection_self_test()
summary = []
for name, args, content in cases:
    reset_file(content)
    r1 = run(ref, args, out / f"{name}.reference1.json")
    reset_file(content)
    r2 = run(ref, args, out / f"{name}.reference2.json")
    pr1, pr2 = projection(r1), projection(r2)
    if pr1 != pr2:
        summary.append({"case": name, "reference_deterministic": False, "candidate_equal": None})
        (out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
        raise SystemExit(f"Lane A reference raw nondeterminism: {name}")

    reset_file(content)
    c = run(cand, args, out / f"{name}.candidate.json")
    pc = projection(c)
    equal = pr1 == pc
    summary.append({
        "case": name,
        "reference_deterministic": True,
        "candidate_equal": equal,
        "events": len(r1.get("events", [])),
        "complete": r1.get("complete"),
        "warnings": len(r1.get("warnings", [])),
    })
    if not equal:
        (out / f"{name}.reference.projection.json").write_text(json.dumps(pr1, indent=2, sort_keys=True) + "\n")
        (out / f"{name}.candidate.projection.json").write_text(json.dumps(pc, indent=2, sort_keys=True) + "\n")
        (out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
        raise SystemExit(f"Lane A reference/A2 raw mismatch: {name}")

(out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print(json.dumps(summary, indent=2, sort_keys=True))
