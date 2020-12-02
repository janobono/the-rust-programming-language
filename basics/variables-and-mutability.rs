// variables-and-mutability.rs

fn main() {
    // Variable declaration and assignment
    let this_is_variable; // variable declaration
    this_is_variable = 3; // now variable is initialized with data type i32
    println!("this_is_variable={}", this_is_variable);

    let another_variable = "value"; // variable declared and assigned in one line
    println!("another_variable={}", another_variable);

    let full_defined_variable: i64 = 1000; // variable declared, data type annotated and assigned value
    println!("full_defined_variable={}", full_defined_variable);

    let suffix_annotation = 5i32; // variable type defined by suffix annotation
    println!("suffix_annotation={}", suffix_annotation);

    // Variable shadowing
    let is_variable_even = 3;
    println!("is_variable_even={}", is_variable_even);
    let is_variable_even = is_variable_even % 2 == 0;
    println!("is_variable_even={}", is_variable_even);

    // mutable variable
    let mut add = 3;
    add += 5;
    println!("add = {}", add);

    // Variable shadowing mutability
    let sub = 3;
    let sub = sub - 1;
    let sub = sub - 2;
    println!("sub = {}", sub);

    // Constants
    const MAX_POINTS: u32 = 100_000; // type must be defined, cant be changed and shadowed
    println!("MAX_POINTS = {}", MAX_POINTS);
}
