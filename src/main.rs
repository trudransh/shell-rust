#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    let mut input = String::new();
    const EXIT_CODE : u32 = 0;
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin(); // read input from user
        // create a new string to store the input
        stdin.read_line(&mut input).unwrap(); // read the input from the user and store it in input (unwrap() is used to handle the error if the input is not read)
        if input.ends_with('\n') {
            input.pop(); // remove the newline character from the input
        }
        println!("{}: command not found", input.trim());
        if input.trim() == "exit" {
            println!("exit {}", EXIT_CODE);
            break;

        }
        input.clear();// print the input from the user
    }
}
