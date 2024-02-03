use rusb::*;
use std::process::exit;

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
