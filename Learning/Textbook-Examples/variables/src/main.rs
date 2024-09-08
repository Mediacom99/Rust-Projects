// Variables and Mutability

use std::io;

fn main() {
    //Mutable variable
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //Constants (always capitalized)
    //Hardcoded values
    const CONSTANT_VALUE: u32 = 69;
    println!("Const value: {CONSTANT_VALUE}");

    //Variable shadowing (even between different types, damn), follows scope rules
    let var = 5;
    println!("Var: {var}");

    {
        let var = "Hello";
        println!("Var inside block: {var}");
    }

    println!("Var outside block: {var}");

    //It follows the last call in the current scope
    //
    /*
     * Data types:
     * scalar type = single value: integers, float, numbers, boolean and char
     */
    //Integers: from 8 to 128 bit and arch
    let numero_big: u8 = 0b11111111;
    println!("{numero_big}");

    //Floting point: f32, f64
    let float: f64 = std::f64::consts::PI;
    println!("{float}");
    /*
    * a char in rust is 4Bytes
    "" -> string literals
    '' -> char literals
    */

    // Compund type:
    //  Tuple: fixed length grouping of a number of values with a variety of primary types (integer,
    //  uinteger, float, bool and stuff)
    let tup: (u8, u16, u32) = (1, 2, 3);
    // let (x, y, z) = tup; //Tuple destructuring
    //Accessing tuple: <tuple-name>.<index-element>
    let t = tup.0;
    let f = tup.1;
    let n = tup.2;
    println!("{t},{f},{n}");
    //Empty tuple:
    // let empty_tuple: () = (); --> it's called a unit (tuple with no values), represents empty
    // return type
    //
    // Array: same type, fixed length (While Vector in std is the dynamic one)
    // let vect: [u32; 5] = [1, 2, 3, 4, 5];
    //Array syntax:
    // let <array-name>: [<data-type>;<number of elements>] = [el1, el2, el3, ...];
    // let <name> = [<element>,<num of repetitions>];
    let vect2 = [std::f64::consts::PI; 10];
    println!("{}", vect2[0]);
    println!("{}", vect2[9]);
    //Rust protects against invalid memory accessing
    let mut usr_input = String::new();
    io::stdin()
        .read_line(&mut usr_input)
        .expect("Cannot read input from stdin");

    let index: usize = usr_input
        .trim()
        .parse()
        .expect("Index entered is not a number!");

    println!("Value of vector at {} is: {}", index, vect2[index]);
    //Try doing a seg fault and see
    //how Rust handles it by panicking instead of keep reading memory
}
