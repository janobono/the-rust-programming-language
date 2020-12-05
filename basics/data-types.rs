// data-types.rs

fn main() {
    // Scalar types - (single value)

    // Integer
    println!("integer 8-bit   [signed i8    min {}, max {}], [unsigned u8    min 0, max {}]", -128i8, 127i8, 255u8);
    println!("integer 16-bit  [signed i16   min {}, max {}], [unsigned u16   min 0, max {}]", -32_768i16, 32_767i16, 65_535u16);
    println!("integer 32-bit  [signed i32   min {}, max {}], [unsigned u32   min 0, max {}]", -2_147_483_648i32, 0b0111_1111_1111_1111_1111_1111_1111_1111i32, 0xffffffffu32);
    println!("integer 64-bit  [signed i64   min {}, max {}], [unsigned u64   min 0, max {}]", std::i64::MIN, std::i64::MAX, std::u64::MAX);
    println!("integer 128-bit [signed i128  min {}, max {}], [unsigned u128  min 0, max {}]", std::i128::MIN, std::i128::MAX, std::u128::MAX);
    println!("integer arch    [signed isize min {}, max {}], [unsigned usize min 0, max {}]", std::isize::MIN, std::usize::MAX, std::usize::MAX);

    // Floating-Point IEEE-754 standard
    let float: f32 = 1.0; // 32 bit float must be defined
    println!("float 32-bit {:.1}", float);
    let float = 2.0; // 64 bit float is default
    println!("float 64-bit {:.1}", float);

    // Numeric Operations
    let sum = 5 + 10; // addition
    println!("5 + 10 = {}", sum);
    let difference = 95.5 - 4.3; // subtraction
    println!("95.5 - 4.3 = {:.2}", difference);
    let product = 4 * 30; // multiplication
    println!("4 * 30 = {}", product);
    let quotient = 56.7 / 32.2; // division
    println!("56.7 / 32.2 = {}", quotient);
    let remainder = 43 % 5; // remainder
    println!("43 % 5 = {}", remainder);

    // Boolean
    let b1 = true;
    let b2 = false;
    println!("b1 = {}, b2 = {}", b1, b2);
    println!("b1 || b2 = {}", b1 || b2);
    println!("b1 && b2 = {}", b1 && b2);
    println!("!b1 = {}", !b1);

    // Character
    let c = 'c';
    println!("character should be {}", c);

    // Compound Types

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup ({}, {}, {})", tup.0, tup.1, tup.2);
    println!("tup {:?}", tup);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x = {} y = {} z = {}", x, y, z);

    // Array
    let a = [1, 2, 3];
    println!("[{}, {}, {}]", a[0], a[1], a[2]);
    let a: [i32; 3] = [1, 2, 3];
    println!("{:?}", a);
    let a = [3; 3];
    println!("{:?}", a);

    // Aliasing
    type NanoSecond = u64;
    let a: NanoSecond = 10;
    println!("NanoSecond {}", a);
}
