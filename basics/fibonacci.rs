// fibonacci.rs
// Generate the nth Fibonacci number.

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg1 = args.get(1);
    match arg1 {
        None => panic!("Please input value!"),
        Some(value) => {
            let n: u32 = value.parse::<u32>().unwrap();
            println!("{}", fibonacci(n));
        },
    };
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
