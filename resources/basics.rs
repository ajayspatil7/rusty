
fn main() {
    
    // TO BUILD PROJ: cargo build && .target/debug/main OR
    // TO RUN PROJ: cargo run
    // TO FORMAT FILE: rustfmt src/main.rs
    // TO COMPILE .rs: rustc src/main.rs
    // TO RUN : ./main
    
    // Hello world
    println!("Hello, world!");

    // VARIABLES
    // Variables : They are immutable, can't change the value of a variable
    // Implicit deceleration 
    let x = 10;

    // Explicit deceleration
    let y: i32 = 20;

    // Formatted variables printing
    println!("The value of x is: {} and value of y is: {}", x, y);

    // Making a mutable variable in Rust
    let mut age = 21;
    println!("The value of age is: {}", age);

    // Changing the values of mutable variables
    age = 25;
    println!("The value of age after muting is: {}", age);

    // Another way to change the values of a variable without using 'let mut' keyword
    let ln = 10;
    println!("The value of ln is: {}", ln);

    let ln = 20;
    println!("The value of ln after changing the value without 'mut' keyword is: {}", ln);

    // Another way to name shadow using scope in Rust
    // Scope is just another interior layer inside a main function
    
    {
        let age = 23;
        println!("Age inside the scope is : {}", age);
    }

    println!("Age outside the scope is : {}", age);
    
    // Remember: you can use the variable inside the scope from the exterior of the scope
    // Example: declare a var in parent scope (main) and perform operations on that variable inside a scope
    // And it works like magic! Even if its a different type inside the scope.

    let outer_var: i16 = 100;

    {
        let modified_outer_var = outer_var - 50;
        println!("Modified value of outer var inside the scope is : {}", modified_outer_var);
    }

    println!("Actual Outer Var value: {}", outer_var);

}
