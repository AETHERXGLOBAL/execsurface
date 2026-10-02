#!/usr/bin/env bash
set -euo pipefail

binary=${1:-execsurface}
requested_binary=$binary
if [[ "$binary" == */* ]]; then
  if ! binary=$(realpath -e -- "$requested_binary"); then
    printf 'ExecSurface binary not found: %s\n' "$requested_binary" >&2
    exit 1
  fi
else
  if ! binary=$(command -v -- "$requested_binary"); then
    printf 'ExecSurface binary not found on PATH: %s\n' "$requested_binary" >&2
    exit 1
  fi
fi

temp_parent=${TMPDIR:-/tmp}
temp_parent=$(cd -- "$temp_parent" && pwd -P)
workdir=$(mktemp -d "$temp_parent/execsurface-hello-drift.XXXXXX")
trap 'rm -rf -- "$workdir"' EXIT
cd "$workdir"
export LC_ALL=C NO_COLOR=1 TMPDIR="$temp_parent"

if ! learn_output=$("$binary" learn -- /bin/bash -lc 'true' 2>&1); then
  printf '%s\n' "$learn_output" >&2
  printf 'Failed to learn the deterministic baseline.\n' >&2
  exit 1
fi
printf '%s\n' "$learn_output"

if ! pass_output=$("$binary" check -- /bin/bash -lc 'true' 2>&1); then
  printf '%s\n' "$pass_output" >&2
  printf 'The unchanged command did not exit successfully.\n' >&2
  exit 1
fi
printf '%s\n' "$pass_output"
if ! grep -Fq 'ExecSurface: PASS' <<<"$pass_output"; then
  printf 'Expected the unchanged command to report PASS.\n' >&2
  exit 1
fi

if review_output=$("$binary" check -- /bin/bash -lc 'true; /bin/echo controlled-drift >/dev/null' 2>&1); then
  review_status=0
else
  review_status=$?
fi
printf '%s\n' "$review_output"
if [[ $review_status -ne 10 ]] || ! grep -Fq 'ExecSurface: REVIEW' <<<"$review_output"; then
  printf 'Expected controlled drift to report REVIEW and exit with status 10; got %s.\n' "$review_status" >&2
  exit 1
fi

printf 'PASS and REVIEW assertions succeeded.\n'
