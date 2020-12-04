// fahrenheit-and-celsius.rs
// Simple program to convert fahrenheit to celsius and vice versa.
use std::env;

enum InputValue {
    Fahrenheit(f64),
    Celsius(f64),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = args.get(1);
    match arg1 {
        None => panic!("Please input value!"),
        Some(val) => match parse(val) {
            InputValue::Fahrenheit(f) => println!("{:.2}C", fahrenheit_to_celsius(f)),
            InputValue::Celsius(c) => println!("{:.2}F", celsius_to_fahrenheit(c)),
        }
    };
}

fn parse(value: &str) -> InputValue {
    let mut value = String::from(value);
    match value.pop().unwrap() {
        'F' => InputValue::Fahrenheit(value.parse::<f64>().unwrap()),
        'C' => InputValue::Celsius(value.parse::<f64>().unwrap()),
        _ => panic!("Unsupported value format! Value must ends with F or C character!"),
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 1.8 + 32.0
}
