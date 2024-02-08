//Main binary

//This crate's library
use atmega328p_keypad::*;

fn main() {
    //Initialize new libusb
    let context = init_context();

    utilprint::list_usb_devices(&context);

    micro_control_read(&context);
}
