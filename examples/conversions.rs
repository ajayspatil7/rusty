use std::io;

fn main() {
    // Arithematic operations in Rust
    // in rust it's very strict to use the same type
    // operands to perform calculation.
    // It should be same type (int + int) or (float + float) and
    // It should be same size (i16 + i16) or (f32 + f32) ONLY
    // And one more important thing to look when using specific sized
    // datatype is OVERFLOW. Make sure that bits doesn't overflow.

    let x: i16 = 10;
    let y: i16 = 20;
    let z: i16 = x + y;
    println!("Addition of {} and {} is = {}", x, y, z);
    // Same goes for other basic operations also + - * / %.

    // Typecasting and Conversion
    let _a = 10;
    // By default the compiler assigns any int as i32
    let _b = 3.14;
    // and for float it's f64
    // we can typecast the var from default to something like this,
    let _c = 3.14_f32;
    // now we changed it to f32 from f64 same goes for int also
    // we can even use the 'as' keyword to typecast the type of a var
    let _d = 127 as i8;
    // this is one of the ways to typecast in rust aka explicit typecasting

    // Converting str to int from user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");

    print!("User input as String : {}", input);

    // okay so, by default the input is str as we know
    // let's make it to take the int input and return int
    // .trim() is to remove the escape sequence eg '/n' '/r' etc.
    // .parse() to parse the str to int
    // .unwrap() to unwrap the int from .parse() to match int type which the variable is of.
    let int_input: i64 = input.trim().parse().unwrap();
    println!("User input as Integer : {}", int_input);
    


}
