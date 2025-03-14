#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;
use lazy_static::lazy_static;

fn main() {
    // Uncomment this block to pass the first stage
    lazy_static!{
        static ref BUILTIN_COMMANDS :HashMap<&'static str , bool> = {
            let mut map = HashMap::new();
            map.insert("exit", true);
            map.insert("type", true);
            map.insert("echo", true);
            map
        };
    }
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();// Wait for user input
        let stdin = io::stdin(); // read input from user
        // create a new string to store the input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap(); // read the input from the user and store it in input (unwrap() is used to handle the error if the input is not read)
        
        let mut input_whitespace = input.split_whitespace(); // split the input into whitespace and store it as an iterator
        let command: Option<&str> = input_whitespace.next(); // get the first word of the input
        let args = input_whitespace.collect::<Vec<&str>>().join(" ");
        match command {
            Some("exit") => {
                std::process::exit(0);
            }
            Some("echo") => {
                println!("{}", args);
            }
            Some("type") =>  {
                if BUILTIN_COMMANDS.contains_key(args.as_str()) {
                    println!("{}: is a shell builtin",args);
                } else {
                    println!("{}: not found", args);
                }
            }
            _ => {
                println!("{}: command not found", command.unwrap());
            }  
        }
        // if BUILTIN_COMMANDS.contains_key()
        // match input.trim() {
        //     "exit 0" => break,
        //     input if input.starts_with("echo") => println!("{}", &input[5..]),
        //      _ => println!("{}: command not found", input.trim())
        //     }
            
        // }
    }
}

