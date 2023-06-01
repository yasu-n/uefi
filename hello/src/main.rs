#![no_main]
#![no_std]

use core::fmt::Write;
use uefi::{prelude::*, table::runtime::ResetType};

#[entry]
fn main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    // initialized logger, memory and alloc
    uefi_services::init(&mut st).expect("Failed to init");

    // reset output
    st.stdout().reset(false).expect("Failed to stdout reset");

    // output "Hello, world!"
    writeln!(st.stdout(), "Hello, world!").unwrap();

    // wait 3 seconds
    st.boot_services().stall(3_000_000);

    // reset output
    st.stdout().reset(false).expect("Faild to stdoute reset");

    // shutdown
    st.runtime_services().reset(ResetType::Shutdown, Status::SUCCESS, None);
}
