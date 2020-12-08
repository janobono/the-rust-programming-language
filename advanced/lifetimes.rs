// lifetimes.rs

// &i32        a reference
// &'a i32     a reference with an explicit lifetime
// &'a mut i32 a mutable reference with an explicit lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime Annotations in Method Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    {
        let p = ImportantExcerpt { part: "part one" };
        println!("p part {}", p.part);
        println!("p level {}", p.level());
    }

    {
        // The Static Lifetime
        // The text of this string is stored directly in the program’s binary, which is always
        // available. Therefore, the lifetime of all string literals is 'static.
        let s: &'static str = "I have a static lifetime.";
        println!("{}", s);
    }

    {
        // Generic Type Parameters, Trait Bounds, and Lifetimes Together
        longest_with_an_announcement("str01", "str001", "milk is sold");
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
