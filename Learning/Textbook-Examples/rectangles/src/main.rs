#[derive(Debug)]
struct Rectangle {
    x: f64, //usually you make the fields private and u make getter functions to access the fields
    y: f64,
}

//NOTE: Self --> alias for the struct(used for returnig the struct itself), self --> alias for the module (used for methods)
//NOTE:
//Print rectangle as a method of the Rectangle struct
//Simply do function normally just refer to the rectangle as self
//Functions inside this impl are called ASSOCIATED FUNCTIONS, associated
//functions are more generally functions inside the impl but that might not have self as a
//prameter, like a constructor for example.
//Also note that you can have multiple impl blocks of the same struct
impl Rectangle {
    //Anything inside here can be accessed as a method of the struct Rectangle:
    //like so:
    // <rectangle-var-name>.method(param1, param2, ....)

    //Constructor, use like this: Rectangle::new(), the new functions is namespaced by the struct
    //(:: is used both for associated functions and namespaces created by modules (Ch. 7 Rust
    //Textbook))
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn new_square(xy: f64) -> Self {
        Self { x: xy, y: xy }
    }

    fn print(&self) {
        println!("Rectangle has the following sides:");
        println!("\t\t\tx: {}\n\t\t\ty: {}\n", self.x, self.y);
    }

    fn area(self: &Rectangle) -> f64 {
        self.x * self.y
    }

    //Method that writes
    fn change_sides(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    //Getter function that returns the x field if it's a valid rectangle side
    fn x(&self) -> f64 {
        if self.x > 0.0 {
            self.x
        } else {
            -1.0
        }
    }
    fn y(&self) -> f64 {
        if self.y > 0.0 {
            self.y
        } else {
            -1.0
        }
    }
    //Setter function that returns the x field if it's a valid rectangle side
    fn set_x(&mut self, x: f64) {
        if x > 0.0 {
            self.x = x;
        } else {
            self.x = 0.0;
        }
    }
    fn set_y(&mut self, y: f64) {
        if y > 0.0 {
            self.y = y;
        } else {
            self.y = 0.0;
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.x > other.x && self.y > other.y
    }

    //NOTE: AUTOMATIC REFERENCING AND DEREFERCING
    //Rust uses automatic referencing and dereferencing of the self parameter in the methods
    //so whether the parameter is &mut, & Rust matches the signature of the method used.
}

/*
fn init_rectangle(x: f64, y: f64) -> Rectangle {
    // dbg!(Rectangle { x, y }) //Debugging macro, prints the struct, its fields and this line
    // number
    // Debug is a trait, other traits can be found in the Appendix C (also look for outer
    // attributes)
    Rectangle { x, y }
}

fn print_rectangle(rec: &Rectangle) {
    println!("Rectangle has the following sides:");
    println!("\t\t\tx: {}\n\t\t\ty: {}\n", rec.x, rec.y);
}

fn area(rec: &Rectangle) -> f64 {
    rec.x * rec.y
}
*/

fn main() {
    let x = 10.0;
    let y = 10.0;
    //Let's make a new rectangle struct
    //let mut rec1 = init_rectangle(x, y);
    let mut rec1 = Rectangle::new(x, y);
    //print_rectangle(&rec1);
    //Using the method:
    rec1.print();

    println!(
        "The area of the rectangle is {} squared pixels",
        rec1.area()
    );

    rec1.change_sides(std::f64::consts::E, std::f64::consts::PI);

    rec1.print();

    println!(
        "The area of the rectangle is {} squared pixels",
        rec1.area()
    );

    //Using setter functions
    rec1.set_x(69.69);
    rec1.set_y(69.69);

    //Using getter functions
    let side_x = rec1.x();
    let side_y = rec1.y();

    println!("side_x: {side_x}");
    println!("side_y: {side_y}");

    let rec2 = Rectangle::new(69.6899, 1.0);
    let rec3 = Rectangle::new_square(0.000001);

    println!("Can rec1 hold rec2 ? {}", rec1.can_hold(&rec2));
    println!("Can rec3 hold rec2 ? {}", rec3.can_hold(&rec2));
}
