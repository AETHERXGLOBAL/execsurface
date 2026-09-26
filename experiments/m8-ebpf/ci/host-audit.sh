#!/usr/bin/env bash
set -euxo pipefail

uname -a
id
rustc --version --verbose
cargo --version
clang --version
test -r /sys/kernel/btf/vmlinux
stat /sys/kernel/btf/vmlinux
sha256sum /sys/kernel/btf/vmlinux
cat /proc/sys/kernel/unprivileged_bpf_disabled || true
ulimit -l
mount | grep -E 'tracefs|debugfs' || true
test -d /sys/kernel/tracing/events || test -d /sys/kernel/debug/tracing/events
