#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use lazy_static::lazy_static;

// lazy_static creates a global variable that is:
// 1. Initialized on first use (not at program start)
// 2. Computed only once, then cached for future access
// 3. Shared among all threads (unlike thread_local which creates per-thread instances)
lazy_static! {
    // &'static str means "a reference to a string that lives for the entire program lifetime"
    // - & means it's a reference (pointer to data)
    // - 'static is a "lifetime" that means "lives forever" (until program ends)
    // - str is Rust's string slice type
    static ref BUILTIN_COMMANDS: HashMap<&'static str, bool> = {
        // We use bool as the value type for simplicity
        // In a more advanced shell, we could store function pointers or command metadata instead
        let mut map = HashMap::new();
        map.insert("echo", true);
        map.insert("exit", true);
        map.insert("type", true);
        map // Return value has no semicolon - this is how Rust returns values from blocks
    };
}

fn main() {
    let stdin = io::stdin(); // Create a handle to standard input
    
    // REPL: Read-Eval-Print Loop - the core structure of a shell
    loop {
        // Read: Get input from user
        print!("$ "); // Print prompt
        io::stdout().flush().unwrap(); // Ensure prompt is displayed before waiting for input
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap(); // Read a line of input
        
        // Parse: Break input into command and arguments
        let mut command = input.split_whitespace(); // Split input on whitespace
        let head = command.next(); // First word is the command
        
        // Join the remaining words with spaces to maintain original spacing
        let tail = command.collect::<Vec<&str>>().join(" "); // Rest is arguments
        
        // Evaluate: Process the command
        match head {
            Some("exit") => std::process::exit(0), // Exit with status code 0
            
            Some("echo") => println!("{}", tail), // Echo just prints its arguments
            
            Some("type") => {
                // For type command, check if the argument exists in our BUILTIN_COMMANDS HashMap
                // This is more maintainable than hardcoding with match or if/else
                if BUILTIN_COMMANDS.contains_key(tail.as_str()) {
                    println!("{} is a shell builtin", tail);
                } else {
                        //  This is where you need to add PATH searching logic
                        match env::var("PATH"){
                            Ok(path) => {
                                let mut found = false;
                                for dir in path.split(":"){
                                    let file_path = Path::new(dir).join(&tail);
                                    if file_path.exists(){
                                        println!("{} is {}", tail,file_path.display());
                                        found = true;
                                        break;
                                    }
                                }
                                if !found{
                                    println!("{}: not found", tail);
                                }

                            },
                            Err(e)=> {
                                println!("{}: not found", tail);
                            }
                        }               

                    
                }
            },
            
            // Print: Output the result
            _ => println!("{}: command not found", input.trim()),
        }
        
        // Loop: Continue to the next iteration
    }
}

// ! This is the alterante and simple approach to the problem
// * This is the solution by codecrafters 
//     #[allow(unused_imports)]
//     use std::io::{self, Write};
// fn main() {
//     let stdin = io::stdin();
//     loop {
//         print!("$ ");
//         io::stdout().flush().unwrap();
//         let mut input = String::new();
//         stdin.read_line(&mut input).unwrap();
//         let mut command = input.split_whitespace();
//         let head = command.next();
//         let tail = command.collect::<Vec<&str>>().join(" ");
//         match head {
//             Some("exit") => std::process::exit(0),
//             Some("echo") => println!("{}", tail),
//             Some("type") => match tail.as_str() {
//                 "echo" | "exit" | "type" => println!("{tail} is a shell builtin"),
//                 _ => println!("{tail}: not found"),
//             },
//             _ => println!("{}: command not found", input.trim()),
//         }
//     }
// }
 
