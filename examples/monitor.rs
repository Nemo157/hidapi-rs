fn log_device(prefix: &str, info: &hidapi::DeviceInfo) {
    let manufacturer = info.manufacturer_string().unwrap_or_default();
    let product = info.product_string().unwrap_or_default();
    let vendor_id = info.vendor_id();
    let product_id = info.product_id();
    let path = info.path().to_string_lossy();

    println!("\
        \n{prefix}\
        \n    path: {path}\
        \n    manufacturer: {manufacturer}\
        \n    product: {product}\
        \n    id: {vendor_id:4x}:{product_id:4x}\
    ");
}

fn main() -> Result<(), hidapi::HidError> {
    let hidapi1 = hidapi::HidApi::new_without_enumerate()?;
    let mut hidapi2 = hidapi::HidApi::new_without_enumerate()?;

    // Create the monitor before enumerating existing devices so that we get
    // duplicates if a device is plugged in during enumeration instead of missing it
    let monitor = hidapi1.monitor()?;

    hidapi2.add_devices(0, 0)?;

    for device in hidapi2.device_list() {
        log_device("initial device", device);
    }

    for event in monitor {
        match event {
            hidapi::Event::Add(device) => log_device("new device", &device),
            event => println!("unknown monitor event: {event:?}"),
        }
    }

    Ok(())
}
