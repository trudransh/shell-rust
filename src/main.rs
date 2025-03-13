#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    let mut input = String::new();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin(); // read input from user
        // create a new string to store the input
        stdin.read_line(&mut input).unwrap(); // read the input from the user
        if input.ends_with('\n') {
            input.pop();
        }
        println!("{}: command not found", input.trim()); 
        input.clear();// print the input from the user
    }
}
