// slice.rs

// Another data type that does not have ownership is the slice. Slices let you reference a contiguous
// sequence of elements in a collection rather than the whole collection.

fn main() {
    {
        // A string slice is a reference to part of a String, and it looks like this:
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("{}, {}!", hello, world);
    }

    {
        let my_string = String::from("hello world");
        // first_word works on slices of `String`s
        let word = first_word(&my_string[..]);
        println!("{}, world!", word);

        let my_string_literal = "hello world";
        // first_word works on slices of string literals
        let word = first_word(&my_string_literal[..]);
        println!("{}, world!", word);

        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
        println!("{}, world!", word);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
