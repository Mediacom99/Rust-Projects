//Utility functions like printing status of objects, usb devices and so on. These are all functions
//that only give information to the user by printing something on stdout

use crate::{PID, TIMEOUT, VID};
use rusb::*;
use std::process::exit;

//Device Descriptor Structure and implementation
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

/* List all usb devices, get Manuf and Prod ID, Max Packet size and Num of config descriptor
Since in order to read the ID's I have to open a device and silly Windows does not let me do
that if that device is either not HID or using WinUSB or some other windows driver...
So for example you will not get any information on a Focusrite for example, even though
it's using the USB protocol... */
//NOTE: Seems to work fine, there are some problems that are expected but should be handled
pub fn list_usb_devices(context: &Context) {
    println!("Listing usb devices...\n");
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

            /* Let's open the device
            Open device to get device handle, get string descriptor
            This is needed for windows that refutes to open a device if it does not have
            a driver for it. */
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

/// NEEDS TO BE FIXED: prints a bunch of info about the device given by the const values VID and
/// PID
pub fn micro_get_info(context: &Context) {
    //Open and get a handle to a device with a certain VID and PID
    let dhandle = match context.open_device_with_vid_pid(VID, PID) {
        Some(res) => res,
        None => {
            println!("Could not open device");
            exit(1);
        }
    };

    //Get device struct from device handle
    let device = dhandle.device();

    //Get device language
    let lang = dhandle
        .read_languages(TIMEOUT)
        .expect("Could not read device languages")[0];

    let cfg_desc = match device.active_config_descriptor() {
        Ok(cfg) => cfg,
        Err(err) => {
            println!(
                "Could not read active configuration descriptor from device! Error: {}",
                err
            );
            exit(13);
        }
    };

    //Read string config descriptor
    match dhandle.read_configuration_string(lang, &cfg_desc, TIMEOUT) {
        Ok(ok) => println!("Config descriptor:\n{}", ok),
        Err(err) => println!("Could not read config string descriptor. Error: {}", err),
    };

    println!("Device Max Power in milliamps: {}", cfg_desc.max_power());
    println!("Number of interfaces: {}", cfg_desc.num_interfaces());

    //Iterate over interfaces in the current configuration:
    for interface in cfg_desc.interfaces() {
        //Iterate over current interface descriptors
        for desc in interface.descriptors() {
            println!("Interface number: {}", desc.interface_number());
            println!("ALternate setting number: {}", desc.setting_number());
            println!("Class code: {}", desc.class_code());
            println!("Sub class code: {}", desc.sub_class_code());
            println!("Number of endpoints: {}", desc.num_endpoints());
            for edesc in desc.endpoint_descriptors() {
                println!("Endpoint address: {}", edesc.address());
                println!("Endpoint number: {}", edesc.number());
                println!("Max packet size: {}", edesc.max_packet_size());
                match edesc.direction() {
                    Direction::In => {
                        println!("Direction: IN");
                    }
                    Direction::Out => {
                        println!("Direction: OUT");
                    }
                }
                match edesc.transfer_type() {
                    TransferType::Control => println!("Transfer: control"),
                    TransferType::Isochronous => println!("Transfer: isochronous"),
                    TransferType::Bulk => println!("Transfer: bulk"),
                    TransferType::Interrupt => println!("Transfer: interrupt"),
                }
            }

            //Read interface descriptor string
            match dhandle.read_interface_string(lang, &desc, TIMEOUT) {
                Ok(ok) => println!("Interface {} descriptor:\n{}", desc.interface_number(), ok),
                Err(err) => println!("Could not read interface string descriptor. Error: {}", err),
            };
        }
    }
}
