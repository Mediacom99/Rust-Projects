/*
*
* Enums are a way to wrap a "general" struct and its "variants"  (very similar structs) together in
* one single object called an enumeration
*/
#[derive(Debug)]
struct IpV4 {
    addr: [u8; 4],
}

#[derive(Debug)]
struct IpV6 {
    addr: String,
}

#[derive(Debug)]
enum State {
    Ok(u8),
    Err(u8),
}

#[derive(Debug)]
enum IpAddress {
    V4(IpV4), //Variant V4 of IpAddress enum that contains a IpV4 struct
    V6(IpV6),
    Status(State), //You can also put an enum inside another enum
                   //NOTE: whatever is inside the parenthesis is called an ASSOCIATED VALUE
                   //also the variants become a constructor for that particular variant when called with the
                   //parenthesis.
                   //The input is the associated data to that variant and the ouput is an instance of that variant
                   //with its enum type
}

//Implement methods and associated functions to an enum
impl IpAddress {
    //Very fucking cool, you can match the enum to do something different for each variant and the
    //associated data of that variant!
    fn print(&self) {
        match self {
            IpAddress::V4(ipv4) => {
                println!(
                    "IP V4: {}.{}.{}.{}",
                    ipv4.addr[0], ipv4.addr[1], ipv4.addr[2], ipv4.addr[3]
                );
            }
            IpAddress::V6(ipv6) => {
                println!("IP V6: {}", ipv6.addr);
            }
            IpAddress::Status(state) => match state {
                State::Ok(val) => {
                    println!("Status: {}", val);
                }
                State::Err(err) => {
                    println!("Error: {}", err);
                }
            },
        }
    }
}

fn main() {
    //Create an instance of a variant:
    let type_four = dbg!(IpAddress::V4(IpV4 {
        addr: [192, 168, 1, 1],
    })); //NOTE: variants are namespaced under its identifier (the enum name), so that different
         //variants can be of the same type
    let status_ip_ok = IpAddress::Status(State::Ok(0)); //Instance of Status variant of IpAddress
    let status_ip_err = IpAddress::Status(State::Err(3)); //that is associated to the enum State in the variant Ok with the associated value 0
    let type_six = IpAddress::V6(IpV6 {
        addr: String::from("2001:db8:3333:4444:5555:6666:7777:8888"),
    });

    type_four.print();
    type_six.print();
    status_ip_ok.print();
    status_ip_err.print();

    //Interesting standard library enum: Option<T> (T is a generic parameter, it can be whatever
    //type)
    //This could be extremely useful for example to check that the user input is of the correct
    //type!
    let option: Option<String> = None;
    let option1: Option<String> = Some(String::from("Hello World"));

    match option {
        Some(value) => {
            println!("{value}")
        }
        None => {
            println!("Not a valid String type!")
        }
    }
    match option1 {
        Some(value) => {
            println!("{value}")
        }
        None => {
            println!("Not a valid String type!")
        }
    }

    //Match something and do something else with whatever else comes out

    let number: u8 = 24;
    match number {
        0 => println!("It's zero: 0"),
        1 => println!("It's one: 1"),
        somethig_else => println!("It's somethig_else: {somethig_else}"),
    }

    //Or maybe you want to do smth that does not require the value:
    match number {
        0 => println!("It's zero: 0"),
        1 => println!("It's one: 1"),
        _ => println!("It's whatever else but I don't need to use it!"),
    }

    //Or maybe you don't want to do anything with the other cases:
    match number {
        0 => println!("It's zero!"),
        1 => println!("It's one!"),
        2 => println!("It's two!"),
        _ => {
            println!("Don't need to run any code!");
            // ()   You can omit the unit expression '()' if there is at least one statement
        }
    }

    //If let syntax:
    /*
        This two code blocks are the same:

        let config: u38 = 0;

        match config = {
        0, => println("It's zero"),
        _ => (),
        }

        if let 0 = config {
        println!("It's zero!");
    } */

    let config: Option<u32> = Some(420);
    // let config: Option<u32> = None;

    if let Some(num) = config {
        println!("The Some number is: {}", num);
    }

    //Ofc  you can do a if let else block like this:
    if let Some(num) = config {
        println!("There's some number: {}", num);
    } else {
        println!("There is no number!");
    }
}
