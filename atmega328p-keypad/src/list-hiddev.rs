/*
    Prints out a list of HID devices
*/

use hidapi::HidApi;
use std::thread::sleep;
use std::time::Duration;

const TIME_S: Duration = Duration::new(2, 0);

fn main() {
    println!("Printing all available hid devices:\n");
    
    loop {
        match HidApi::new() {
            Ok(api) => {
                for device in api.device_list() {
                    println!(
                        "VID: {:04x}, PID: {:04x}, Serial: {}, Product name: {}, Interface: {}",
                        device.vendor_id(),
                        device.product_id(),
                        match device.serial_number() {
                            Some(s) => s,
                            _ => "<COULD NOT FETCH>",
                        },
                        match device.product_string() {
                            Some(s) => s,
                            _ => "<COULD NOT FETCH>",
                        },
                        device.interface_number()
                    );
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
        sleep(TIME_S);
    }
}
