# Project N.N. Kernel

Rust-based kernel with a custom UEFI bootloader, built for multiple CPU architectures.

## Status
- Scope: kernel-only (no user modules yet)
- Boot: custom UEFI loader
- Architectures: x86_64 and aarch64 (others later)

## Quick Start (outline)
- See /docs/BUILD.md for prerequisites and build steps
- See /docs/ARCHITECTURE.md for the design overview

## Repository Layout
- /bootloader/ custom UEFI bootloader
- /kernel/ no_std kernel
- /targets/ custom Rust target specs
- /docs/ documentation
- /tools/ helper scripts (future)

## v1 Definition (planned)
- UEFI bootloader loads kernel and passes memory map
- Kernel enters 64-bit mode and initializes basic arch setup
- Console output (VGA on x86_64, serial for debug)
- IDT/GDT on x86_64, exception vectors on aarch64
- Frame allocator for physical memory

## Principles
- Minimal core: only hardware control and essential services in kernel
- Modules later: everything else moves to user space
- Safety first: Rust no_std with explicit unsafe boundaries

## License
MIT. See /LICENSE.
