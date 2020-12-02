// functions.rs

// function with parameter
fn print_number(number: i32) {
    println!("number = {}", number);
}

// function with parameters returning value
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let x = sum(5, 5);
    print_number(x);
}
