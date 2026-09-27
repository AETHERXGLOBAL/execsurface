import json
import os
import subprocess
import sys
from collections import Counter
from pathlib import Path

ref, cand, helper, fixture, evidence = sys.argv[1:]
out = Path(evidence) / "differential"
out.mkdir(parents=True, exist_ok=True)
fixed = "/tmp/execsurface-phase-fixed-file"

cases = [
    ("irrelevant", [helper, "getpid", "5000"], None),
    ("fileio", [helper, "fileio", fixed], b"fixture"),
    ("failed-open", [helper, "failed-open"], None),
    ("signal", [helper, "signal"], None),
    ("fork", [helper, "fork", fixed], b"fixture"),
    ("vfork", [helper, "vfork"], None),
    ("exec", [helper, "exec"], None),
    ("failed-exec", [helper, "failed-exec"], None),
    ("thread-exec", [helper, "thread-exec"], None),
    ("exit-group", [helper, "exit-group"], None),
    ("fixture-thread-read", [fixture, "thread-read", fixed, "8"], b"fixture"),
    ("fixture-spawn", [fixture, "spawn", "phase-aware-sentinel"], None),
]


def reset_file(content):
    try:
        os.unlink(fixed)
    except FileNotFoundError:
        pass
    if content is not None:
        Path(fixed).write_bytes(content)


def run(binary, args, path):
    with open(path, "w") as f:
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
    return json.loads(Path(path).read_text())


def normalize_child_ids(value):
    if isinstance(value, dict):
        return {
            k: (0 if k == "child_tid" else normalize_child_ids(v))
            for k, v in value.items()
        }
    if isinstance(value, list):
        return [normalize_child_ids(v) for v in value]
    return value


def projection(d):
    warnings = Counter(
        json.dumps(
            {"code": w.get("code"), "message": w.get("message")}, sort_keys=True
        )
        for w in d.get("warnings", [])
    )
    events = Counter(
        json.dumps(
            normalize_child_ids(e.get("kind")), sort_keys=True, separators=(",", ":")
        )
        for e in d.get("events", [])
    )
    return {
        "backend": d.get("backend"),
        "complete": d.get("complete"),
        "outcome": d.get("outcome"),
        "warnings": dict(sorted(warnings.items())),
        "events": dict(sorted(events.items())),
    }


summary = []
for name, args, content in cases:
    reset_file(content)
    rd = run(ref, args, out / f"{name}.reference.json")
    reset_file(content)
    cd = run(cand, args, out / f"{name}.candidate.json")
    rp, cp = projection(rd), projection(cd)
    ok = rp == cp
    (out / f"{name}.reference.projection.json").write_text(
        json.dumps(rp, indent=2, sort_keys=True) + "\n"
    )
    (out / f"{name}.candidate.projection.json").write_text(
        json.dumps(cp, indent=2, sort_keys=True) + "\n"
    )
    summary.append(
        {
            "case": name,
            "equal": ok,
            "reference_events": sum(rp["events"].values()),
            "candidate_events": sum(cp["events"].values()),
            "reference_complete": rp["complete"],
            "candidate_complete": cp["complete"],
        }
    )
    if not ok:
        print(json.dumps({"case": name, "reference": rp, "candidate": cp}, indent=2, sort_keys=True))
        (out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
        raise SystemExit(1)

(out / "summary.json").write_text(json.dumps(summary, indent=2, sort_keys=True) + "\n")
print(json.dumps(summary, indent=2, sort_keys=True))
