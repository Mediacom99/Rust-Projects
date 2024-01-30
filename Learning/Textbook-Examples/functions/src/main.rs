/*

    Notes:
        Rust is an expressiom-based language so:
            1. statements = things that do smth but do NOT return any value
            2. expressions = evaulate smth and DO return a value
    So in Rust you can't do: "let a = (let y = 6)" because let is a statement, not an expression. In C you can because assigning returns the value assigned

    The most simple for of an expression is a scope block, or numbers or functions that return smth.
    You can do this:
    let a = {
            let x = 1;
            x + 1;
        };
    At this point a = 2

    Function paradigm:

    fn <function_name>(<arg1>: <arg1-type>, <arg2>:<arg2-type>,...) -> <type-to-return> {

    <expression>
    }

    IMPORTANT: semicolons turn expression into statement. An expression is NOT supposed to end with a semicolon
*/

fn main() {
    println!("Hello, world!");
    let result = another_function(1.5, 1.5);
    println!("Result: {result}");
    let a = {
        let x = 1;
        x + 1 //If you put a semicolon here, it does not work
    };
    println!("{a}");
    println!("{}", weird_number());
    println!("{}", weird_char());
}

fn another_function(x: f64, y: f64) -> f64 {
    x + y
}

//Weird but allowed function since a number is an expression that evaluates to the number it
//expresses
fn weird_number() -> i32 {
    5
}
fn weird_char() -> char {
    '$'
}

//This function evaluates to a unit, expressed by (), which is what a statement "evaluates to"
/*
fn weird_broken() -> i32 {
    5;
}
*/
