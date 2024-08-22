
use std::io;

fn main(){
    // Handling user input from console
    // By default rust doesn't have input() just like python
    // We use crates :: aka libraries/packages and import it in our file to handle input.
    // The console input is stored in "std::io" crate we import it using "use" keyword
    
    // 1. Initialise a mutable var to store the user input
    let mut inputs: String = String::new();
    // 2. From the crate, use io's stdin().read_line() to read the line
    // 3. Reference the read_line() to where the input needs to be stored
    // 4. For safety purpose, use .expect() to handle any potential errors
    io::stdin().read_line(&mut inputs).expect("Failed to read");
    println!("user input: {}", inputs);
}