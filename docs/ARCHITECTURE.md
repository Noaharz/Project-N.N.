# Architecture

## Overview
The kernel follows an inverted pyramid design.

- Hardware at the top: CPU, RAM, devices
- Kernel in the middle: minimal, trusted core
- Modules at the base: higher-level services in user mode (future)

This repo currently targets kernel-only v1 to establish a solid base before user-mode modules.

## Boot Flow (UEFI)
1. UEFI firmware loads the custom bootloader.
2. Bootloader initializes UEFI services and reads the memory map.
3. Bootloader loads the kernel image into memory.
4. Bootloader exits UEFI boot services.
5. Bootloader jumps to kernel entry with a handoff struct.

## Kernel Responsibilities (v1)
- Basic arch initialization
- Interrupt and exception handling setup
- Early console output
- Physical memory management bootstrap

## Boot Handoff (v1)
- Memory map (UEFI descriptors)
- Optional GOP framebuffer info (base, size, format)

## Multi-Architecture Strategy
- Architecture-specific code is isolated under /kernel/src/arch/
- Shared kernel services live in /kernel/src/
- Bootloader supports UEFI for x86_64 and aarch64

## Safety and Unsafe Code
- Unsafe code is allowed only where required for hardware control
- Each unsafe block should be small and documented
- No implicit global state without explicit initialization
