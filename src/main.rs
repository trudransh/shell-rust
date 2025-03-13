#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin(); // read input from user
        let mut input = String::new(); // create a new string to store the input
        stdin.read_line(&mut input).unwrap(); // read the input from the user
        println!("{}: command not found", input.trim()); // print the input from the user
        break;
    }
}
