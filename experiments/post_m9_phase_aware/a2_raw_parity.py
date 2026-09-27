import copy
import json
import os
import subprocess
import sys
from pathlib import Path

if len(sys.argv) != 6:
    raise SystemExit("usage: a2_raw_parity.py REF CAND HELPER FIXTURE EVIDENCE")

ref, cand, helper, fixture, evidence = sys.argv[1:]
out = Path(evidence) / "raw-parity"
out.mkdir(parents=True, exist_ok=True)
fixed = "/tmp/execsurface-a2-fixed"

cases = [
    ("irrelevant", [helper, "getpid", "5000"], None),
    ("fileio", [helper, "fileio", fixed], b"fixture"),
    ("failed-open", [helper, "failed-open"], None),
    ("signal", [helper, "signal"], None),
    ("restart", [helper, "restart"], None),
    ("fork", [helper, "fork", fixed], b"fixture"),
    ("vfork", [helper, "vfork"], None),
    ("exec", [helper, "exec"], None),
    ("failed-exec", [helper, "failed-exec"], None),
    ("thread-exec", [helper, "thread-exec"], None),
    ("exit-group", [helper, "exit-group"], None),
    ("fixture-thread-read", [fixture, "thread-read", fixed, "8"], b"fixture"),
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
        event["tid"] = token(event.get("tid"))
        kind = event.get("kind")
        if isinstance(kind, dict) and "ProcessSpawn" in kind:
            payload = kind["ProcessSpawn"]
            if isinstance(payload, dict) and "child_tid" in payload:
                payload["child_tid"] = token(payload.get("child_tid"))
    for warning in d.get("warnings", []):
        if warning.get("tid") is not None:
            warning["tid"] = token(warning.get("tid"))
    return d


summary = []
for name, args, content in cases:
    reset_file(content)
    r1 = run(ref, args, out / f"{name}.reference1.json")
    reset_file(content)
    r2 = run(ref, args, out / f"{name}.reference2.json")
    pr1, pr2 = projection(r1), projection(r2)
    deterministic = pr1 == pr2
    if not deterministic:
        summary.append(
            {"case": name, "reference_deterministic": False, "candidate_equal": None}
        )
        (out / "summary.json").write_text(
            json.dumps(summary, indent=2, sort_keys=True) + "\n"
        )
        raise SystemExit(
            f"reference nondeterminism under frozen raw comparator: {name}"
        )

    reset_file(content)
    c = run(cand, args, out / f"{name}.candidate.json")
    pc = projection(c)
    equal = pr1 == pc
    summary.append(
        {
            "case": name,
            "reference_deterministic": True,
            "candidate_equal": equal,
            "events": len(r1.get("events", [])),
            "complete": r1.get("complete"),
            "warnings": len(r1.get("warnings", [])),
        }
    )
    (out / f"{name}.reference.projection.json").write_text(
        json.dumps(pr1, indent=2, sort_keys=True) + "\n"
    )
    (out / f"{name}.candidate.projection.json").write_text(
        json.dumps(pc, indent=2, sort_keys=True) + "\n"
    )
    if not equal:
        (out / "summary.json").write_text(
            json.dumps(summary, indent=2, sort_keys=True) + "\n"
        )
        raise SystemExit(f"reference/A2 raw parity mismatch: {name}")

(out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print(json.dumps(summary, indent=2, sort_keys=True))
