#!/usr/bin/env python3
import argparse
import gzip
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

MODES = ("direct", "ptrace", "persistent")
ROTATIONS = tuple(MODES[i:] + MODES[:i] for i in range(len(MODES)))


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--cli", required=True)
    parser.add_argument("--persistent", required=True)
    parser.add_argument("--adapter", required=True)
    parser.add_argument("--workdir", required=True)
    parser.add_argument("--command", required=True)
    parser.add_argument("--target-id", required=True)
    parser.add_argument("--revision", required=True)
    parser.add_argument("--source-sha", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--raw-dir", required=True)
    parser.add_argument("--warmups", type=int, default=3)
    parser.add_argument("--samples", type=int, default=15)
    parser.add_argument("--timeout-seconds", type=float, default=180.0)
    args = parser.parse_args()
    if os.geteuid() != 0:
        parser.error("E4 harness must run as root so all timed modes share one privilege context")
    if args.warmups != 3 or args.samples != 15:
        parser.error("E4 frozen protocol requires exactly 3 warmups and 15 measured samples")
    for path in (args.cli, args.persistent, args.adapter):
        if not Path(path).is_file():
            parser.error(f"required executable missing: {path}")
    if not Path(args.workdir).is_dir():
        parser.error(f"workdir missing: {args.workdir}")
    return args


def sha256_bytes(data):
    return hashlib.sha256(data).hexdigest()


def p90(values):
    ordered = sorted(values)
    return ordered[max(0, math.ceil(0.90 * len(ordered)) - 1)]


def summarize(values):
    median = statistics.median(values)
    return {
        "count": len(values),
        "median_ms": median,
        "mad_ms": statistics.median(abs(x - median) for x in values),
        "min_ms": min(values),
        "max_ms": max(values),
        "p90_ms": p90(values),
    }


def persist_bytes(raw_dir, stem, suffix, data):
    path = raw_dir / f"{stem}.{suffix}.gz"
    with gzip.open(path, "wb", compresslevel=6) as handle:
        handle.write(data)
    return str(path)


def run_subprocess(command, *, cwd, env, timeout):
    started = time.perf_counter_ns()
    try:
        proc = subprocess.run(
            command,
            cwd=cwd,
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
        )
        timed_out = False
    except subprocess.TimeoutExpired as error:
        stopped = time.perf_counter_ns()
        return {
            "returncode": 124,
            "stdout": error.stdout or b"",
            "stderr": error.stderr or b"",
            "wall_ms": (stopped - started) / 1_000_000.0,
            "timed_out": True,
        }
    stopped = time.perf_counter_ns()
    return {
        "returncode": proc.returncode,
        "stdout": proc.stdout,
        "stderr": proc.stderr,
        "wall_ms": (stopped - started) / 1_000_000.0,
        "timed_out": timed_out,
    }


def persistent_health(report):
    sessions = report.get("sessions", [])
    authority = report.get("authority", {})
    session_rows = []
    all_sessions_healthy = len(sessions) == 2
    for session in sessions:
        checks = {
            "outcome": session.get("outcome") == {"exit_code": 0, "signal": None},
            "clean_for_m8_7a": session.get("clean_for_m8_7a") is True,
            "lifecycle_drain_complete": session.get("lifecycle_drain_complete") is True,
            "active_remaining": session.get("active_remaining") == 0,
            "stale_epoch_events": session.get("stale_epoch_events") == 0,
            "integrity_errors": session.get("integrity_errors") == 0,
            "decode_error_delta": session.get("decode_error_delta") == 0,
            "producer_drop_delta": session.get("producer_drop_delta") == 0,
            "routing_error_delta": session.get("routing_error_delta") == 0,
            "event_limit_hit": session.get("event_limit_hit") is False,
            "membership_map_empty": session.get("membership_map_empty") is True,
            "pending_mechanism_map_empty": session.get("pending_mechanism_map_empty") is True,
        }
        healthy = all(checks.values())
        all_sessions_healthy = all_sessions_healthy and healthy
        session_rows.append(
            {
                "epoch": session.get("epoch"),
                "elapsed_ms": session.get("elapsed_ms"),
                "event_count": session.get("event_count"),
                "spawn_count": session.get("spawn_count"),
                "exec_count": session.get("exec_count"),
                "exit_count": session.get("exit_count"),
                "checks": checks,
                "healthy": healthy,
            }
        )

    final_checks = {
        "session_count": len(sessions) == 2,
        "unexpected_without_session": report.get("unexpected_without_session") == 0,
        "decode_errors_total": report.get("decode_errors_total") == 0,
        "final_membership_map_empty": report.get("final_membership_map_empty") is True,
        "final_pending_mechanism_map_empty": report.get("final_pending_mechanism_map_empty") is True,
        "ptrace_correctness_reference": authority.get("ptrace_correctness_reference") is True,
        "full_surface_comparable_false": authority.get("full_surface_comparable") is False,
        "ebpf_pass_authorized_false": authority.get("ebpf_pass_authorized") is False,
        "product_integration_authorized_false": authority.get("product_integration_authorized") is False,
    }
    return all_sessions_healthy and all(final_checks.values()), session_rows, final_checks


def invoke(mode, args, raw_dir, phase, index):
    stem = f"{phase}-{index:02d}-{mode}"
    env = os.environ.copy()
    cwd = args.workdir

    with tempfile.TemporaryDirectory(prefix="execsurface-e4-") as temp_dir:
        report_path = Path(temp_dir) / "persistent-report.json"
        if mode == "direct":
            command = ["/bin/bash", "-lc", args.command]
        elif mode == "ptrace":
            command = [args.cli, "observe", "--", "/bin/bash", "-lc", args.command]
        elif mode == "persistent":
            env["EXECSURFACE_E4_COMMAND"] = args.command
            command = [args.persistent, "--report", str(report_path), "--fixture", args.adapter]
        else:
            raise ValueError(mode)

        result = run_subprocess(command, cwd=cwd, env=env, timeout=args.timeout_seconds)
        stdout = result.pop("stdout")
        stderr = result.pop("stderr")
        row = {
            "mode": mode,
            "phase": phase,
            "index": index,
            **result,
            "stdout_bytes": len(stdout),
            "stderr_bytes": len(stderr),
            "stdout_sha256": sha256_bytes(stdout),
            "stderr_sha256": sha256_bytes(stderr),
        }
        persist_bytes(raw_dir, stem, "stdout", stdout)
        persist_bytes(raw_dir, stem, "stderr", stderr)

        if mode == "direct":
            row["clean"] = row["returncode"] == 0 and not row["timed_out"]
            return row

        if mode == "ptrace":
            try:
                report = json.loads(stdout)
            except (json.JSONDecodeError, UnicodeDecodeError) as error:
                row["clean"] = False
                row["error"] = f"invalid ptrace observation JSON: {error}"
                return row
            backend = report.get("backend", {})
            row.update(
                {
                    "ptrace_complete": report.get("complete"),
                    "ptrace_warnings": report.get("warnings"),
                    "ptrace_outcome": report.get("outcome"),
                    "ptrace_backend": backend.get("name"),
                    "ptrace_event_count": len(report.get("events", [])),
                }
            )
            row["clean"] = (
                row["returncode"] == 0
                and not row["timed_out"]
                and report.get("complete") is True
                and report.get("warnings") == []
                and report.get("outcome") == {"exit_code": 0, "signal": None}
                and backend.get("name") == "linux-ptrace-metadata-v2"
            )
            return row

        if not report_path.is_file():
            row["clean"] = False
            row["error"] = "persistent report missing"
            return row
        report_bytes = report_path.read_bytes()
        persist_bytes(raw_dir, stem, "report.json", report_bytes)
        try:
            report = json.loads(report_bytes)
        except (json.JSONDecodeError, UnicodeDecodeError) as error:
            row["clean"] = False
            row["error"] = f"invalid persistent report JSON: {error}"
            return row
        healthy, sessions, final_checks = persistent_health(report)
        row.update(
            {
                "session_count": len(sessions),
                "amortized_wall_per_session_ms": row["wall_ms"] / 2.0,
                "sessions": sessions,
                "internal_session_ms": [s.get("elapsed_ms") for s in sessions],
                "one_time_lifecycle": report.get("one_time_lifecycle"),
                "final_checks": final_checks,
                "report_sha256": sha256_bytes(report_bytes),
            }
        )
        row["clean"] = (
            row["returncode"] == 0 and not row["timed_out"] and healthy
        )
        return row


def main():
    args = parse_args()
    raw_dir = Path(args.raw_dir)
    raw_dir.mkdir(parents=True, exist_ok=True)
    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)

    evidence = {
        "schema_version": 1,
        "classification": "IN_PROGRESS",
        "target_id": args.target_id,
        "revision": args.revision,
        "source_sha": args.source_sha,
        "host": {
            "kernel": platform.release(),
            "machine": platform.machine(),
            "logical_cpu_count": os.cpu_count(),
            "effective_uid": os.geteuid(),
        },
        "protocol": {
            "modes": list(MODES),
            "warmups_per_mode": args.warmups,
            "measured_samples_per_mode": args.samples,
            "measured_order": "three-mode rotating schedule",
            "outlier_removal": "none",
            "sample_replacement": "none",
            "persistent_sessions_per_invocation": 2,
            "primary_persistent_metric": "invocation_wall_ms / 2",
            "threshold_percent": 10.0,
            "privilege_context": "all timed modes run by one root harness",
        },
        "warmups": [],
        "samples": {mode: [] for mode in MODES},
    }

    def checkpoint():
        output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    for mode in MODES:
        for i in range(1, args.warmups + 1):
            row = invoke(mode, args, raw_dir, "warmup", i)
            evidence["warmups"].append(row)
            checkpoint()
            if not row.get("clean"):
                evidence["classification"] = "E4_BLOCKED_BY_INCOMPLETE_EVIDENCE"
                evidence["blocked_at"] = {"phase": "warmup", "mode": mode, "index": i}
                checkpoint()
                raise RuntimeError(f"unhealthy E4 warmup: {mode} #{i}: {row}")

    for repetition in range(args.samples):
        for mode in ROTATIONS[repetition % len(ROTATIONS)]:
            row = invoke(mode, args, raw_dir, "measured", repetition + 1)
            evidence["samples"][mode].append(row)
            checkpoint()
            if not row.get("clean"):
                evidence["classification"] = "E4_BLOCKED_BY_INCOMPLETE_EVIDENCE"
                evidence["blocked_at"] = {
                    "phase": "measured",
                    "mode": mode,
                    "index": repetition + 1,
                }
                checkpoint()
                raise RuntimeError(f"unhealthy E4 measured sample: {mode} #{repetition + 1}: {row}")

    direct_values = [x["wall_ms"] for x in evidence["samples"]["direct"]]
    ptrace_values = [x["wall_ms"] for x in evidence["samples"]["ptrace"]]
    persistent_values = [
        x["amortized_wall_per_session_ms"] for x in evidence["samples"]["persistent"]
    ]
    internal_values = [
        value
        for row in evidence["samples"]["persistent"]
        for value in row["internal_session_ms"]
        if isinstance(value, (int, float))
    ]

    summary = {
        "direct": summarize(direct_values),
        "ptrace": summarize(ptrace_values),
        "persistent_amortized_per_session": summarize(persistent_values),
        "persistent_internal_session_diagnostic": summarize(internal_values),
    }
    ptrace_median = summary["ptrace"]["median_ms"]
    persistent_median = summary["persistent_amortized_per_session"]["median_ms"]
    reduction_percent = 100.0 * (ptrace_median - persistent_median) / ptrace_median
    threshold_pass = persistent_median < ptrace_median and reduction_percent >= 10.0

    evidence["summary"] = summary
    evidence["value"] = {
        "ptrace_median_ms": ptrace_median,
        "persistent_amortized_median_ms": persistent_median,
        "median_reduction_ms": ptrace_median - persistent_median,
        "median_reduction_percent": reduction_percent,
        "threshold_percent": 10.0,
        "candidate_lower_than_ptrace": persistent_median < ptrace_median,
        "threshold_pass": threshold_pass,
    }
    evidence["classification"] = (
        "TARGET_VALUE_ESTABLISHED" if threshold_pass else "TARGET_VALUE_NOT_ESTABLISHED"
    )
    checkpoint()

    print(json.dumps({"target_id": args.target_id, "classification": evidence["classification"], "summary": summary, "value": evidence["value"]}, indent=2, sort_keys=True))
    return 0 if threshold_pass else 10


if __name__ == "__main__":
    raise SystemExit(main())
