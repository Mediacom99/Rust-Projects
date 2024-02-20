/*
*
*
* Code layout:
*   1. Should have a function for each type of transfer both read and write!
*   2. They all need a handle and a control report, should probably think of some way to
*   3. hold all important current device info in one place
*   4. Once I have all of that in one module, I can make another module that deals with printing
*      and doing stuff, basically the read and write function will either return what they read or
*      ask for what they should write (maybe later the read and write funcs will accept the
*      aformentioned big struct with all the device info, a big function should be called that
*      creates this big thing give device VID and PID)
*
*      So smth like:
*
*      fn transfer_read/write(DEVICE_INFO, write buffer, [maybe here the transfer report in case
*      it's control]) -> read_buffer {
*
*           DO WHATEVER
*      }
*
* */

pub mod transfer;
pub mod utilprint;

use rusb::*;
use std::{process::exit, thread::sleep, time::Duration, vec::Vec};

pub const TIMEOUT: Duration = Duration::from_secs(60 * 5);
pub const WAIT: Duration = Duration::from_millis(500);
pub const VID: u16 = 0x16c0;
pub const PID: u16 = 0x05dc;
pub const MAXBUFSIZE: usize = 100;
pub const READ_BUFSIZE: usize = 1;

/// Report for control transfer (write)
struct CRWrite {
    //Control transfer read request
    request_type: u8,
    brequest: u8,
    wvalue: u16,
    windex: u16,
    buffer: Vec<u8>,
}

/// Report for control transfer (read)
struct CRRead {
    //Control transfer read request
    request_type: u8,
    brequest: u8,
    wvalue: u16,
    windex: u16,
    buffer: [u8; MAXBUFSIZE],
}

///NOTE:I SHOULD USE A VEC STRUCT INSTEAD OF A U8 ARRAY HERE, SO THAT I CAN HAVE A SINGLE STRUCT FOR
///A CONTROL REPORT TYPE
///ALSO I SHOULD ADD SETTER AND GETTER FUNCTIONS
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

/// CRWrite implementation
impl CRWrite {
    ///Setup write control report with default values
    fn setup(buffer: Vec<u8>) -> CRWrite {
        CRWrite {
            request_type: 0b00100001,
            brequest: 0x09,
            wvalue: 0x0000, //byte vuoto + byte (+)
            windex: 0x0000,
            buffer,
        }
    }

    // Set wvalue in control report
    fn _set_wvalue(&mut self, wvalue: u16) {
        self.wvalue = wvalue;
    }
}

///Initialize the libusb context
///NOTE: should keep context management completely private from bin and automatic inside lib
pub fn init_context() -> Context {
    Context::new().expect("Could not create a libusb context!")
}

///Read bytes from the device using a control transfer type
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

//
//
//
//
//
//
//
//
//
//
//

///Write a byte (or more), get back the same byte (or more) in a loop
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
            Ok(val) => val,
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

//
//
//
//
//
//
//
//
//
//

/// Get string from user using std::io::stdin and returning the string as a Vec<u8>
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

//
//
//
//
//
//
//
//
//
//
//
//

///Write a control report that sends in the control transfer parameter wvalue the char that will be sent using interrupt
// Read the interrupt
pub fn micro_interrupt_read(context: &Context) {
    println!(
        "Reading input from [VID: {}  PID: {}] using an interrupt transfer...\n",
        VID, PID
    );
    //Open and get a handle to a device with a certain VID and PID
    let mut dhandle = match context.open_device_with_vid_pid(VID, PID) {
        Some(res) => res,
        None => {
            println!("Could not open device");
            exit(1);
        }
    };

    //Initialize control transfer to give the micro the byte to send with the interrupt

    let buffer: Vec<u8> = Vec::new(); //Sending empty buffer because the char is provided in wvalue
    let mut wrep: CRWrite = CRWrite::setup(buffer);
    wrep._set_wvalue(0x2b00); //Hex for ascii '+'

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
    println!("Control report buffer bytes sent: {}", bytes_sent);
    println!("Control request sent successfully");

    //NOTE:EVERYTHING THAT USES OTHER TRANSFERS OTHER THAN CONTROL MUST GO AFTER ANY CONTROL STUFF
    //Claim the correct interface that cointains the endpoint configured for this type of transfer
    //The interface is automatically release when the device handle goes out of scope
    match dhandle.claim_interface(0) {
        Ok(()) => {
            println!("Successfully claimed interface 0");
        }
        Err(err) => {
            println!("Could not claim interface! Error: {}", err);
        }
    }

    //Read from micro using interrupt transfer
    let mut buf: [u8; READ_BUFSIZE] = [0];

    loop {
        let bytes_read: usize = match dhandle.read_interrupt(0x81, &mut buf, TIMEOUT) {
            Ok(br) => br,
            Err(err) => {
                println!("Could not read bytes using interrupt. Error: {}", err);
                continue;
            }
        };
        println!(
            "Bytes read from interrupt: {bytes_read} Input read: {}",
            String::from_utf8_lossy(&buf)
        );
    }
}

//
//
//
//
//
//
//
//
//
//
//
