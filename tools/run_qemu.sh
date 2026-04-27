#!/usr/bin/env bash
set -euo pipefail

BUILD_DIR="${1:-out/default}"

exec qemu-system-aarch64 \
  -machine virt,virtualization=on,gic-version=3 \
  -cpu cortex-a53 \
  -m 1536M \
  -nographic \
  -serial mon:stdio \
  -smp 8 \
  -kernel "${BUILD_DIR}/kernel.elf"
