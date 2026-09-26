#!/usr/bin/env python3
import argparse
import hashlib
import json
import os
import platform
import statistics
import subprocess
import tempfile
import time
from pathlib import Path

HARD_INCOMPLETE = {
    "incomplete_loss",
    "incomplete_limit",
    "incomplete_decode",
    "incomplete_collector",
    "incomplete_lifecycle",
}
MODES = ["direct", "ptrace", "libbpf_per_invocation"]


def parse_args():
    p = argparse.ArgumentParser()
    p.add_argument("--cli", required=True)
    p.add_argument("--collector", required=True)
    p.add_argument("--workdir", required=True)
    p.add_argument("--shell-script", required=True)
    p.add_argument("--output", required=True)
    p.add_argument("--commit-sha", required=True)
    p.add_argument("--workload", required=True)
    p.add_argument("--warmups", type=int, default=3)
    p.add_argument("--samples", type=int, default=15)
    p.add_argument("--timeout", type=float, default=180.0)
    args = p.parse_args()
    if os.geteuid() != 0:
        p.error("value-screen harness must run as root so every mode has the same privilege context")
    if args.warmups != 3 or args.samples != 15:
        p.error("M9.1 preregistration requires exactly 3 warmups and 15 measured samples per mode")
    for item in (args.cli, args.collector):
        if not Path(item).is_file():
            p.error(f"missing executable: {item}")
    if not Path(args.workdir).is_dir():
        p.error("workdir does not exist")
    return args


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def median(values):
    return float(statistics.median(values))


def invoke(mode, args, ordinal, phase):
    base = ["/bin/bash", "-lc", args.shell_script]
    report_digest = None
    report_health = None
    if mode == "direct":
        command = base
    elif mode == "ptrace":
        command = [args.cli, "observe", "--"] + base
    elif mode == "libbpf_per_invocation":
        temp = tempfile.NamedTemporaryFile(prefix="m9-ebpf-report-", suffix=".json", delete=False)
        report_path = Path(temp.name)
        temp.close()
        report_path.unlink(missing_ok=True)
        command = [args.collector, "--report", str(report_path), "--"] + base
    else:
        raise ValueError(mode)

    started = time.perf_counter_ns()
    proc = subprocess.run(
        command,
        cwd=args.workdir,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        timeout=args.timeout,
    )
    wall_ms = (time.perf_counter_ns() - started) / 1_000_000.0
    row = {
        "mode": mode,
        "phase": phase,
        "ordinal": ordinal,
        "wall_ms": wall_ms,
        "returncode": proc.returncode,
        "stdout_bytes": len(proc.stdout),
        "stderr_bytes": len(proc.stderr),
        "clean": proc.returncode == 0,
    }

    if mode == "ptrace":
        try:
            report = json.loads(proc.stdout)
            outcome = report.get("outcome", {})
            row["ptrace_complete"] = report.get("complete")
            row["ptrace_exit_code"] = outcome.get("exit_code")
            row["ptrace_signal"] = outcome.get("signal")
            row["ptrace_event_count"] = len(report.get("events", []))
            row["ptrace_stdout_sha256"] = hashlib.sha256(proc.stdout).hexdigest()
            row["clean"] = (
                row["clean"]
                and report.get("complete") is True
                and outcome.get("exit_code") == 0
                and outcome.get("signal") is None
            )
        except Exception as exc:
            row["clean"] = False
            row["error"] = f"ptrace report parse failure: {exc}"

    if mode == "libbpf_per_invocation":
        try:
            if not report_path.is_file():
                raise RuntimeError("collector report missing")
            report_digest = sha256_file(report_path)
            report = json.loads(report_path.read_text(encoding="utf-8"))
            completeness = report.get("completeness")
            dropped = report.get("dropped_events")
            lifecycle = report.get("lifecycle_drain_complete")
            failure = report.get("collector_failure")
            report_health = {
                "report_sha256": report_digest,
                "completeness": completeness,
                "dropped_events": dropped,
                "lifecycle_drain_complete": lifecycle,
                "collector_failure": failure,
                "event_count": len(report.get("events", [])),
            }
            row.update(report_health)
            row["clean"] = (
                row["clean"]
                and completeness not in HARD_INCOMPLETE
                and dropped == 0
                and lifecycle is True
                and failure is None
            )
        except Exception as exc:
            row["clean"] = False
            row["error"] = f"eBPF health failure: {exc}"
        finally:
            report_path.unlink(missing_ok=True)

    if not row["clean"]:
        row["stderr_tail"] = proc.stderr.decode("utf-8", errors="replace")[-4000:]
    return row


