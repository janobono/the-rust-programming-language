// enums-and-pattern-matching.rs

// Enums allow you to define a type by enumerating its possible variants.

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn print_message(message: Message) {
    println!("{}",
             match message {
                 Message::Quit => String::from("quit"),
                 Message::Write(message) => format!("{}", message),
                 _ => String::from("default"),
             },
    );
}

fn main() {
    {
        print_message(Message::Write(String::from("hello, world!")));
        print_message(Message::Quit);
    }

    // The Option<T> Enum and Its Advantages Over Null Values
    {
        let v = vec![1, 2, 3];
        let none = v.get(3);
        match none {
            None => println!("None"),
            Some(value) => println!("Some({})", value),
        };
    }
}
