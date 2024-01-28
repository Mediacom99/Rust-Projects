/*
*   1. Create Hidapi context
*   2. reset devices
*   3. add device with VID and PID
*   4. open that device
*   5. Do smth using HidDevice (read write)
* */

use hidapi::*;
use listhiddev::TIME_S;
use std::thread::sleep;

// Local Modules
mod listhiddev;

// const VID: u16 = 5824;
// const PID: u16 = 1503;
const VID: u16 = 3599;
const PID: u16 = 3;
const PAYLOAD_BYTES: usize = 128;
const REPORT_BYTES: usize = 8;

fn print_byte_stream_hex(bytes_read: usize, buf: &[u8]) {
    for bytes in 0..bytes_read {
        print!("0x{:x} ", buf[bytes]);
        if bytes % 2 == 0 {
            println!(" ");
        }
    }
    println!(" ");
}

fn main() {
    let mut buf: [u8; MAX_REPORT_DESCRIPTOR_SIZE] = [0; MAX_REPORT_DESCRIPTOR_SIZE];

    println!("Initializing HidApi context...");
    //Initialize an HidApi contex
    let mut api = HidApi::new().expect("Failed to initialize HidApi context");

    listhiddev::list_hid_devices(&api);

    println!("Resetting hid devices...");
    //Reset devices
    api.reset_devices()
        .expect("Could not reset the hid devices");

    println!("Adding ATMega328p device...");
    //Add device with certain VID and PID
    api.add_devices(VID, PID).expect("Could not add device!");

    println!("Opening ATMega328p device...");
    //Open HID device
    let atmega: HidDevice = api.open(VID, PID).expect("Could not open the device!");

    println!("Getting report descriptor...");
    //Get report descriptor
    let bytes_read = atmega
        .get_report_descriptor(&mut buf)
        .expect("Could not get report descriptor!");

    println!("Bytes read: {bytes_read}");
    println!("Report descriptor:");
    print_byte_stream_hex(bytes_read, &buf);

    println!("\n\nStarting reading loop...\n\n");

    //Write an output request to the device
    let mut report: [u8; REPORT_BYTES] = [0; REPORT_BYTES];

    report[0] = 0xa1;
    report[1] = 0x01;
    report[2] = 0x03;
    report[3] = 0x00;
    report[4] = 0x00;
    report[5] = 0x00;
    report[6] = 0x00;
    report[7] = 0x00;

    let written_bytes = atmega
        .write(&report)
        .expect("Could not write output report to device");
    println!("Written bytes: {written_bytes}");

    //Set device in non-blocking mode (read waits for input from device)
    atmega
        .set_blocking_mode(false)
        .expect("Could NOT set the device in NON-blocking mode!");

    //Not using numbered reports/Trying to get some data from the micro
    let mut payload: [u8; PAYLOAD_BYTES] = [0; PAYLOAD_BYTES]; //8 bytes payload
    loop {
        let succ_read = atmega
            .get_feature_report(&mut payload)
            .expect("Could not read report from device");
        println!("Bytes read: {succ_read}");
        print_byte_stream_hex(PAYLOAD_BYTES, &payload);
        sleep(TIME_S);
        /*
        let payload: Vec<u8> = payload.to_vec();
        println!(
            "Payload:\n{}",
            String::from_utf8(payload).expect("Could not convert payload into UTF-8 string!")
        );
        */
    }
}
