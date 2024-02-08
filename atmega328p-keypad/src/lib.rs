pub mod utilprint;

use rusb::*;
use std::process::exit;
use std::time::Duration;

pub const TIMEOUT: Duration = Duration::from_secs(60);
pub const VID: u16 = 0x16c0;
pub const PID: u16 = 0x05dc;
pub const MAXBUFSIZE: usize = 1024;

//Initialize the libusb context
//NOTE: should keep context management completely private from bin and automatic inside lib
pub fn init_context() -> Context {
    Context::new().expect("Could not create a libusb context!")
}

//Read bytes from the device using a control transfer type
pub fn micro_control_read(context: &Context) {
    println!(
        "Reading input from [VID: {}  PID: {}] using a control transfer...\n",
        VID, PID
    );
    //Open and get a handle to a device with a certain VID and PID
    let dhandle = match context.open_device_with_vid_pid(VID, PID) {
        Some(res) => res,
        None => {
            println!("Could not open device");
            exit(1);
        }
    };

    //Control transfer read request
    let request_type: u8 = 0xa1;
    let request: u8 = 0x01;
    let value: u16 = 0x00;
    let index: u16 = 0x00;
    let mut buffer: [u8; MAXBUFSIZE] = [0x00; MAXBUFSIZE];

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

        /* lossy will turn every valid utf8 character into the appropriate symbol, while the invalid
        ones will be shown as this symbol: � (it returns a smart pointer) */
        let buf_to_string = String::from_utf8_lossy(&buffer);
        println!("From device: {}", buf_to_string);
    }
}
