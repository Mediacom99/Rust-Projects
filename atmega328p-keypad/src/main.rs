//Main binary

//This crate's library
use atmega328p_keypad::*;
use std::{env, process::exit};

fn print_help() {
    println!("Provide at least one argument from the following:");
    println!("\tlist                                    List connected usb devices");
    println!("\tread-control                            Read from micro using control transfer");
    println!("\twrite-read-control                      Write as many as 100 bytes to the micro he sends them back!");
    println!("\tread-interrupt                          Read from micro using interrupt transfer");
    println!("\tmicro-get-info                          Get as many information about active configuration, interfaces and endpoints");
    println!("\thelp                                    Prints this help message");
}

fn main() {
    //Get command line arguments
    let arguments: env::Args = env::args();
    if arguments.len() < 2 {
        println!(
            "What do you want to do ? Choose a command and pass it as an argument to the program!"
        );
        print_help();
        exit(0);
    }

    let command = arguments.last().unwrap();

    //Init libusb context
    let context = init_context();

    match command.as_str() {
        "list" => {
            utilprint::list_usb_devices(&context);
        }
        "read-control" => {
            micro_control_read(&context);
        }
        "write-read-control" => {
            micro_control_write_read(&context);
        }
        "read-interrupt" => {
            micro_interrupt_read(&context);
        }
        "micro-get-info" => {
            micro_get_info(&context);
        }
        "help" => {
            print_help();
        }

        _ => {
            println!("Not a valid command!");
            exit(99);
        }
    }
}
