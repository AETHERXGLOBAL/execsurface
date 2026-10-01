#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
DIR="$ROOT/experiments/m10/bpf-lsm-file"
OUT="$DIR/out"
mkdir -p "$OUT"

{
  echo "date_utc=$(date -u +%FT%TZ)"
  echo "uname=$(uname -a)"
  echo "kernel=$(uname -r)"
  echo "uid=$(id -u)"
  echo "btf_vmlinux=$([[ -r /sys/kernel/btf/vmlinux ]] && echo yes || echo no)"
  if [[ -r /sys/kernel/security/lsm ]]; then
    echo "lsm_list=$(cat /sys/kernel/security/lsm)"
  else
    echo "lsm_list=UNREADABLE"
  fi
} | tee "$OUT/host-audit.txt"

if [[ ! -r /sys/kernel/btf/vmlinux ]]; then
  echo "classification=BPF_LSM_FILE_HOST_UNAVAILABLE reason=no_vmlinux_btf" | tee "$OUT/result.txt"
  exit 0
fi

LSM_LIST="$(cat /sys/kernel/security/lsm 2>/dev/null || true)"
if [[ ",$LSM_LIST," != *,bpf,* ]]; then
  echo "classification=BPF_LSM_FILE_HOST_UNAVAILABLE reason=bpf_not_in_active_lsm_list lsm_list=${LSM_LIST:-UNREADABLE}" | tee "$OUT/result.txt"
  exit 0
fi

bpftool btf dump file /sys/kernel/btf/vmlinux format c > "$DIR/vmlinux.h"
clang -g -O2 -target bpf -D__TARGET_ARCH_x86 \
  -I"$DIR" -I/usr/include/$(uname -m)-linux-gnu \
  -c "$DIR/bpf_lsm_file.bpf.c" -o "$DIR/bpf_lsm_file.bpf.o"
bpftool gen skeleton "$DIR/bpf_lsm_file.bpf.o" > "$DIR/bpf_lsm_file.skel.h"
gcc -O2 -g -Wall -Wextra -Werror \
  -I"$DIR" "$DIR/loader.c" -o "$DIR/m10_bpf_lsm_file" \
  -lbpf -lelf -lz

set +e
sudo "$DIR/m10_bpf_lsm_file" > "$OUT/loader.txt" 2>&1
RC=$?
set -e
cat "$OUT/loader.txt"

echo "loader_exit=$RC" >> "$OUT/result.txt"
if [[ $RC -eq 0 ]]; then
  CLASS="$(grep '^classification_candidate=' "$OUT/loader.txt" | tail -n1 | cut -d= -f2-)"
  echo "classification=${CLASS:-BPF_LSM_FILE_INFRA_FAILURE}" >> "$OUT/result.txt"
elif [[ $RC -eq 42 ]]; then
  echo "classification=BPF_LSM_FILE_HOST_UNAVAILABLE reason=loader_attach_or_load" >> "$OUT/result.txt"
else
  echo "classification=BPF_LSM_FILE_INFRA_FAILURE reason=loader_exit_${RC}" >> "$OUT/result.txt"
fi
cat "$OUT/result.txt"

# A scientific classification is data, not CI pass/fail. Preserve artifacts either way.
exit 0
