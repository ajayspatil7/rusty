fn main(){
    // BIG ASS NUMBERS: Basically int ~ i types (int16, int32, int64) -> (i16, i32, i64)
    // Most common is i32 for any number which is not SO huge. 
    // Same for float: (float16, float32, float64) -> (f16, f32, f64)
    // f16 is outdated and not prefered you can use it but upto you. Many hardware won't even support this poor guy! 
    // So use, f32 bruh!

    // A SIGNED INT var with different type of size has a limit, Range(-(2^n), (2^n) - 1)
    // For ex : i8 has limit -(2^7) to (2^7) - 1
    // In numbers its :  -(2^7) = -128 and (2^7) - 1 = 127
    // So the range of i8 is from -128 to 127
    // Same goes for other SIGNED int as well
    let _extrasmallsignedint: i8 = 127;
    let _smallsignedint: i16 = 3276;
    let _mediumsignedint: i32 = 2147483647;
    let _largesignedint: i64 = 9223372036854775807;

    // An UNSIGNED INT with different type of size also has a limit, Range(0, 2^N-1)
    // Example : u8 has range (0, 255) How ? 2^8 = 256 - 1 = 255 simple dude!
    // If you've time do it for other unsigned int as well
    let _extrasmallunsignedint: u8 = 255;
    let _smallunsignedint: u16 = 65535;

    // FLOATS are a bit different story,
    // float is generally made of 2 components : (Mantissa and exponent) even sign is included in some cases
    // mantisa generally represents the precision of a number : higher the mantisa higher the precision
    // the value of PI = 3.14 is precise but more precise would be 3.1428571429
    // In rust there are primarily 2 types of float :fp32 and :fp64
    // :fp32 is a single precision float
    // :fp64 is a double precision float 




    // CONSTANTS
    // Value and Type can never be changed. I MEAN NEVER!
    // Naming convention : const ALL_CAPS!
    const _PI = 3.14;
    
}