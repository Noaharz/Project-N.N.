# Project N.N. Kernel

Rust-based kernel + custom UEFI bootloader.

## Goals (v1)
- Custom UEFI bootloader (x86_64 + aarch64)
- Kernel-only (no user modules yet)
- Basic console output (VGA for x86_64, serial for debug)
- IDT/GDT (x86_64) + interrupts
- Frame allocator

## Layout
- `bootloader/` - custom UEFI bootloader
- `kernel/` - no_std kernel
- `targets/` - custom Rust target specs

## Notes
This repo is intentionally minimal at the start. We'll grow the system in phases.
