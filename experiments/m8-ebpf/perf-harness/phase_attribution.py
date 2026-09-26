#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import statistics
import subprocess
import tempfile
import time
from pathlib import Path

MODES = ["direct", "ptrace", "libbpf"]
ROTATIONS = [
    ["direct", "ptrace", "libbpf"],
    ["ptrace", "libbpf", "direct"],
    ["libbpf", "direct", "ptrace"],
]
HARD_EBPF_INCOMPLETE = {
    "incomplete_loss",
    "incomplete_limit",
    "incomplete_decode",
    "incomplete_collector",
    "incomplete_lifecycle",
}


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cli", required=True)
    parser.add_argument("--collector", required=True)
    parser.add_argument("--fixture", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--commit-sha", required=True)
    parser.add_argument("--samples", type=int, default=7)
    parser.add_argument("--warmups", type=int, default=1)
    args = parser.parse_args()
    if os.geteuid() != 0:
        parser.error("phase-attribution harness must run as root for equal privilege context")
    if args.samples < 3:
        parser.error("--samples must be >= 3")
    return args


def command_for(mode, args, marker_path):
    target = [args.fixture, marker_path]
    if mode == "direct":
        return target
    if mode == "ptrace":
        return [args.cli, "observe", "--", *target]
    if mode == "libbpf":
        return [
            args.cli,
            "observe",
            "--backend",
            "experimental-libbpf",
            "--collector",
            args.collector,
            "--",
            *target,
        ]
    raise ValueError(mode)


def read_target_markers(path):
    text = Path(path).read_text(encoding="utf-8").strip()
    fields = text.split()
    if len(fields) != 2:
        raise RuntimeError(f"invalid target marker file: {text!r}")
    start_ns, end_ns = map(int, fields)
    if start_ns <= 0 or end_ns < start_ns:
        raise RuntimeError(f"invalid target marker range: {start_ns} {end_ns}")
    return start_ns, end_ns


def observer_clean(mode, proc):
    if proc.returncode != 0:
        return False, {"returncode": proc.returncode, "stderr": proc.stderr.decode("utf-8", errors="replace")[-1000:]}
    if mode == "direct":
        return True, {"returncode": 0}
    try:
        report = json.loads(proc.stdout)
    except json.JSONDecodeError as error:
        return False, {"returncode": 0, "error": f"invalid JSON: {error}"}
    if mode == "ptrace":
        return report.get("complete") is True, {
            "complete": report.get("complete"),
            "event_count": len(report.get("events", [])),
        }
    completeness = report.get("completeness")
    clean = (
        completeness not in HARD_EBPF_INCOMPLETE
        and report.get("dropped_events") == 0
        and report.get("collector_failure") is None
        and report.get("lifecycle_drain_complete") is True
    )
    return clean, {
        "completeness": completeness,
        "dropped_events": report.get("dropped_events"),
        "event_count": len(report.get("events", [])),
        "lifecycle_drain_complete": report.get("lifecycle_drain_complete"),
    }


def invoke(mode, args, marker_path):
    command = command_for(mode, args, marker_path)
    outer_start_ns = time.monotonic_ns()
    proc = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    outer_end_ns = time.monotonic_ns()
    clean, health = observer_clean(mode, proc)
    if not Path(marker_path).is_file():
        clean = False
        health["marker_missing"] = True
        target_start_ns = target_end_ns = 0
    else:
        target_start_ns, target_end_ns = read_target_markers(marker_path)

    pre_ns = target_start_ns - outer_start_ns if target_start_ns else -1
    target_ns = target_end_ns - target_start_ns if target_start_ns else -1
    post_ns = outer_end_ns - target_end_ns if target_end_ns else -1
    total_ns = outer_end_ns - outer_start_ns
    if min(pre_ns, target_ns, post_ns) < 0:
        clean = False
        health["clock_order_invalid"] = True

    return {
        "mode": mode,
        "clean": clean,
        "pre_target_ms": pre_ns / 1_000_000.0,
        "target_runtime_ms": target_ns / 1_000_000.0,
        "post_target_ms": post_ns / 1_000_000.0,
        "total_ms": total_ns / 1_000_000.0,
        "health": health,
    }


def summarize(samples, field):
    values = [sample[field] for sample in samples if sample["clean"]]
    if len(values) != len(samples) or not values:
        return {"eligible": False, "clean_samples": len(values), "total_samples": len(samples)}
    median = statistics.median(values)
    mad = statistics.median(abs(value - median) for value in values)
    return {
        "eligible": True,
        "median_ms": median,
        "mad_ms": mad,
        "min_ms": min(values),
        "max_ms": max(values),
    }


def main():
    args = parse_args()
    evidence = {
        "schema_version": 1,
        "claim_scope": "external shared-monotonic-clock phase attribution for current per-invocation architecture",
        "repository_commit_sha": args.commit_sha,
        "method": {
            "outer_clock": "Python time.monotonic_ns",
            "target_clock": "C clock_gettime(CLOCK_MONOTONIC)",
            "pre_target": "outer launch start to target main entry",
            "target_runtime": "target main entry to target pre-exit marker",
            "post_target": "target pre-exit marker to observer process return",
            "privilege_context": "all modes launched by one root harness process",
        },
        "samples": {mode: [] for mode in MODES},
        "summary": {},
    }

    with tempfile.TemporaryDirectory(prefix="execsurface-m86-phase-") as tempdir:
        for mode in MODES:
            for warmup in range(args.warmups):
                marker = str(Path(tempdir) / f"warmup-{mode}-{warmup}.txt")
                sample = invoke(mode, args, marker)
                if not sample["clean"]:
                    raise RuntimeError(f"phase warmup failed mode={mode}: {sample}")

        for repetition in range(args.samples):
            for mode in ROTATIONS[repetition % len(ROTATIONS)]:
                marker = str(Path(tempdir) / f"sample-{repetition}-{mode}.txt")
                sample = invoke(mode, args, marker)
                sample["repetition"] = repetition
                evidence["samples"][mode].append(sample)

    for mode in MODES:
        mode_samples = evidence["samples"][mode]
        evidence["summary"][mode] = {
            field: summarize(mode_samples, field)
            for field in ("pre_target_ms", "target_runtime_ms", "post_target_ms", "total_ms")
        }
        if not all(row["eligible"] for row in evidence["summary"][mode].values()):
            raise SystemExit(f"phase attribution contains ineligible samples for {mode}")

    payload = json.dumps(evidence, indent=2, sort_keys=True) + "\n"
    Path(args.output).write_text(payload, encoding="utf-8")
    print("M8_6_PHASE_ATTRIBUTION_PASS")
    for mode in MODES:
        row = evidence["summary"][mode]
        print(
            mode,
            "pre=", round(row["pre_target_ms"]["median_ms"], 6),
            "target=", round(row["target_runtime_ms"]["median_ms"], 6),
            "post=", round(row["post_target_ms"]["median_ms"], 6),
            "total=", round(row["total_ms"]["median_ms"], 6),
        )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
