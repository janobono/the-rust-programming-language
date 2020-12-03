// error-handling.rs

// Rust groups errors into two major categories: recoverable and unrecoverable errors.
// For a recoverable error, such as a file not found error, it’s reasonable to report the problem
// to the user and retry the operation.
// Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array.

// Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors
// and the panic! macro that stops execution when the program encounters an unrecoverable error.

// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up
// the stack and cleans up the data from each function it encounters.
// But this walking back and cleanup is a lot of work.
// The alternative is to immediately abort, which ends the program without cleaning up.
// Memory that the program was using will then need to be cleaned up by the operating system.
// You can switch from unwinding to aborting upon a panic by adding panic = 'abort'
// to the appropriate [profile] sections in your Cargo.toml file.

use std::env;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    {
        let args: Vec<String> = env::args().collect();
        let arg1 = args.get(1);
        match arg1 {
            None => panic!("Define least one argument!"),
            Some(v) => println!("arg1 = {}", v),
        }
    }

    {
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }

    {
        // Matching on Different Errors
        let f = File::open("hello.txt");
        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };
    }

    {
        // Shortcuts for Panic on Error: unwrap and expect
        let f = File::open("hello.txt").unwrap(); // this will panic if error
        let f = File::open("hello.txt").expect("Failed to open hello.txt"); // Modified error message
    }

    {
        // Propagating Errors
        let user_name = read_username_from_file();
        println!("{:?}", user_name);
    }

    {
        // A Shortcut for Propagating Errors: the ? Operator
        let user_name = read_username_from_file2();
        println!("{:?}", user_name);
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
