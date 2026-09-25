#!/usr/bin/env bash
set -euo pipefail

: "${EXECSURFACE_ACTION_BIN:?ExecSurface binary path is missing}"
: "${EXECSURFACE_COMMAND:?command input is required}"
: "${EXECSURFACE_BASELINE:?baseline input is required}"

evidence_dir="$(mktemp -d "${RUNNER_TEMP:-/tmp}/execsurface-evidence.XXXXXX")"
report_json="$evidence_dir/report.json"
summary_markdown="$evidence_dir/summary.md"

args=(check --baseline "$EXECSURFACE_BASELINE" --json-output "$report_json" --markdown-output "$summary_markdown")
if [[ -n "${EXECSURFACE_POLICY:-}" ]]; then
  args+=(--policy "$EXECSURFACE_POLICY")
fi

set +e
"$EXECSURFACE_ACTION_BIN" "${args[@]}" -- /bin/bash -lc "$EXECSURFACE_COMMAND"
status=$?
set -e

case "$status" in
  0) verdict="pass" ;;
  10) verdict="review" ;;
  20) verdict="block" ;;
  *)
    status=2
    verdict="error"
    "$EXECSURFACE_ACTION_BIN" render-error       --message "ExecSurface check did not produce a policy verdict; inspect workflow logs."       --json-output "$report_json"       --markdown-output "$summary_markdown"
    ;;
esac

if [[ -n "${GITHUB_STEP_SUMMARY:-}" ]]; then
  cat "$summary_markdown" >> "$GITHUB_STEP_SUMMARY"
fi

case "$verdict" in
  pass) echo "::notice title=ExecSurface::PASS" ;;
  review) echo "::warning title=ExecSurface::REVIEW" ;;
  block) echo "::error title=ExecSurface::BLOCK" ;;
  error) echo "::error title=ExecSurface::ERROR" ;;
esac

{
  echo "verdict=$verdict"
  echo "exit-code=$status"
  echo "report-json=$report_json"
  echo "summary-markdown=$summary_markdown"
  echo "evidence-dir=$evidence_dir"
  echo "sarif-status=not-generated:no-source-provenance"
} >> "$GITHUB_OUTPUT"
