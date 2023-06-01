#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::vec;
use alloc::format;
use log::info;
use uefi::{
    prelude::*,
    proto::media::{
        file::{File, FileAttribute, FileMode, FileType},
        fs::SimpleFileSystem,
    },
    CStr16, Result, table::{runtime::ResetType, boot::MemoryDescriptor},
};

#[entry]
fn main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    // initialized uefi services
    uefi_services::init(&mut st).expect("init to failed");

    // dump memory map
    dump_memmap(st.boot_services()).unwrap();

    // stall 10 second
    st.boot_services().stall(10_000_000);

    // shutdown
    st.runtime_services().reset(ResetType::Shutdown, Status::SUCCESS, None);
}

/// dump memory map
fn dump_memmap(bt: &BootServices) -> Result {
    // retrieve mem size
    let mem_size = bt.memory_map_size().map_size + 8 * core::mem::size_of::<MemoryDescriptor>();
    info!("mem size: {}", mem_size);

    // retrieve memory map
    let mut buf = vec![0; mem_size];
    let mem_map = bt.memory_map(&mut buf).unwrap();

    let sfs_handle = bt.get_handle_for_protocol::<SimpleFileSystem>()?;
    let mut sfsp = bt.open_protocol_exclusive::<SimpleFileSystem>(sfs_handle)?;

    let mut buf = [0; 16];
    let cstr = CStr16::from_str_with_buf("memmap", &mut buf).unwrap();
    let mut root_dir = sfsp.open_volume().unwrap();
    let mut file = match root_dir
        .open(cstr, FileMode::CreateReadWrite, FileAttribute::empty())
        .unwrap()
        .into_type()
        .unwrap()
    {
        FileType::Regular(file) => file,
        FileType::Dir(_) => {
            panic!();
        }
    };

    file.write("index, type, type(name), physical start, number of page, attribute\n".as_bytes()).unwrap();

    for d in mem_map.entries() {
        let line = format!(
            "{:x}, {:?}, {:08x}, {:x}, {:x}\n",
            d.ty.0,
            d.ty,
            d.phys_start,
            d.page_count,
            d.att.bits() & 0xfffff
        );
        file.write(line.as_bytes()).unwrap();
    }

    drop(file);
    Ok(())
}
