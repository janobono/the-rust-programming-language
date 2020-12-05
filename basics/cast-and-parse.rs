// cast-and-parse.rs

fn main() {
    {
        let decimal = 65.4321_f32;
        println!("original f32 value {}", decimal);

        // cast to unsigned int 8-bit
        let integer = decimal as u8;
        println!("cast to u8 value {}", integer);

        // integer can be cast to char
        let character = integer as char;
        println!("integer to char {}", character);

        println!("Casting: {} -> {} -> {}", decimal, integer, character);

        // 1000 already fits in a u16
        println!("1000 as a u16 is: {}", 1000 as u16);

        // we must use #![allow(overflowing_literals)] to enable this
        // 1000 - 256 - 256 - 256 = 232
        // Under the hood, the first 8 least significant bits (LSB) are kept,
        // while the rest towards the most significant bit (MSB) get truncated.
        //println!("1000 as a u8 is : {}", 1000 as u8);
        // -1 + 256 = 255
        //println!("  -1 as a u8 is : {}", (-1i8) as u8);
    }

    {
        // we should parse from
        let decimal = "65.4321".parse::<f32>().unwrap();
        println!("value {}", decimal);
        let decimal = String::from("65.432").parse::<f32>().unwrap();
        println!("value {}", decimal);
        let decimal = String::from("65.4321")[..5].parse::<f32>().unwrap();
        println!("value {}", decimal);
    }
}
