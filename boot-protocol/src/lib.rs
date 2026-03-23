#![no_std]

pub const HANDOFF_MAGIC: u64 = 0x4E_4E_4B_45_52_4E_45_4C; // "NNKERNEL"

#[repr(C)]
pub struct Handoff {
    pub magic: u64,
    pub memory_map: MemoryMapInfo,
}

#[repr(C)]
pub struct MemoryMapInfo {
    pub buffer: *const u8,
    pub size: usize,
    pub descriptor_size: usize,
    pub descriptor_version: u32,
}
