/*

    Binary crate: executable
    Library crate: code to be used in another program and can't be executed on its own

    Chapter II of The Rust Programming Language

*/

use rand::Rng; //Using the Rng trait of rand crate
use std::cmp::Ordering;
use std::io;
use std::process::exit;

// Rust implicitely puts in scope std::string where String is the struct
fn main() {
    let mut secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!\n\n");

    loop {
        let mut guess = String::new();

        // println!("The secret number is: {secret_number}");

        println!("Please input your guess:");

        let stdin = io::stdin(); //get a handle to the stdin stream

        //read stdin stream line, place it in guess String
        // let br =
        stdin.read_line(&mut guess).expect("Failed to read line");
        // println!("Bytes read: {br}");

        //shadowing the variable name, trim whitespaces, parse into u32 and check input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please write a number!");
                continue;
            } //jump to next loop iteration from here, ignore next code
        };

        println!("You guessed: {guess}");

        //Check if it matches the secret number
        //guess.cmp() -> Ordering type (Less and others are variants of that type)
        //than you do basically a switch statement with the three variants.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");

                let mut again = String::new();
                println!("Wanna play again? Then type \"yes\"");
                stdin.read_line(&mut again).expect("Failed to read line");
                // if again.trim().eq(&String::from("yes")) {
                // if again.trim().eq("yes") {
                if again.trim() == "yes" {
                    secret_number = rand::thread_rng().gen_range(1..=100);
                    println!("Changed secret number!\n");
                    continue;
                } else {
                    println!("\nThanks for playing!\n");
                    exit(0);
                }
            }
        }

        println!("Try again!\n");
    }
}
