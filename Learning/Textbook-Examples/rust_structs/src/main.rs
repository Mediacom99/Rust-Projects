//Define a struct
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = init_user(
        String::from("user1XdXd78787_d7s8"),
        String::from("user199secsi@yahoo.com"),
    );
    print_user(&user1);

    println!("\n\tUpdating username and email...\n");

    let user2 = init_from_old(
        user1,
        String::from("Mediacom"),
        String::from("bertoliedoardo99@gmail.com"),
    );
    //Here I cannot ever use user1 because username and email were moved to user2 fields!
    print_user(&user2);

    // Tuple Structures!
    println!("\n\nTuple Structures part:\n\n");
    // It's basically a tuple that uses the struct keyword to become its own type.
    struct Color(i32, i32, i32);

    let black = Color(0, 0, 0);

    //You can access the struct tuple elements just like a normal tuple
    println!("Color: [{} -- {} -- {}]", black.0, black.1, black.2);

    //And you can also destructure a struct tuple like a normal tuple:
    let Color(r, g, b) = black;
    println!("R: {r}, G: {g}, B: {b}");

    // NOTE: You can add references as fields of a struct but you need to add a lifetime to the
    // struct (Ch. 10 in Rust Textbook)

    //FIXME: wow this is very coooooool!

    //You can also use the macro dbg! which takes ownership of an expression, prints the file and
    //line number of the dbg call and the result of the expression, then gives back permission of
    //the value
    let user3 = dbg!(user2);
    println!("{:#?}", user3);

    //Since dgb! returns ownership we can use it like: let somethign = dgb!(expression which
    //evaluation needs to bind to something).

    //You can print a struct using the debug specifier '{:?} or {:#?} for pretty-print, but you
    //also have to add the debug info outer attribute to the struct'

    //println!("User2: {:#?}", user2);
}

fn init_user(username: String, email: String) -> User {
    User {
        active: true,
        sign_in_count: 1,
        username, //Shorthand to initialize struct, make sure that parameters name are the same as
        //the fields names
        email,
    }
}

fn print_user(user: &User) {
    println!("Username: {}", user.username);
    println!("IsActive?: {}", user.active);
    println!("Email: {}", user.email);
    println!("User sign in count: {}", user.sign_in_count);
}

fn init_from_old(old_user: User, username: String, email: String) -> User {
    // They do the same thing!
    /*
        User {
            active: old_user.active
            sign_in_count: old_user.sign_in_count,
            username,
            email,
        }
    */
    User {
        username, //Here if I use username or email from the old_user, the old_user gets dropped
        //because Strings are moved, not copied!
        email,
        ..old_user
    }
    //Anyway the older_user is gone because I'm not using a reference, thus at the end of this
    //function's scope old_user will be dropped!
}
