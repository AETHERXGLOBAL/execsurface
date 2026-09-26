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
import tempfile
import time
from pathlib import Path

MODES = ["direct", "ptrace", "libbpf_per_invocation", "persistent_two_session"]
HARD_EBPF_INCOMPLETE = {
    "incomplete_loss",
    "incomplete_limit",
    "incomplete_decode",
    "incomplete_collector",
    "incomplete_lifecycle",
}
ROTATIONS = [MODES[index:] + MODES[:index] for index in range(len(MODES))]


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cli", required=True)
    parser.add_argument("--collector", required=True)
    parser.add_argument("--persistent", required=True)
    parser.add_argument("--fixture", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--commit-sha", required=True)
    parser.add_argument("--warmups", type=int, default=3)
    parser.add_argument("--samples", type=int, default=15)
    args = parser.parse_args()
    if args.warmups < 0:
        parser.error("--warmups must be >= 0")
    if args.samples < 5:
        parser.error("--samples must be >= 5")
    if os.geteuid() != 0:
        parser.error("M8.7c harness must run as root so every mode shares one privilege context")
    for value in (args.cli, args.collector, args.persistent, args.fixture):
        if not Path(value).is_file():
            parser.error(f"required executable does not exist: {value}")
    return args


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def os_release():
    data = {}
    try:
        for line in Path("/etc/os-release").read_text(encoding="utf-8").splitlines():
            if "=" in line:
                key, value = line.split("=", 1)
                data[key] = value.strip().strip('"')
    except OSError:
        pass
    return data


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
        "logical_cpu_count": os.cpu_count(),
        "effective_uid": os.geteuid(),
        "btf_vmlinux_readable": os.access(btf, os.R_OK),
        "binary_sha256": {
            "execsurface": sha256_file(args.cli),
            "per_invocation_collector": sha256_file(args.collector),
            "persistent_observer": sha256_file(args.persistent),
            "benchmark_fixture": sha256_file(args.fixture),
        },
    }


def p90_nearest_rank(values):
    ordered = sorted(values)
    return ordered[max(0, math.ceil(0.90 * len(ordered)) - 1)]


def summarize_values(values):
    median = statistics.median(values)
    return {
        "count": len(values),
        "median_ms": median,
        "min_ms": min(values),
        "max_ms": max(values),
        "mad_ms": statistics.median(abs(value - median) for value in values),
        "p90_ms": p90_nearest_rank(values),
    }


def read_json(path):
    return json.loads(Path(path).read_text(encoding="utf-8"))


def invoke(mode, args):
    with tempfile.TemporaryDirectory(prefix="m8-7c-") as temp_dir:
        report_path = Path(temp_dir) / "report.json"
        if mode == "direct":
            command = [args.fixture, "tree"]
        elif mode == "ptrace":
            command = [args.cli, "observe", "--", args.fixture, "tree"]
        elif mode == "libbpf_per_invocation":
            command = [args.collector, "--report", str(report_path), "--", args.fixture, "tree"]
        elif mode == "persistent_two_session":
            command = [args.persistent, "--report", str(report_path), "--fixture", args.fixture]
        else:
            raise ValueError(mode)

        started = time.perf_counter_ns()
        proc = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        stopped = time.perf_counter_ns()
        wall_ms = (stopped - started) / 1_000_000.0

        sample = {
            "mode": mode,
            "wall_ms": wall_ms,
            "returncode": proc.returncode,
            "stdout_bytes": len(proc.stdout),
            "stderr_bytes": len(proc.stderr),
            "clean": proc.returncode == 0,
        }

        if mode == "direct":
            return sample

        if mode == "ptrace":
            try:
                report = json.loads(proc.stdout)
            except json.JSONDecodeError as error:
                sample["clean"] = False
                sample["error"] = f"ptrace stdout is not JSON: {error}"
                return sample
            sample["ptrace_complete"] = report.get("complete")
            sample["ptrace_event_count"] = len(report.get("events", []))
            sample["clean"] = sample["clean"] and report.get("complete") is True
            return sample

        if not report_path.is_file():
            sample["clean"] = False
            sample["error"] = "observer report file missing"
            return sample

        report = read_json(report_path)
        if mode == "libbpf_per_invocation":
            completeness = report.get("completeness")
            dropped = report.get("dropped_events")
            lifecycle = report.get("lifecycle_drain_complete")
            collector_failure = report.get("collector_failure")
            sample.update(
                {
                    "completeness": completeness,
                    "dropped_events": dropped,
                    "lifecycle_drain_complete": lifecycle,
                    "collector_failure": collector_failure,
                    "event_count": len(report.get("events", [])),
                }
            )
            sample["clean"] = (
                sample["clean"]
                and completeness not in HARD_EBPF_INCOMPLETE
                and dropped == 0
                and lifecycle is True
                and collector_failure is None
            )
            return sample

        sessions = report.get("sessions", [])
        authority = report.get("authority", {})
        session_health = []
        for session in sessions:
            healthy = (
                session.get("clean_for_m8_7a") is True
                and session.get("lifecycle_drain_complete") is True
                and session.get("active_remaining") == 0
                and session.get("stale_epoch_events") == 0
                and session.get("integrity_errors") == 0
                and session.get("decode_error_delta") == 0
                and session.get("producer_drop_delta") == 0
                and session.get("routing_error_delta") == 0
                and session.get("event_limit_hit") is False
                and session.get("membership_map_empty") is True
                and session.get("pending_mechanism_map_empty") is True
                and session.get("spawn_count") == 8
                and session.get("exec_count") == 9
                and session.get("exit_count") == 9
                and session.get("event_count") == 26
            )
            session_health.append(healthy)

        sample.update(
            {
                "session_count": len(sessions),
                "amortized_wall_per_session_ms": wall_ms / 2.0,
                "internal_session_ms": [session.get("elapsed_ms") for session in sessions],
                "session_health": session_health,
                "one_time_lifecycle": report.get("one_time_lifecycle"),
                "unexpected_without_session": report.get("unexpected_without_session"),
                "decode_errors_total": report.get("decode_errors_total"),
                "final_membership_map_empty": report.get("final_membership_map_empty"),
                "final_pending_mechanism_map_empty": report.get("final_pending_mechanism_map_empty"),
                "authority": authority,
            }
        )
        sample["clean"] = (
            sample["clean"]
            and len(sessions) == 2
            and all(session_health)
            and report.get("unexpected_without_session") == 0
            and report.get("decode_errors_total") == 0
            and report.get("final_membership_map_empty") is True
            and report.get("final_pending_mechanism_map_empty") is True
            and authority.get("ptrace_correctness_reference") is True
            and authority.get("full_surface_comparable") is False
            and authority.get("ebpf_pass_authorized") is False
            and authority.get("product_integration_authorized") is False
        )
        return sample


