fn main() {
    let test = false;

    //If is an expression, thus I can use it on the right side of a let statement
    let number = if test {
        5
    } else if !test {
        6
    } else {
        1
    };

    println!("Number: {number}");

    //Blocks of code evaluate to the last value without a semicolumn, numbers are epxressions of
    //themselves
    //
    //

    //You can also specify loop labels to use break and continue with the labels
    //
    let mut count = 0;
    'counting_up: loop {
        println!("Count = {}", count);
        let mut remaining = 10;

        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break; //No label = breaking inner loop
            }
            if count == 2 {
                break 'counting_up; //Label = break the specified loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //For loop to run through an iterator
    //
    let a: [u32; 100] = [69; 100];

    //elements cannot be set to smth, you can only access them, it seems
    for elements in a {
        println!("{elements}");
    }
}
