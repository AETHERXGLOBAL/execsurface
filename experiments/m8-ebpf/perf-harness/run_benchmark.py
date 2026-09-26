#!/usr/bin/env python3
import argparse
import datetime as dt
import hashlib
import json
import math
import os
import platform
import statistics
import subprocess
import sys
import time
from pathlib import Path

WORKLOADS = ["short_exec", "process_tree", "file_open", "high_event_rate"]
MODES = ["direct", "ptrace", "libbpf"]
HARD_EBPF_INCOMPLETE = {
    "incomplete_loss",
    "incomplete_limit",
    "incomplete_decode",
    "incomplete_collector",
    "incomplete_lifecycle",
}
ROTATIONS = [
    ["direct", "ptrace", "libbpf"],
    ["ptrace", "libbpf", "direct"],
    ["libbpf", "direct", "ptrace"],
]


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cli", required=True)
    parser.add_argument("--collector", required=True)
    parser.add_argument("--fixture", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--commit-sha", required=True)
    parser.add_argument("--samples", type=int, default=11)
    parser.add_argument("--warmups", type=int, default=2)
    args = parser.parse_args()
    if args.samples < 3:
        parser.error("--samples must be >= 3")
    if args.warmups < 0:
        parser.error("--warmups must be >= 0")
    if os.geteuid() != 0:
        parser.error("benchmark harness must run as root so all three modes share one privilege context")
    for path in (args.cli, args.collector, args.fixture):
        if not Path(path).is_file():
            parser.error(f"required executable does not exist: {path}")
    return args


def run_text(command):
    proc = subprocess.run(command, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return proc.returncode, proc.stdout.strip(), proc.stderr.strip()


def version(command):
    try:
        code, out, err = run_text(command)
        text = out or err
        return text.splitlines()[0] if code == 0 and text else None
    except OSError:
        return None


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def os_release():
    result = {}
    try:
        for line in Path("/etc/os-release").read_text(encoding="utf-8").splitlines():
            if "=" not in line:
                continue
            key, value = line.split("=", 1)
            result[key] = value.strip().strip('"')
    except OSError:
        pass
    return result


def cpu_model():
    try:
        for line in Path("/proc/cpuinfo").read_text(encoding="utf-8", errors="replace").splitlines():
            if line.lower().startswith("model name") and ":" in line:
                return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return None


def host_metadata(args):
    btf = Path("/sys/kernel/btf/vmlinux")
    return {
        "utc_timestamp": dt.datetime.now(dt.timezone.utc).isoformat(),
        "repository_commit_sha": args.commit_sha,
        "runner_name": os.environ.get("RUNNER_NAME"),
        "runner_os": os.environ.get("RUNNER_OS"),
        "runner_arch": os.environ.get("RUNNER_ARCH"),
        "os_release": os_release(),
        "kernel_release": platform.release(),
        "machine": platform.machine(),
        "cpu_model": cpu_model(),
        "logical_cpu_count": os.cpu_count(),
        "effective_uid": os.geteuid(),
        "rustc": version(["rustc", "--version"]),
        "cargo": version(["cargo", "--version"]),
        "clang": version(["clang", "--version"]),
        "btf_vmlinux_readable": os.access(btf, os.R_OK),
        "btf_vmlinux_sha256": sha256_file(btf) if btf.is_file() and os.access(btf, os.R_OK) else None,
        "binary_sha256": {
            "execsurface": sha256_file(args.cli),
            "collector": sha256_file(args.collector),
            "fixture": sha256_file(args.fixture),
        },
    }


def command_for(mode, args, workload):
    target = [args.fixture, workload]
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


def parse_health(mode, proc):
    base = {
        "clean": proc.returncode == 0,
        "process_returncode": proc.returncode,
        "stdout_bytes": len(proc.stdout),
        "stderr_bytes": len(proc.stderr),
        "stdout_sha256": hashlib.sha256(proc.stdout).hexdigest(),
    }
    if proc.returncode != 0:
        base["error"] = proc.stderr.decode("utf-8", errors="replace")[-2000:]
        return base
    if mode == "direct":
        return base

    try:
        report = json.loads(proc.stdout)
    except json.JSONDecodeError as error:
        base["clean"] = False
        base["error"] = f"observer stdout is not JSON: {error}"
        return base

    if mode == "ptrace":
        complete = report.get("complete") is True
        base.update(
            {
                "complete": report.get("complete"),
                "warning_count": len(report.get("warnings", [])),
                "event_count": len(report.get("events", [])),
            }
        )
        base["clean"] = base["clean"] and complete
        return base

    completeness = report.get("completeness")
    dropped = report.get("dropped_events")
    collector_failure = report.get("collector_failure")
    lifecycle = report.get("lifecycle_drain_complete")
    base.update(
        {
            "completeness": completeness,
            "observation_complete": report.get("observation_complete"),
            "dropped_events": dropped,
            "lifecycle_drain_complete": lifecycle,
            "collector_failure": collector_failure,
            "warning_count": len(report.get("warnings", [])),
            "event_count": len(report.get("events", [])),
        }
    )
    base["clean"] = (
        base["clean"]
        and completeness not in HARD_EBPF_INCOMPLETE
        and dropped == 0
        and collector_failure is None
        and lifecycle is True
    )
    return base


def invoke(mode, args, workload):
    command = command_for(mode, args, workload)
    start = time.perf_counter_ns()
    proc = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stop = time.perf_counter_ns()
    health = parse_health(mode, proc)
    return {
        "mode": mode,
        "duration_ns": stop - start,
        "duration_ms": (stop - start) / 1_000_000.0,
        "health": health,
    }


def p90_nearest_rank(values):
    ordered = sorted(values)
    rank = max(1, math.ceil(0.90 * len(ordered)))
    return ordered[rank - 1]


def summarize(samples):
    clean = [sample for sample in samples if sample["health"]["clean"]]
    values = [sample["duration_ms"] for sample in clean]
    if len(clean) != len(samples) or not values:
        return {
            "eligible": False,
            "total_samples": len(samples),
            "clean_samples": len(clean),
            "reason": "one or more timed samples failed the clean evidence-health gate",
        }
    median = statistics.median(values)
    deviations = [abs(value - median) for value in values]
    return {
        "eligible": True,
        "total_samples": len(samples),
        "clean_samples": len(clean),
        "first_measured_ms": values[0],
        "median_ms": median,
        "min_ms": min(values),
        "max_ms": max(values),
        "mad_ms": statistics.median(deviations),
        "p90_ms": p90_nearest_rank(values),
    }


def main():
    args = parse_args()
    evidence = {
        "schema_version": 1,
        "claim_scope": "single-host end-to-end invocation measurements; not universal speedup evidence",
        "host": host_metadata(args),
        "protocol": {
            "warmups_per_mode_per_workload": args.warmups,
            "timed_samples_per_mode_per_workload": args.samples,
            "timed_mode_order": "rotating three-order schedule",
            "clock": "time.perf_counter_ns monotonic",
            "outlier_policy": "none removed",
            "p90_rule": "nearest-rank ceil(0.90*n)",
            "privilege_context": "all modes executed by the same root benchmark harness",
        },
        "workloads": {},
    }

    any_ineligible = False
    for workload in WORKLOADS:
        for mode in MODES:
            for _ in range(args.warmups):
                warmup = invoke(mode, args, workload)
                if not warmup["health"]["clean"]:
                    raise RuntimeError(
                        f"warmup failed health gate workload={workload} mode={mode}: {warmup['health']}"
                    )

        samples = {mode: [] for mode in MODES}
        for repetition in range(args.samples):
            for mode in ROTATIONS[repetition % len(ROTATIONS)]:
                sample = invoke(mode, args, workload)
                sample["repetition"] = repetition
                samples[mode].append(sample)

        summaries = {mode: summarize(samples[mode]) for mode in MODES}
        if not all(summary["eligible"] for summary in summaries.values()):
            any_ineligible = True

        ratios = None
        if all(summary["eligible"] for summary in summaries.values()):
            direct = summaries["direct"]["median_ms"]
            ptrace = summaries["ptrace"]["median_ms"]
            libbpf = summaries["libbpf"]["median_ms"]
            ratios = {
                "ptrace_over_direct": ptrace / direct,
                "libbpf_over_direct": libbpf / direct,
                "libbpf_vs_ptrace": libbpf / ptrace,
            }

        evidence["workloads"][workload] = {
            "samples": samples,
            "summary": summaries,
            "computed_ratios": ratios,
        }

    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if any_ineligible:
        print("M8_6_BENCHMARK_INCOMPLETE: at least one timed sample failed health gating", file=sys.stderr)
        return 3
    print("M8_6_BENCHMARK_PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
