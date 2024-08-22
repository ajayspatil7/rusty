fn main(){

    // DATATYPES: PRIMITIVE DATATYPES
    // Two types of Primitive data types : Scalar and Compound
    // Scalar: int, float, bool, cha
    // Compound: array, tuple

    // SCALAR
    // INT
    // SIGNED INT : i8, i16, i32, i64, i128 (can have negative and positive nums)
    let _x: i8 = 8;
    let _y: i16 = -122;

    // UNSIGNED INT : u8, u,16, u32, u64, u128 (can not have negative nums)
    let _z: u8 = 10;
    let _xz: u32 = 1000;
    
    // FLOATS 
    // Two types float 32 and float 64 both are by default any float is f64
    // they are also single precision and double precision respectively
    let _xfloat: f32 = 3.14;
    let _yfloat: f64 = 9.08;

    // BOOLEAN (true or false)
    
    let _isearthflat: bool = false;
    let _areflateartherstupid: bool = true;

    // CHARACTER / CHAR
    // The final datatype in scalar type
    // anything inside a 'single quote' is a char

    let _middlename: char = 'S';

    // COMPOUND TYPE
    // Tuple: Fixed len and immutable (no more addition after initialisation) and can be heterogeneus
    // Tuples can be made mutable using the 'mut' keyword if needed.
    
    // Implicit tuple declaration
    let _middlenameageisalive = ('S', 21, true);

    // Explicit tuple declaration
    let _threedimensions: (f32, f32, f32) = (10.5, 5.25, 23.5);

    // Acessing tuple elements by idx
    // println!("Element 1: {} Element 2: {} Element 3: {}", _threedimensions.0, _threedimensions.1, _threedimensions.2);

    // Mutable tuple
    let mut cgpasgpa: (f32, f32) = (8.70, 8.90);
    cgpasgpa.0 = 9.1;
    cgpasgpa.1 = 9.8;

    // print!("Tuple elements after changing the values Element1: {} Element2: {}", cgpasgpa.0, cgpasgpa.1);

    // Array: Can be of varying len and are immutable in Rust and should be homogeneus only (same datatype elements)
    // Array can be made mutable using the 'mut' keyword if needed

    // Implicit array declaration 
    // Syntax -> let name: [type;size] = [ele1, ele2, eleN]
    let nums: [i32;5] = [10, 20, 30, 40, 50];

    // Array's can be indexed just like any other language using arr[idx]
    println!("Element1: {} Element5: {}", nums[0], nums[4]);


}