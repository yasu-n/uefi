#![no_main]
#![no_std]

use log::info;
use uefi::{
    prelude::*,
    proto::{
        device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly},
        loaded_image::LoadedImage,
    },
    table::{boot::SearchType, runtime::ResetType},
    Identify, Result,
};

#[entry]
fn main(handle: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut st).expect("Failed to init");
    let boot_services = st.boot_services();

    print_image_path(boot_services).expect("Failed to print image path");

    boot_services.stall(10_000_000);

    st.runtime_services()
        .reset(ResetType::Shutdown, Status::SUCCESS, None);
}

fn print_image_path(boot_services: &BootServices) -> Result {
    let loaded_image =
        boot_services.open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())?;

    let device_path_to_text_handle = *boot_services
        .locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID))?
        .first()
        .expect("DevicePathToText is missing");

    let device_path_to_text =
        boot_services.open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle)?;

    let image_device_path = loaded_image.file_path().expect("File path is not set");
    let image_device_path_text = device_path_to_text
        .convert_device_path_to_text(
            boot_services,
            image_device_path,
            DisplayOnly(true),
            AllowShortcuts(false),
        )
        .expect("convert_device_path_to_text failed");

    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}
