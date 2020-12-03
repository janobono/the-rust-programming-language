// fahrenheit-and-celsius.rs
// Simple program to convert fahrenheit to celsius and vice versa.
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let val: &str = &args[1];
    if val.contains("F") {

    } else if val.contains("C") {

    } else {
      panic!("Unit not specified!");
    }
}