def main():
    args = parse_args()
    evidence = {
        "schema_version": 1,
        "claim_scope": "exact-host M8.7c measurement; no universal speedup or production claim",
        "host": host_metadata(args),
        "protocol": {
            "warmups_per_mode": args.warmups,
            "timed_invocations_per_mode": args.samples,
            "persistent_sessions_per_invocation": 2,
            "timed_mode_order": "rotating four-order schedule",
            "clock": "time.perf_counter_ns monotonic; persistent internal sessions use Rust Instant",
            "outlier_policy": "none removed",
            "p90_rule": "nearest-rank ceil(0.90*n)",
            "privilege_context": "all modes run by the same root harness",
            "persistent_barrier_bias": "persistent mode includes its required bootstrap barrier exec; no subtraction",
        },
        "warmups": [],
        "samples": {mode: [] for mode in MODES},
    }

    for mode in MODES:
        for repetition in range(args.warmups):
            sample = invoke(mode, args)
            sample["warmup"] = repetition
            evidence["warmups"].append(sample)
            if not sample["clean"]:
                raise RuntimeError(f"warmup failed mode={mode}: {sample}")

    for repetition in range(args.samples):
        for mode in ROTATIONS[repetition % len(ROTATIONS)]:
            sample = invoke(mode, args)
            sample["repetition"] = repetition
            evidence["samples"][mode].append(sample)

    for mode, samples in evidence["samples"].items():
        if len(samples) != args.samples or not all(sample["clean"] for sample in samples):
            Path(args.output).write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n", encoding="utf-8")
            raise RuntimeError(f"timed sample failed health gate mode={mode}")

    summary = {
        "direct": summarize_values([row["wall_ms"] for row in evidence["samples"]["direct"]]),
        "ptrace": summarize_values([row["wall_ms"] for row in evidence["samples"]["ptrace"]]),
        "libbpf_per_invocation": summarize_values(
            [row["wall_ms"] for row in evidence["samples"]["libbpf_per_invocation"]]
        ),
        "persistent_two_session_amortized": summarize_values(
            [row["amortized_wall_per_session_ms"] for row in evidence["samples"]["persistent_two_session"]]
        ),
        "persistent_internal_session": summarize_values(
            [
                value
                for row in evidence["samples"]["persistent_two_session"]
                for value in row["internal_session_ms"]
            ]
        ),
    }
    evidence["summary"] = summary

    libbpf = summary["libbpf_per_invocation"]["median_ms"]
    persistent_amortized = summary["persistent_two_session_amortized"]["median_ms"]
    persistent_internal = summary["persistent_internal_session"]["median_ms"]
    evidence["ratios"] = {
        "persistent_amortized_over_per_invocation_libbpf": persistent_amortized / libbpf,
        "persistent_internal_over_per_invocation_libbpf": persistent_internal / libbpf,
        "persistent_amortized_over_ptrace": persistent_amortized / summary["ptrace"]["median_ms"],
        "per_invocation_libbpf_over_ptrace": libbpf / summary["ptrace"]["median_ms"],
    }

    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    print("M8_7C_PERFORMANCE_EVIDENCE_PASS")
    for key, row in summary.items():
        print(key, "median_ms=", round(row["median_ms"], 6), "mad_ms=", round(row["mad_ms"], 6), "p90_ms=", round(row["p90_ms"], 6))
    print("ratios=", {key: round(value, 6) for key, value in evidence["ratios"].items()})
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
