

// FUNCTIONS

fn main() {

    // The function we are in rn is a main function
    // it is called by the compiler directly, we don't need to call it
    // Let's call a user defined function
    println!("Hello, world from the main() function!");
    hello_world();

    // Calling a user defined function with parameters
    let result: i32 = sum(10, 20);
    println!("The result of sum() function is: {}", result);

}

// We can create our own function  using 'fn' keyword and call it
// it is adviced to follow snake_case in rust for naming a function :)
// this is called a function without any parameters
fn hello_world(){
    println!("Hello, world from a hello_world() function!");
}

// Statements vs Expression 
// Statements are the one which doesn't return any value eg: let x: i32 = 100;
// Expressions are the one which can return any value eg: sum(10, 20) returns 30

// Let's create a function with parameters(parm1, parm2) which returns a value
// Note: in rust no need to explicitly mention 'return' if the expression is last line in function body
// by default the expressions in rust returns the value but, it's a good practise to return it.
fn sum(a: i32, b: i32) -> i32{
    return a + b;
}


