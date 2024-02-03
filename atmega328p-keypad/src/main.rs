/*
*   1. Create Hidapi context
*   2. reset devices
*   3. add device with VID and PID
*   4. open that device
*   5. Do smth using HidDevice (read write)
* */
use rusb::*;
mod rusbutils;
use std::process::exit;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(60);

const VID: u16 = 0x16c0;
const PID: u16 = 0x05dc;
const MAXBUFSIZE: usize = 1024;
//const VID: u16 = 3599;
//const PID: u16 = 3;
//const PAYLOAD_BYTES: usize = 128;
//const REPORT_BYTES: usize = 8;

fn main() {
    //Create new libusb context
    let context = Context::new().expect("Could not create a libusb context!");

    //list_usb_devices(&context);

    //Open and get a handle to a device with a certain VID and PID
    let dhandle = match context.open_device_with_vid_pid(VID, PID) {
        Some(res) => res,
        None => {
            println!("Could not open device");
            exit(1);
        }
    };

    //Hid request protocol
    let request_type: u8 = 0xa1;
    let request: u8 = 0x01;
    let value: u16 = 0x00;
    let index: u16 = 0x00;
    let mut buffer: [u8; MAXBUFSIZE] = [0; MAXBUFSIZE];

    loop {
        //First send an input request to the device then read whatever the device sends back
        let bytes_read: usize =
            match dhandle.read_control(request_type, request, value, index, &mut buffer, TIMEOUT) {
                Ok(br) => br,
                Err(err) => {
                    println!("No bytes read! Error: {}", err);
                    exit(2);
                }
            };
        println!("Bytes read: {bytes_read}");
        //lossy will turn every valid utf8 character into the appropriate symbol, while the invalid
        //ones will be shown as this symbol: � (it returns a smart pointer)
        let from_device = String::from_utf8_lossy(&buffer);
        println!("From device: {}", from_device);
    }
}
