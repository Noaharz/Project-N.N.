#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${ROOT_DIR}/dist"
EFI_DIR="${OUT_DIR}/efi/EFI/ProjectNN"

BOOTLOADER_BIN="${1:-}"  # Path to bootloader EFI binary
KERNEL_ELF="${2:-}"       # Path to kernel ELF
ARCH="${ARCH:-x86_64}"

if [[ -z "${BOOTLOADER_BIN}" || -z "${KERNEL_ELF}" ]]; then
  echo "Usage: tools/mk_efi_image.sh <bootloader.efi> <kernel.elf>"
  exit 1
fi

rm -rf "${OUT_DIR}/efi"
mkdir -p "${EFI_DIR}"

if [[ "${ARCH}" == "aarch64" ]]; then
  cp "${BOOTLOADER_BIN}" "${EFI_DIR}/bootaa64.efi"
else
  cp "${BOOTLOADER_BIN}" "${EFI_DIR}/bootx64.efi"
fi
cp "${KERNEL_ELF}" "${EFI_DIR}/kernel.elf"

echo "EFI image prepared at ${OUT_DIR}/efi"
