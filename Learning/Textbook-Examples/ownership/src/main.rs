/*
*
* Understanding ownership in Rust, Notes:
*
*   1. Every value has an owner, as soon as that owner goes out of scope the value is dropped
*
*/

fn main() {
    //String is allocated in the heap, that memory gets dropped as soon as the owner, thus 's' goes
    //out of scope. When that happens Rust calls automatically the function 'drop' that frees the
    //memory occupied.
    {
        let mut s = String::from("Hello");
        s.push_str(", World!");
        println!("{}", s);
        drop(s); //This gets called automatically
    }
    //Here s is not in scope anymore, whatever memory was used by s now it's free
    //
    //A String has three fields: pointer, length, capacity (these are on the stack), pointer points
    //to the first element of the array representing the String in heap
    //So letting a string equal to another string is just to copy its fields, they will both point
    //at the same location in memory!
    //
    //  EXAMPLE OF A """"" MOVE """""
    let s1 = String::from("Hey!"); //Allocated in the heap
    let s2 = s1; // --> THIS IS CALLED A 'move': s1 was moved into s2

    println!("{}", s2);
    //What happens when they both go out of scope since they point to the same mem address ?
    //This is called a double free error, thus Rust to solve this CONSIDERS s1  NO LONGER VALID
    //AFTER DECLARING s2!
    //
    //This gives an error:
    //println!("{}", s1);
    //
    //If you want to do a deep copy (thus allocate twice the memory in the heap), you
    //need to use a function of String
    //let s2 = s1.clone();
    //
    //You can also implement a Copy trait to a data type so that Rust does not move the first value
    //after cloning, rather it keeps both but only ON THE STACK (just like for integers in the stack)
    //
    //NOTHING THAT REQUIRES ALLOCATION IN THE HEAP CAN IMPLEMENT THE COPY TRAIT
    // Stack --> always deep copy
    // Heap --> always prefer shallow copy unless stated otherwise using a specific copy/clone
    // function
    //
    // For example a tuple containing only types that implement Copy can implement Copy,
    // a dynamic array unknown at compile time CANNOT implement COPY
    //

    //Let's look at this example:
    let s = String::from("hello");

    take_ownership(s); //Here s is moved into somestring, thus ownership has changed and s is no
                       //longer valid
                       //
                       //Here s is no longer VALID
                       // println!("{}", s); //THIS IS AN ERROR

    let gimme = gives_ownership();
    println!("Gimme before using reference instead of copy {}", gimme);

    //Thus the general rule is:
    //      1. Assigning a value to another variable moves it.
    //      2. When a variable that includes data in the heap goes out of scope, the value will be
    //         cleaned up by drop() unless ownership of the data has been moved to another
    //         variable.
    //
    //Little problem: is there any way we could pass a variable to a function without moving the
    //value in the ownership sense ? yes, REFERENCES AND BORROWING!
    // Neverming it's pretty easy, just pass a pointer to the variable and don't copy it!
    //
    println!("Length of gimme: {}", calculate_lentgh(&gimme));
    //Here I can still access gimme:
    println!("Gimme after using calculate_lentgh: {}", gimme);

    //The action of creating a reference is called borrowing
    //
    //Mutable references: ONLY ONE REFERENCE TO A VALUE PER SCOPE, NOT MORE!
    //Also you cannot have both a mut ref and a normal ref to the same value !
    //All of this is used to prevent data racing
    //
    //No dangling references: pointer that points to a mem address that is used by someone else.
    //The compiler checks that the data does not go out of scope before the references to that data
    //does
    //let ref_to_none = dangle();
    //
    //

    // STRING SLICES
}

//This function is an example of dangling references
//fn dangle() -> &String {
//    let s = String::from("Hello");
//    &s
//} //Here s is not a valid variable anymore, because it goes out of scope so it gets dropped
//(deallocated)

fn take_ownership(somestring: String) {
    println!("Inside take_ownership: {}", somestring); //Here somestring comes into scope as a copy
                                                       //shallow copy of the argument passed when this func is called.
} //Here somestring goes out of scope, thus drop() is automatically called
  //
  //
  // Return values can give ownership
fn gives_ownership() -> String {
    let s = String::from("here you go!");
    s //Here s' value is passed to whatever calls this function, thus s does not own anything
      //anymore
}
fn calculate_lentgh(s: &String) -> usize {
    s.len()
}
