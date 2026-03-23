# Build and Run

## Prerequisites
- Rust toolchain (stable or nightly)
- LLVM tools (for objcopy and ld.lld when needed)
- QEMU for x86_64 and aarch64 testing

Note: This document will be updated as soon as target specs and build scripts are finalized.

## Planned Build Steps (preview)
1. Build the kernel for the custom target in /targets/.
2. Build the UEFI bootloader for the matching UEFI target.
3. Package the bootloader and kernel into a UEFI image.
4. Run via QEMU using UEFI firmware.

Kernel path expected by the bootloader:
- /EFI/ProjectNN/kernel.elf

## Scripts
These helper scripts live in /tools:
- /tools/mk_efi_image.sh builds a UEFI directory with BOOTX64.EFI and kernel.elf
- /tools/run_qemu_x86_64.sh runs the image in QEMU with OVMF
- /tools/run_qemu_aarch64.sh runs the image in QEMU with AAVMF

## Example Flow (x86_64)
1. Build bootloader and kernel (paths are examples):
   - bootloader: target/x86_64-unknown-uefi/release/bootloader.efi
   - kernel: target/x86_64-unknown-kernel/release/kernel.elf
2. Package:
   - ARCH=x86_64 tools/mk_efi_image.sh <bootloader.efi> <kernel.elf>
3. Run:
   - tools/run_qemu_x86_64.sh

## Firmware Paths
You can override firmware locations using env vars:
- OVMF_CODE and OVMF_VARS (x86_64)
- AAVMF_CODE and AAVMF_VARS (aarch64)

## Planned Run Targets
- x86_64: QEMU with OVMF
- aarch64: QEMU with AAVMF

## Debugging (planned)
- Serial output on COM1
- QEMU GDB stub for early boot debugging
