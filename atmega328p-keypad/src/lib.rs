pub mod utilprint;

use rusb::*;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::vec::*;

pub const TIMEOUT: Duration = Duration::from_secs(60 * 5);
pub const WAIT: Duration = Duration::from_millis(500);
pub const VID: u16 = 0x16c0;
pub const PID: u16 = 0x05dc;
pub const MAXBUFSIZE: usize = 100;

//NOTE:SHOULD DO AN ENUM OF THIS TWO
struct CRWrite {
    //Control transfer read request
    request_type: u8,
    brequest: u8,
    wvalue: u16,
    windex: u16,
    buffer: Vec<u8>,
}

struct CRRead {
    //Control transfer read request
    request_type: u8,
    brequest: u8,
    wvalue: u16,
    windex: u16,
    buffer: [u8; MAXBUFSIZE],
}

impl CRRead {
    //Setup read control report
    fn setup(buffer: [u8; MAXBUFSIZE]) -> CRRead {
        CRRead {
            request_type: 0b10100001,
            brequest: 0x01,
            wvalue: 0x00,
            windex: 0x00,
            buffer,
        }
    }
}

impl CRWrite {
    //Setup write control report
    fn setup(buffer: Vec<u8>) -> CRWrite {
        CRWrite {
            request_type: 0b00100001,
            brequest: 0x09,
            wvalue: 0x00,
            windex: 0x00,
            buffer,
        }
    }
}

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

    let buffer = [0; MAXBUFSIZE];
    let mut rrep: CRRead = CRRead::setup(buffer);

    loop {
        //First send an input request to the device then read whatever the device sends back
        let bytes_read: usize = match dhandle.read_control(
            rrep.request_type,
            rrep.brequest,
            rrep.wvalue,
            rrep.windex,
            &mut rrep.buffer,
            TIMEOUT,
        ) {
            Ok(br) => br,
            Err(err) => {
                println!("No bytes read! Error: {}", err);
                exit(2);
            }
        };

        println!("Bytes read: {bytes_read}");

        let mut nice_buffer: Vec<u8> = Vec::from(rrep.buffer);
        nice_buffer.resize(bytes_read, 0);

        /* lossy will turn every valid utf8 character into the appropriate symbol, while the invalid
        ones will be shown as this symbol: � (it returns a smart pointer) */
        let buf_to_string = String::from_utf8_lossy(nice_buffer.as_slice());
        println!("From device: {}", buf_to_string);

        sleep(WAIT);
    }
}

//Write a byte (or more), get back the same byte (or more) in a loop
pub fn micro_control_write_read(context: &Context) {
    println!(
        "Writing to and reading from [VID: {}  PID: {}] in a loop...\n",
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

    //Read buffer to write from stdin:

    loop {
        let buffer_read: [u8; MAXBUFSIZE] = [0; MAXBUFSIZE];
        let mut rrep: CRRead = CRRead::setup(buffer_read);
        let mut wrep: CRWrite = CRWrite::setup(get_from_user());

        println!("$-------------------------------------$");

        //Cut last two char from vector got from user (return and terminating char)
        wrep.buffer
            .pop()
            .expect("Could not pop from write buffer containing user input");
        wrep.buffer
            .pop()
            .expect("Could not pop from write buffer containing user input");

        /* dbg!(&wrep.buffer); */

        let bytes_sent: usize = match dhandle.write_control(
            wrep.request_type,
            wrep.brequest,
            wrep.wvalue,
            wrep.windex,
            wrep.buffer.as_slice(),
            TIMEOUT,
        ) {
            Ok(bs) => bs,
            Err(err) => {
                println!("No bytes sent! Error: {}", err);
                exit(2);
            }
        };
        println!("Bytes sent: {}", bytes_sent);

        //First send an input request to the device then read whatever the device sends back
        let bytes_read: usize = match dhandle.read_control(
            rrep.request_type,
            rrep.brequest,
            rrep.wvalue,
            rrep.windex,
            &mut rrep.buffer,
            TIMEOUT,
        ) {
            Ok(br) => br,
            Err(err) => {
                println!("No bytes read! Error: {}", err);
                exit(2);
            }
        };

        /* dbg!(&rrep.buffer); */

        let mut nice_buffer: Vec<u8> = Vec::from(rrep.buffer);
        nice_buffer.resize(bytes_read, 0);

        /* lossy will turn every valid utf8 character into the appropriate symbol, while the invalid
        ones will be shown as this symbol: � (it returns a smart pointer) */
        let buf_to_string = String::from_utf8_lossy(nice_buffer.as_slice());
        println!("Received: {}\t({bytes_read})", buf_to_string);

        /* dbg!(buf_to_string); */
    }
}

fn get_from_user() -> Vec<u8> {
    println!("Write what you want to send to atmega:");
    let mut from_user: String = String::new();
    match std::io::stdin().read_line(&mut from_user) {
        Ok(bfu) => {
            println!("Bytes read from user: {}", bfu - 2);
            from_user.into_bytes()
        }
        Err(err) => {
            println!("Could not read input from user. Error: {}", err);
            exit(30);
        }
    }
}