def main():
    args = parse_args()
    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)

    evidence = {
        "schema_version": 1,
        "experiment": "M9.1 same-workload eBPF value screen",
        "workload": args.workload,
        "repository_commit_sha": args.commit_sha,
        "command": ["/bin/bash", "-lc", args.shell_script],
        "workdir": args.workdir,
        "host": {
            "kernel": platform.release(),
            "machine": platform.machine(),
            "effective_uid": os.geteuid(),
            "btf_vmlinux_readable": os.access("/sys/kernel/btf/vmlinux", os.R_OK),
            "logical_cpu_count": os.cpu_count(),
        },
        "binary_sha256": {
            "execsurface": sha256_file(args.cli),
            "libbpf_per_invocation": sha256_file(args.collector),
        },
        "protocol": {
            "warmups_per_mode": 3,
            "measured_samples_per_mode": 15,
            "mode_order": "rotating direct/ptrace/libbpf_per_invocation",
            "outlier_policy": "none removed",
            "clock": "time.perf_counter_ns monotonic",
            "privilege_context": "all modes run from the same root harness",
            "value_threshold": {
                "relative": "median_eBPF <= 0.80 * median_ptrace",
                "absolute": "median_ptrace - median_eBPF >= 250 ms",
            },
        },
        "authority": {
            "ptrace_correctness_reference": True,
            "full_surface_comparable": False,
            "ebpf_learn_authorized": False,
            "ebpf_check_authorized": False,
            "ebpf_pass_authorized": False,
            "backend_auto_select_authorized": False,
            "baseline_interchange_authorized": False,
            "public_integration_authorized": False,
        },
        "warmups": [],
        "samples": {mode: [] for mode in MODES},
    }

    for mode in MODES:
        for i in range(3):
            row = invoke(mode, args, i + 1, "warmup")
            evidence["warmups"].append(row)
            if not row["clean"]:
                evidence["status"] = "PARTIAL_NO_VALUE_CLAIM"
                evidence["failure"] = f"warmup health gate failed for {mode}"
                output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
                raise RuntimeError(evidence["failure"])

    rotations = [MODES[i:] + MODES[:i] for i in range(len(MODES))]
    for i in range(15):
        for mode in rotations[i % len(rotations)]:
            row = invoke(mode, args, i + 1, "measured")
            evidence["samples"][mode].append(row)
            if not row["clean"]:
                evidence["status"] = "PARTIAL_NO_VALUE_CLAIM"
                evidence["failure"] = f"measured health gate failed for {mode} sample {i+1}"
                output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
                raise RuntimeError(evidence["failure"])

    medians = {
        mode: median([row["wall_ms"] for row in rows])
        for mode, rows in evidence["samples"].items()
    }
    ptrace = medians["ptrace"]
    ebpf = medians["libbpf_per_invocation"]
    absolute_gain = ptrace - ebpf
    relative_ratio = ebpf / ptrace
    value_signal = ebpf <= 0.80 * ptrace and absolute_gain >= 250.0
    evidence["summary"] = {
        "median_direct_ms": medians["direct"],
        "median_ptrace_ms": ptrace,
        "median_ebpf_ms": ebpf,
        "ebpf_over_ptrace_ratio": relative_ratio,
        "ptrace_minus_ebpf_ms": absolute_gain,
        "value_threshold_passed": value_signal,
    }
    evidence["status"] = "VALUE_SIGNAL_FOR_FURTHER_RESEARCH" if value_signal else "NO_VALUE_SIGNAL"
    output.write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"workload": args.workload, "status": evidence["status"], **evidence["summary"]}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
