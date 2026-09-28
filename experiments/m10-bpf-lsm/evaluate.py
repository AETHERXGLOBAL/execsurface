#!/usr/bin/env python3
import json
import sys
from pathlib import Path

report_path = Path(sys.argv[1])
out_path = Path(sys.argv[2])
report = json.loads(report_path.read_text())

result = {
    "protocol": "M10_3_BPF_LSM_FILE_001",
    "classification": "M10_3_EVIDENCE_INCOMPLETE",
    "reason": None,
    "producer_drops": report.get("producer_drops"),
    "decode_errors": report.get("decode_errors"),
    "event_count": len(report.get("events", [])),
}

try:
    truth = json.loads(report["target_stdout"].strip())
except Exception as exc:
    result["reason"] = f"missing_or_invalid_target_ground_truth: {exc}"
    out_path.write_text(json.dumps(result, indent=2) + "\n")
    sys.exit(0)

result["truth"] = truth

if not report.get("attachment_available"):
    result["reason"] = "attachment_not_available"
elif not report.get("session_registered"):
    result["reason"] = "session_not_registered"
elif report.get("target_exit_code") != 0 or report.get("target_signal") is not None:
    result["reason"] = "target_not_clean"
elif report.get("producer_drops") != 0:
    result["reason"] = "producer_loss"
elif report.get("decode_errors") != 0:
    result["reason"] = "decode_error"
else:
    matches = [
        e for e in report.get("events", [])
        if int(e.get("dev", -1)) == int(truth["dev"])
        and int(e.get("inode", -1)) == int(truth["ino"])
    ]
    result["matching_events"] = len(matches)
    if len(matches) == 1:
        result["classification"] = "M10_3_OBJECT_IDENTITY_MATCH"
        result["reason"] = "exactly_one_session_event_matches_fstat_dev_inode"
    elif len(matches) == 0:
        result["classification"] = "M10_3_OBJECT_IDENTITY_MISMATCH"
        result["reason"] = "no_session_event_matches_fstat_dev_inode"
    else:
        result["reason"] = "multiple_matching_events_not_unique"

out_path.write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result, indent=2))
