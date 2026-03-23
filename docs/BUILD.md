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

## Planned Run Targets
- x86_64: QEMU with OVMF
- aarch64: QEMU with AAVMF

## Debugging (planned)
- Serial output on COM1
- QEMU GDB stub for early boot debugging
