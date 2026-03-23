#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OVMF_CODE="${OVMF_CODE:-/usr/share/OVMF/OVMF_CODE.fd}"
OVMF_VARS="${OVMF_VARS:-/usr/share/OVMF/OVMF_VARS.fd}"
EFI_DIR="${ROOT_DIR}/dist/efi"

if [[ ! -f "${OVMF_CODE}" ]]; then
  echo "OVMF_CODE not found: ${OVMF_CODE}"
  exit 1
fi
if [[ ! -f "${OVMF_VARS}" ]]; then
  echo "OVMF_VARS not found: ${OVMF_VARS}"
  exit 1
fi

qemu-system-x86_64 \
  -machine q35 \
  -m 512M \
  -drive if=pflash,format=raw,readonly=on,file="${OVMF_CODE}" \
  -drive if=pflash,format=raw,file="${OVMF_VARS}" \
  -drive format=raw,file=fat:rw:"${EFI_DIR}" \
  -serial stdio \
  -no-reboot
