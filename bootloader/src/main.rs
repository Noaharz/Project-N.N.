#![no_std]
#![no_main]

use boot_protocol::{FramebufferFormat, FramebufferInfo, Handoff, MemoryMapInfo, HANDOFF_MAGIC};
use core::{mem::size_of, panic::PanicInfo, ptr};
use uefi::prelude::*;
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat};
use uefi::proto::media::file::{File, FileAttribute, FileInfo, FileMode, RegularFile};
use uefi::table::boot::{AllocateType, MemoryType};

const KERNEL_PATH: &str = "\\EFI\\ProjectNN\\kernel.elf";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

entry!(efi_main);

fn efi_main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let _ = system_table.stdout().reset(false);
    let _ = system_table
        .stdout()
        .write_str("Project N.N. UEFI bootloader\r\n");

    let entry_addr = match load_kernel(image_handle, &mut system_table) {
        Ok(addr) => addr,
        Err(status) => {
            let _ = system_table.stdout().write_str("Kernel load failed\r\n");
            return status;
        }
    };

    let bs = system_table.boot_services();
    let framebuffer = get_framebuffer_info(bs).unwrap_or(FramebufferInfo {
        base: core::ptr::null_mut(),
        size: 0,
        width: 0,
        height: 0,
        stride: 0,
        format: FramebufferFormat::Unknown,
    });

    let (_mmap_buf, mmap_info, map_key) = match build_memory_map(bs) {
        Ok(v) => v,
        Err(status) => {
            let _ = system_table.stdout().write_str("Memory map failed\r\n");
            return status;
        }
    };

    let handoff = Handoff {
        magic: HANDOFF_MAGIC,
        memory_map: mmap_info,
        framebuffer,
    };

    let _rt = match exit_boot_services_with_retry(system_table, image_handle, map_key) {
        Ok(rt) => rt,
        Err(status) => return status,
    };

    let entry: extern "C" fn(*const Handoff) -> ! = unsafe { core::mem::transmute(entry_addr) };
    entry(&handoff as *const Handoff);
}

fn load_kernel(
    image_handle: Handle,
    system_table: &mut SystemTable<Boot>,
) -> Result<u64, Status> {
    let bs = system_table.boot_services();
    let fs = bs.get_image_file_system(image_handle)?;
    let mut root = fs.open_volume()?;

    let file = root.open(KERNEL_PATH, FileMode::Read, FileAttribute::empty())?;
    let mut kernel = match file.into_type()? {
        File::Regular(file) => file,
        _ => return Err(Status::LOAD_ERROR),
    };

    let kernel_size = file_size(&mut kernel, bs)?;
    let kernel_buf = unsafe { bs.allocate_pool(MemoryType::LOADER_DATA, kernel_size)? };
    let kernel_slice = unsafe { core::slice::from_raw_parts_mut(kernel_buf, kernel_size) };
    kernel.read(kernel_slice)?;

    let entry_addr = load_elf64(kernel_slice, bs)?;
    Ok(entry_addr)
}

fn file_size(file: &mut RegularFile, bs: &BootServices) -> Result<usize, Status> {
    let info_size = size_of::<FileInfo>() + 512;
    let info_buf = unsafe { bs.allocate_pool(MemoryType::LOADER_DATA, info_size)? };
    let info_slice = unsafe { core::slice::from_raw_parts_mut(info_buf, info_size) };

    let info = file.get_info::<FileInfo>(info_slice)?;
    Ok(info.file_size() as usize)
}

fn build_memory_map(
    bs: &BootServices,
) -> Result<(*mut u8, MemoryMapInfo, uefi::table::boot::MemoryMapKey), Status> {
    let map_size = bs.memory_map_size();
    let buf_size = map_size.map_size + 8 * map_size.entry_size;
    let mmap_buf = unsafe { bs.allocate_pool(MemoryType::LOADER_DATA, buf_size)? };
    let mmap_slice = unsafe { core::slice::from_raw_parts_mut(mmap_buf, buf_size) };

    let mmap = bs.memory_map(mmap_slice)?;

    let info = MemoryMapInfo {
        buffer: mmap_slice.as_ptr(),
        size: mmap.map_size,
        descriptor_size: mmap.entry_size,
        descriptor_version: mmap.version,
    };

    Ok((mmap_buf, info, mmap.key))
}

