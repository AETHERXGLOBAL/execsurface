#!/usr/bin/env bash
set -euo pipefail

scenario="${1:?scenario required}"
root="${P6_WORK_ROOT:-/tmp/execsurface-p6}"
mkdir -p "$root"

# Deterministic marker emitted by every scenario. It is workload evidence only,
# not a product verdict.
printf 'p6_workload_version=1\nscenario=%s\n' "$scenario"

case "$scenario" in
  S0_CONTROL)
    /usr/bin/printf 'p6-control\n' > /dev/null
    ;;

  S1_CHILD_PROCESS_EXPANSION)
    # Fixed executable and fixed argv; intended to create one explicit process
    # expansion relative to S0 without reading user or repository secrets.
    /bin/bash -c '/usr/bin/printf "p6-child-expansion\\n" >/dev/null'
    ;;

  S2_NETWORK_DESTINATION_EXPANSION)
    # IANA-reserved documentation endpoint. The hostname is the frozen logical
    # destination; resolved IPs are recorded at runtime because DNS may vary.
    getent ahosts example.com | awk '{print $1}' | sort -u > "$root/s2-resolved-ips.txt" || true
    curl --fail --silent --show-error --location --max-time 15 \
      --output /dev/null https://example.com/
    ;;

  S3_FILE_WRITE_EXPANSION)
    # Sandboxed deterministic write; no sensitive host path is used.
    printf 'execsurface-p6-s3\n' > "$root/s3.txt"
    test "$(cat "$root/s3.txt")" = 'execsurface-p6-s3'
    ;;

  *)
    echo "unknown P6 scenario: $scenario" >&2
    exit 64
    ;;
esac
