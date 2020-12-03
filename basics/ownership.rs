// ownership.rs

// Memory is managed through a system of ownership with a set of rules that the compiler checks at compile time.
// None of the ownership features slow down your program while it’s running.

// Ownership Rules
// - Each value in Rust has a variable that’s called its owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

fn main() {

    // Variable Scope
    {
        let s = "hello"; // s is valid from this point forward
        // do stuff witch s
        println!("{}, world!", s);
    } // this scope is now over, and s is no longer valid

    // Move
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}, world!", s1); - variable is moved so we can't use it
        println!("{}, world!", s2);
    }

    // Clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("{}, world!", s1);
        println!("{}, world!", s2);
    }

    // Copy
    {
        let x = 5;
        let y = x;
        // stack variables are copied so we don't have to use clone for heap variables Copy trait must be implemented
        // integer types, boolean type, floating point types, character type, tuples, if they only contain types that are also Copy.

        println!("x = {}, y = {}", x, y);
    }

    // Ownership and Functions
    {
        let s = String::from("hello");  // s comes into scope
        takes_ownership(s); // s's value moves into the function...
        // ... and so is no longer valid here
        //  println!("{}, world!", s); this is not valid

        let x = 5; // x comes into scope
        makes_copy(x);                  // x would move into the function,
        // but i32 is Copy, so it’s okay to still use x afterward
        println!("x can be used {}", x)
    }

    // Return Values and Scope
    {
        let s1 = gives_ownership(); // gives_ownership moves its return
        // value into s1
        println!("{}, world!", s1);

        let s2 = String::from("hello"); // s2 comes into scope
        println!("{}, world!", s2);

        let s3 = takes_and_gives_back(s2);  // s2 is moved into
        // takes_and_gives_back, which also
        // moves its return value into s3
        println!("{}, world!", s3);

        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len); // this is for example reference is better solution
    }
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("takes ownership {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("makes copy {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String { // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("hello"); // some_string comes into scope
    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
