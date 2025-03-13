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
        stdin.read_line(&mut input).unwrap(); // read the input from the user and store it in input (unwrap() is used to handle the error if the input is not read)
        
        match input.trim() {
            "exit 0" => break,
            input if input.starts_with("echo") => println!("{}", &input[5..]),
             _ => println!("{}: command not found", input.trim())
            }
            
        }
    }

