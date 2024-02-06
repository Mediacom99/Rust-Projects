/*
*   1. Create Hidapi context
*   2. reset devices
*   3. add device with VID and PID
*   4. open that device
*   5. Do smth using HidDevice (read write)
* */
use rusb::Context;
mod rusbutils;

fn main() {
    //Create new libusb context
    let context = Context::new().expect("Could not create a libusb context!");

    //List all usb devices, get Manuf and Prod ID, Max Packet size and Num of config descriptor
    //Since in order to read the ID's I have to open a device and silly Windows does not let me do
    //that if that device is either not HID or using WinUSB or some other windows driver...
    //So for example you will not get any information on a Focusrite for example, even though
    //it's using the USB protocol...
    // rusbutils::list_usb_devices(&context);

    //Read bytes from the micro, turn them into a utf8 string if possible and print it out
    rusbutils::read_from_micro(&context);
}