fn exit_boot_services_with_retry(
    system_table: SystemTable<Boot>,
    image_handle: Handle,
    map_key: uefi::table::boot::MemoryMapKey,
) -> Result<uefi::table::SystemTable<uefi::table::Runtime>, Status> {
    match unsafe { system_table.exit_boot_services(image_handle, map_key) } {
        Ok(rt) => Ok(rt),
        Err((mut st, _status)) => {
            let _ = st.stdout().write_str("Retry exit boot services\r\n");
            let bs = st.boot_services();
            let (_buf, _info, new_key) = build_memory_map(bs)?;
            match unsafe { st.exit_boot_services(image_handle, new_key) } {
                Ok(rt) => Ok(rt),
                Err((_st, status)) => Err(status),
            }
        }
    }
}

fn get_framebuffer_info(bs: &BootServices) -> Option<FramebufferInfo> {
    let gop_ptr = bs.locate_protocol::<GraphicsOutput>().ok()?;
    let gop = unsafe { &mut *gop_ptr.get() };
    let mode = gop.current_mode_info();
    let resolution = mode.resolution();
    let stride = mode.stride() as u32;
    let format = match mode.pixel_format() {
        PixelFormat::Rgb => FramebufferFormat::Rgb,
        PixelFormat::Bgr => FramebufferFormat::Bgr,
        _ => FramebufferFormat::Unknown,
    };
    let fb = gop.frame_buffer();
    Some(FramebufferInfo {
        base: fb.as_mut_ptr(),
        size: fb.size(),
        width: resolution.0,
        height: resolution.1,
        stride,
        format,
    })
}

fn load_elf64(image: &[u8], bs: &BootServices) -> Result<u64, Status> {
    if image.len() < size_of::<Elf64Ehdr>() {
        return Err(Status::LOAD_ERROR);
    }

    let ehdr = unsafe { &*(image.as_ptr() as *const Elf64Ehdr) };
    if &ehdr.e_ident[0..4] != b"\x7FELF" {
        return Err(Status::LOAD_ERROR);
    }
    if ehdr.e_ident[4] != 2 {
        return Err(Status::LOAD_ERROR);
    }
    if ehdr.e_phoff == 0 || ehdr.e_phentsize as usize != size_of::<Elf64Phdr>() {
        return Err(Status::LOAD_ERROR);
    }

    let phoff = ehdr.e_phoff as usize;
    let phnum = ehdr.e_phnum as usize;
    let phdrs_end = phoff + phnum * size_of::<Elf64Phdr>();
    if phdrs_end > image.len() {
        return Err(Status::LOAD_ERROR);
    }

    for i in 0..phnum {
        let base = phoff + i * size_of::<Elf64Phdr>();
        let phdr = unsafe { &*(image[base..].as_ptr() as *const Elf64Phdr) };
        if phdr.p_type != PT_LOAD {
            continue;
        }

        let file_end = phdr.p_offset as usize + phdr.p_filesz as usize;
        if file_end > image.len() {
            return Err(Status::LOAD_ERROR);
        }

        let dest = phdr.p_paddr as usize;
        let mem_size = phdr.p_memsz as usize;
        let file_size = phdr.p_filesz as usize;
        let pages = (mem_size + 0xFFF) / 0x1000;

        unsafe {
            bs.allocate_pages(
                AllocateType::Address(dest),
                MemoryType::LOADER_DATA,
                pages,
            )?;
            ptr::copy_nonoverlapping(
                image.as_ptr().add(phdr.p_offset as usize),
                dest as *mut u8,
                file_size,
            );
            if mem_size > file_size {
                ptr::write_bytes((dest + file_size) as *mut u8, 0, mem_size - file_size);
            }
        }
    }

    Ok(ehdr.e_entry)
}

const PT_LOAD: u32 = 1;

#[repr(C)]
struct Elf64Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
struct Elf64Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}
