// common-collections.rs
use std::collections::HashMap;

fn main() {
    // A vector allows you to store a variable number of values next to each other.
    {
        let v: Vec<i32> = Vec::new();
        println!("{:?}", v);

        let v = vec![1, 2, 3];
        println!("{:?}", v);

        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        v.pop();
        println!("{:?}", v);

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        println!("The x element is {:?}", v.get(100));

        // Iterating over the Values in a Vector
        for i in &v {
            println!("{}", i);
        }

        // modify mutable references
        let mut v = vec![1, 2, 3];
        for i in &mut v {
            *i += 50;
        }
        println!("{:?}", v);
    }

    // A string is a collection of characters.
    {
        let mut s = String::new();
        s.push('i');
        s.push_str("nitial contents");
        println!("{}", s);

        let data = "initial contents";
        let s = data.to_string();
        println!("{}", s);

        let s = "initial contents".to_string();
        println!("{}", s);

        let s = String::from("initial contents");
        println!("{}", s);

        // Concatenation with the + Operator or the format! Macro
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("{}", s3);
        println!("{}", s2);
        // println!("{}", s1); s1 is borrowed so can't be used

        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = format!("{} {}", s1, s2); // ownership not changed
        println!("{}", s3);
        println!("{}", s2);
        println!("{}", s1);

        // Slicing Strings
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("{}", s);

        for c in "नमस्ते".chars() {
            println!("{}", c);
        }
    }

    // A hash map allows you to associate a value with a particular key.
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        println!("{:?}", scores);

        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
        println!("{:?}", scores);

        // Hash Maps and Ownership
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point
        println!("{:?}", map);

        // Accessing Values in a Hash Map
        let field_name = String::from("Favorite color");
        let field_value = map.get(&field_name);
        println!("{:?}", field_value);

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }

        // Overwriting a Value
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);
        println!("{:?}", scores);

        // Only Inserting a Value If the Key Has No Value
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        println!("{:?}", scores);

        let text = "hello world wonderful world";
        let mut map = HashMap::new();
        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
}
