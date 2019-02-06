#![feature(alloc)]  
#![no_std]
#![no_main]

extern crate uefi;
extern crate uefi_services;
#[macro_use]
extern crate log;
extern crate alloc;

use uefi::prelude::*;
use uefi::table::boot::MemoryType;
use crate::alloc::vec::Vec;

use x86_64::instructions::port::Port;

const EFI_PAGE_SIZE: u64 = 0x1000;

fn memory_map(bt: &BootServices) {
    // Get the estimated map size
    let map_size = bt.memory_map_size();

    // Build a buffer bigger enough to handle the memory map
    let mut buffer = Vec::with_capacity(map_size + 7);
    let off = (buffer.as_ptr() as usize) % 8;
    unsafe {
        buffer.set_len(map_size + off);
    }

    let (_, desc_iter) = bt
        .memory_map(&mut buffer[off..])
        .expect("Failed to retrieve UEFI memory map")
        .split().1;

    assert!(desc_iter.len() > 0, "Memory map is empty");

    // Print out a list of all the usable memory we see in the memory map.
    // Don't print out everything, the memory map is probably pretty big
    // (e.g. OVMF under QEMU returns a map with nearly 50 entries here).
    info!("efi: usable memory ranges ({} total)", desc_iter.len());
    for (_, descriptor) in desc_iter.enumerate() {
        match descriptor.ty {
            MemoryType::CONVENTIONAL => {
                let size = descriptor.page_count * EFI_PAGE_SIZE;
                let end_address = descriptor.phys_start + size;
                info!("> {:#x} - {:#x} ({} KiB)", descriptor.phys_start, end_address, size);
            },
            _ => {},
        }
    }
}

#[no_mangle]
pub extern "win64" fn uefi_start(_image_handle: uefi::Handle, system_table: SystemTable<Boot>) -> ! {
    // Initialize logging.
    uefi_services::init(&system_table)
        .unwrap()
        .expect("Failed to initialize");

    // Print out the UEFI revision number
    {
        let rev = system_table.uefi_revision();
        let (major, minor) = (rev.major(), rev.minor());
        
        info!("UEFI {}.{}", major, minor);
    }

    memory_map(system_table.boot_services());
    
    let mut port = Port::<u32>::new(0xf4);
    unsafe {
        port.write(0);
    }
    
    loop {};
}
