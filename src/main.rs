mod server;
mod steam;
mod devices;
mod utils;

pub fn pick_device() -> Option<evdev::Device> {
    let target_vendor_id = 0xb05u16;
    let target_product_id = 0x1abeu16;

    let devices = evdev::enumerate();
    for (_, device) in devices {
        let input_id = device.input_id();

        if input_id.vendor() == target_vendor_id && input_id.product() == target_product_id {
            return Some(device);
        }
    }

    None
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let device_opt = pick_device();
    match device_opt {
        Some(device) => {
            // Use the device
            println!("Device found: {}", device.name().unwrap_or("Unnamed device"));
        }
        None => {
            println!("No device found");
        }
    }

    /*let mut threads = Vec::new();

    threads.push(server::start_server());

    let _watcher = match steam::patch_steam() {
        Ok(watcher) => watcher,
        Err(_) => {
            eprintln!("Error setting up file watcher. Exiting...");
            std::process::exit(1);
        },
    };

    for thread in threads {
        thread.join().unwrap();
    }*/

    Ok(())
}