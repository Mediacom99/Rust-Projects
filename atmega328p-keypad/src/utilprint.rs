//Utility functions like printing status of objects, usb devices and so on. These are all functions
//that only give information to the user by printing something on stdout

use crate::TIMEOUT;
use rusb::*;
use std::process::exit;

///Device Descriptor Structure, contains VID, PID, Max Packet Size, Num of Config descriptors, Man
///ID and Prod ID
struct DeviceDescriptor {
    vid: u16,        //Vendor id
    pid: u16,        //Product id
    mps: u8,         //max packet size
    ncd: u8,         //num of config descriptors
    man_id: String,  //Manufacturer ID
    prod_id: String, //Product ID
}

///DeviceDescriptor struct implementation
impl DeviceDescriptor {
    ///Prints VID, PID, Max Packet Size and Number of Config descriptors
    fn print(&self) {
        println!("VID: {:x}", self.vid);
        println!("PID: {:x}", self.pid);
        println!("Max packet size: {}B", self.mps);
        println!("Number of config descriptors: {}", self.ncd);
    }

    ///Prints Manufacturer ID and Product ID if set, otherwise needs to be initialized to an empty
    ///line or a space
    fn print_string(&self) {
        println!("Manufacturer ID: {}", self.man_id);
        println!("Product ID: {}", self.prod_id);
    }
}

/*











*/

///List all usb devices, get Manuf and Prod ID, Max Packet size and Num of config descriptor
///Since in order to read the ID's I have to open a device and silly Windows does not let me do
/// that if that device is either not HID or using WinUSB or some other windows driver...
/// So for example you will not get any information on a Focusrite for example, even though
/// it's using the USB protocol... */
///NOTE: Seems to work fine, there are some problems that are expected but should be handled
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
            print_device_info(&device);
        }
    }
}

///Prints VID, PID, Max Packet Size and Number of Configurations always. Prints also the Manufacturer ID and the Product ID if the device can
///be opened.
fn print_device_info(device: &Device<Context>) {
    let dev_desc = device
        .device_descriptor()
        .expect("Could not get the device descriptor for this device");

    let mut device_info = DeviceDescriptor {
        vid: dev_desc.vendor_id(),
        pid: dev_desc.product_id(),
        mps: dev_desc.max_packet_size(),
        ncd: dev_desc.num_configurations(),
        man_id: String::from(""),
        prod_id: String::from(""),
    };

    match device.open() {
        Ok(handle) => {
            device_info.man_id = match handle.read_manufacturer_string_ascii(&dev_desc) {
                Ok(string) => string,
                Err(_err) => {
                    // println!("Could not read manufacturer ID: {}", err);
                    device_info.print();
                    println!(" ");
                    return;
                }
            };

            device_info.prod_id = match handle.read_product_string_ascii(&dev_desc) {
                Ok(string) => string,
                Err(_err) => {
                    // println!("Could not read product ID: {}", err);
                    device_info.print();
                    println!(" ");
                    return;
                }
            };

            device_info.print_string();
            device_info.print();
            println!(" ");
        }
        Err(_err) => {
            // println!("Could not open usb device: {}", err);
            device_info.print();
            println!(" ");
        }
    };
}

/*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
*
* */

/// Prints all the information rusb can gather from an opened device and its handle,
/// it will print configuration, interface and endpoint descriptors and list the transfer type,
/// this will be done for every configuration, every interface and every configured endpoint.
///
//FIXME:CHECK THAT ITS ALL GOOD AND IMPLEMENT IT CORRECTLY WITH THE OTHER FUNCTIONS
fn get_device_handle_info(dhandle: &DeviceHandle<Context>) {
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
