

/*
*   1. Create Hidapi context
*   2. reset devices
*   3. add device with VID and PID
*   4. open that device
*   5. Do smth using HidDevice (read write)
* */

use hidapi::*;
// use hidapi::HidError;

const VID: u16 = 1;
const PID: u16 = 2;

fn main(){

    //Initialize an HidApi contex 
    let api = HidApi::new().expect("Failed to initialize HidApi context");

    //Reset devices
    api.reset_devices().error("Could not reset the hid devices");

    //Add device with certain VID and PID
    
}
