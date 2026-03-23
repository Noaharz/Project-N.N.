#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
AAVMF_CODE="${AAVMF_CODE:-/usr/share/AAVMF/AAVMF_CODE.fd}"
AAVMF_VARS="${AAVMF_VARS:-/usr/share/AAVMF/AAVMF_VARS.fd}"
EFI_DIR="${ROOT_DIR}/dist/efi"

if [[ ! -f "${AAVMF_CODE}" ]]; then
  echo "AAVMF_CODE not found: ${AAVMF_CODE}"
  exit 1
fi
if [[ ! -f "${AAVMF_VARS}" ]]; then
  echo "AAVMF_VARS not found: ${AAVMF_VARS}"
  exit 1
fi

qemu-system-aarch64 \
  -machine virt \
  -cpu cortex-a57 \
  -m 512M \
  -drive if=pflash,format=raw,readonly=on,file="${AAVMF_CODE}" \
  -drive if=pflash,format=raw,file="${AAVMF_VARS}" \
  -drive format=raw,file=fat:rw:"${EFI_DIR}" \
  -serial stdio \
  -no-reboot
