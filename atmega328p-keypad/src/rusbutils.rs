use rusb::*;
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

struct DeviceDescriptor {
    vid: u16,        //Vendor id
    pid: u16,        //Product id
    mps: u8,         //max packet size
    ncd: u8,         //num of config descriptors
    man_id: String,  //Manufacturer ID
    prod_id: String, //Product ID
}

impl DeviceDescriptor {
    fn print(&self) {
        println!("Manufacturer ID: {}", self.man_id);
        println!("Product ID: {}", self.prod_id);
        println!("VID: {:x}", self.vid);
        println!("PID: {:x}", self.pid);
        println!("Max packet size: {}B", self.mps);
        println!("Number of config descriptors: {}", self.ncd);
        println!("\n");
    }
}

//Read bytes from the device using a control transfer type
pub fn read_from_micro(context: &Context) {
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

        //lossy will turn every valid utf8 character into the appropriate symbol, while the invalid
        //ones will be shown as this symbol: � (it returns a smart pointer)
        let buf_to_string = String::from_utf8_lossy(&buffer);
        println!("From device: {}", buf_to_string);
    }
}

//NOTE: Seems to work fine, there are some problems that are catched but should be fixed
pub fn list_usb_devices(context: &Context) {
    //Check if the device list is empty
    if context
        .devices()
        .expect("Could not list usb devices!")
        .is_empty()
    {
        println!("No usb devices detected...");
        exit(1);
    } else {
        //Iterate over all devices listed by the context and print some info like VID, PID, max
        //packet size and so on

        //Setup loop over all devices
        for device in context
            .devices()
            .expect("Could not get iterator over usb devices from context")
            .iter()
        {
            //Get a DEVICE DESCRIPTOR for the current device
            let dev_desc = device
                .device_descriptor()
                .expect("Could not get the device descriptor for this device");

            //Add to our struct whatever info we can get without opening the device
            let mut device_info = DeviceDescriptor {
                vid: dev_desc.vendor_id(),
                pid: dev_desc.product_id(),
                mps: dev_desc.max_packet_size(),
                ncd: dev_desc.num_configurations(),
                man_id: String::from(""),
                prod_id: String::from(""),
            };

            //Let's open the device
            //Open device to get device handle, get string descriptor
            //This is needed for windows that refutes to open a device if it does not have
            //a driver for it.
            match device.open() {
                Ok(handle) => {
                    device_info.man_id = match handle.read_manufacturer_string_ascii(&dev_desc) {
                        Ok(string) => string,
                        Err(err) => {
                            println!("Could not read manufacturer ID: {}", err);
                            device_info.print();
                            continue;
                        }
                    };

                    device_info.prod_id = match handle.read_product_string_ascii(&dev_desc) {
                        Ok(string) => string,
                        Err(err) => {
                            println!("Could not read product ID: {}", err);
                            device_info.print();
                            continue;
                        }
                    }
                }
                Err(err) => {
                    println!("Could not open usb device: {}", err);
                    device_info.print();
                    continue;
                }
            };
            device_info.print();
        }
    }
}
