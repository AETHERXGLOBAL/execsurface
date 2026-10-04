#!/usr/bin/env python3
"""Reproduce the bounded P8-A3 Alpha.5 raw-evidence interoperability gap.

This is evidence-only. It does not change ExecSurface product semantics.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import subprocess
import sys
import tempfile

ABSENT_WRITE_RESULT_FIELDS = {
    "bytes_transferred",
    "byte_count",
    "result",
    "return_value",
    "syscall_result",
    "success",
}
TYPED_HEALTH_FIELDS = {"collection_health", "health", "completeness"}


def fail(message: str) -> None:
    raise SystemExit(message)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()

    binary = args.binary.resolve()
    if not binary.is_file():
        fail("selected-binary-missing")

    with tempfile.TemporaryDirectory(prefix="execsurface-p8-a3-") as raw_tmp:
        root = Path(raw_tmp).resolve()
        target = root / "target.bin"
        workload = root / "workload.py"
        target.write_bytes(b"before00\n")
        workload.write_text(
            "from pathlib import Path\n"
            "import sys\n"
            "path = Path(sys.argv[1])\n"
            "with path.open('wb') as handle:\n"
            "    count = handle.write(b'updated1\\n')\n"
            "    handle.flush()\n"
            "assert count == 9\n",
            encoding="utf-8",
        )

        proc = subprocess.run(
            [str(binary), "observe", "--", sys.executable, str(workload), str(target)],
            capture_output=True,
            text=True,
            timeout=60,
            check=False,
        )
        if proc.returncode != 0:
            fail(f"observe-failed:{proc.returncode}:{proc.stderr[-2000:]}")

        try:
            observation = json.loads(proc.stdout)
        except json.JSONDecodeError as exc:
            fail(f"observation-json-invalid:{exc}")

        if observation.get("schema_version") != 2:
            fail(f"unexpected-schema:{observation.get('schema_version')!r}")
        if type(observation.get("complete")) is not bool:
            fail("global-complete-missing")
        if not isinstance(observation.get("warnings"), list):
            fail("global-warnings-missing")
        if not isinstance(observation.get("backend"), dict):
            fail("backend-metadata-missing")

        writes = [
            event
            for event in observation.get("events", [])
            if event.get("event_type") == "file_descriptor_access"
            and event.get("operation") == "write"
            and event.get("path") == str(target)
        ]
        if not writes:
            fail("target-write-event-not-observed")

        unexpected_result_fields = sorted(
            set().union(*(set(event) & ABSENT_WRITE_RESULT_FIELDS for event in writes))
        )
        if unexpected_result_fields:
            fail("alpha5-write-result-field-now-present:" + ",".join(unexpected_result_fields))

        typed_health_present = bool(set(observation) & TYPED_HEALTH_FIELDS)
        if typed_health_present:
            fail("alpha5-typed-health-field-now-present")

        report = {
            "decision": "P8_A3_G1_GAP_REPRODUCED_BOUNDED",
            "target": str(target),
            "schemaVersion": observation["schema_version"],
            "backend": observation["backend"],
            "globalComplete": observation["complete"],
            "warningCodes": [
                warning.get("code")
                for warning in observation["warnings"]
                if isinstance(warning, dict)
            ],
            "writeEventCount": len(writes),
            "writeEventKeySets": [sorted(event) for event in writes],
            "successfulWriteResultOrByteCountCarried": False,
            "typedCollectionHealthEnvelopePresent": False,
            "scope": [
                "published Alpha.5 raw schema v2 only",
                "successful target write observed",
                "no claim about production adoption",
                "no claim about independent custody",
                "no product/schema change",
            ],
        }
        args.output.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n",
            encoding="utf-8",
        )
        print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    main()
