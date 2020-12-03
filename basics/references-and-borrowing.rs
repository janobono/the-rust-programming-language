// references-and-borrowing.rs

fn main() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}, world!", s);
    }

    {
        // A data race is similar to a race condition and happens when these three behaviors occur:
        // - Two or more pointers access the same data at the same time.
        // - At least one of the pointers is being used to write to the data.
        // - There’s no mechanism being used to synchronize access to the data.

        // You can have only one mutable reference to a particular piece of data in a particular scope
        let mut s = String::from("hello");
        let r1 = &mut s;
        //let r2 = &mut s; -- second borrow
        //println!("{}, {}", r1, r2);
        println!("{}, world!", r1);
    }

    {
        // combining mutable and immutable references is not allowed
        let mut s = String::from("hello");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        // let r3 = &mut s; // BIG PROBLEM
        // println!("{}, {}, and {}", r1, r2, r3);
        println!("{}, {}, and {}", r1, r2, s);
    }

    {
        // cannot have a mutable reference while we have an immutable one
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("{} and {}", r1, r2);
        // r1 and r2 are no longer used after this point

        let r3 = &mut s; // no problem
        println!("{}, world!", r3);
    }

    // Dangling References
    // In languages with pointers, it’s easy to erroneously create a dangling pointer, a pointer that
    // references a location in memory that may have been given to someone else, by freeing some memory
    // while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that
    // references will never be dangling references: if you have a reference to some data, the compiler
    // will ensure that the data will not go out of scope before the reference to the data does.
    {
        let s = dangle();
        println!("{}, world!", s);
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
//fn dangle() -> &String {
    let s = String::from("hello");

    //&s
    // we return a reference to the String, s
    // } Here, s goes out of scope, and is dropped. Its memory goes away.
    // Danger!

    s
}
