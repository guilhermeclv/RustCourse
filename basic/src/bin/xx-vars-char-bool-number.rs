use std::mem;
fn main(){
    // rust use utf8 as default encoding and unicode, because it is necessary use 4 bytes (32 bits) for a simple char
    // CHAR
    let _simple_var:i32; //declare defore use
    _simple_var = 10; //assign firt time (value not mutable)
    //_simple_var = 20; //assign second time not work (value not mutable)
    let simple_char: char = 'I';
    let char_unicode_heart: char = '\u{2764}';
    let char_unicode_heart_2: char = '\u{1F496}';
    let size_in_bytes = mem::size_of::<char>();
    println!("{} {}  + {} = Rust", simple_char, char_unicode_heart, char_unicode_heart_2);
    println!("a char size is: {} bytes ({} bits)\n", size_in_bytes,size_in_bytes*8);
    
    // BOOLEAN
    let boolean_true: bool = true;
    let boolean_false: bool = false;
    let size_in_bytes = mem::size_of::<bool>();
    println!("{} != {}", boolean_true, boolean_false);
    println!("A bool size is: {} bytes ({} bits)", size_in_bytes,size_in_bytes*8);
    assert!(boolean_true == !boolean_false); // ! is a logical operator, not a bitwise operator
    if boolean_true { // is possible use boolean as condition
        println!("boolean_true is true");
    }
    if !boolean_false { // is possible use boolean as condition
        println!("boolean_false is false");
    }
    println!("boolean operations (true AND false = {}) - (true OR false = {}) - (NOT true = {})", boolean_true&&boolean_false, boolean_true||boolean_false, !boolean_true);
    println!("\n");

    // NUMBERS
    let number_i8: i8 = 127; // 8 bits max value
    let number_i16: i16 = 32767; // 16 bits max value
    let number_i32: i32 = 2147483647; // 32 bits max value
    let number_i64: i64 = 9223372036854775807; // 64 bits max value
    let number_i128: i128 = 170141183460469231731687303715884105727; // 128 bits max value

    let number_u8: u8 = 255; // 8 bits max value
    let number_u16: u16 = 65535; // 16 bits max value
    let number_u32: u32 = 4294967295; // 32 bits max value
    let number_u64: u64 = 18446744073709551615; // 64 bits max value
    let number_u128: u128 = 340282366920938463463374607431768211455; // 128 bits max value

    let number_f32: f32 = 3.40282347e+38_f32; // 32 bits max value
    let number_f64: f64 = 1.7976931348623157e+308_f64; // 64 bits max value
    let _defined_number_w_type = 123_f32; // you can use (_type) to define a number with a type
    let _defined_number_big = 1_000_000; // you can use (_) to define a number with a big number
    let _defined_number_hex = 0xff; // you can use (0x) to define a number with a hex number
    let _defined_number_bin = 0b1111_0000_u8; // you can use (0b) to define a number with a binary number

    
    println!("max number_i8: {}", number_i8);
    println!("max number_i16: {}", number_i16);
    println!("max number_i32: {}", number_i32);
    println!("max number_i64: {}", number_i64);
    println!("max number_i128: {}", number_i128);
    println!("max number_u8: {}", number_u8);
    println!("max number_u16: {}", number_u16);
    println!("max number_u32: {}", number_u32);
    println!("max number_u64: {}", number_u64);
    println!("max number_u128: {}", number_u128);
    println!("max number_f32: {}", number_f32);
    println!("max number_f64: {}", number_f64);
    
    if number_i8>0 { // number_i8 not work condition the correct is number_i8>0
        println!("numbers not work like a boolean");
    }

    println!("I can verify the max value of a number using (i8::max_value(): {}) or deprecated way (std::i8::MAX: {})", std::i8::MAX, i8::max_value());
    println!("I can verify the min value of a number using (i8::min_value(): {}) or deprecated way (std::i8::MIN: {})", std::i8::MIN, i8::min_value());

    let number_1: i8 = -5;
    println!("number 0b{:b} is {} in decimal", number_1, number_1);
    println!("this number has {} zeros and {} ones", number_1.count_zeros(), number_1.count_ones());
    println!("this number has {} leading zeros", number_1.leading_zeros());
    println!("this number has {} trailing zeros", number_1.trailing_zeros());
    println!("using rotate_left(2) the number is 0b{:b}", number_1.rotate_left(2)); //like swift left
    println!("using rotate_right(2) the number is 0b{:b} = {}", number_1.rotate_right(2),number_1.rotate_right(2));
    
    let value: u32 = 0x1234ABCD;
    let swapped_value = value.swap_bytes();

    println!("original value (HEX): {:X}", value);
    println!("swapped value (HEX): {:X}", swapped_value);

    let float_exemple:f32  = -100.5;
    println!("float_exemple: {}", float_exemple);
    println!("float_exemple.round(): {}", float_exemple.round());
    println!("float_exemple.floor(): {}", float_exemple.floor());
    println!("float_exemple.ceil(): {}", float_exemple.ceil());
    println!("float_exemple.trunc(): {}", float_exemple.trunc());
    println!("float_exemple.fract(): {}", float_exemple.fract());
    println!("float_exemple.abs(): {}", float_exemple.abs());
    println!("float_exemple.signum(): {}", float_exemple.signum());
    println!("is finite: {}", float_exemple.is_finite());
    println!("is infinite: {}", float_exemple.is_infinite());
    println!("is nan: {}", float_exemple.is_nan());

    //consts Only values in compile time -> YOU CANT USE VARS IN CONSTS
    const PI: f32 = 3.14159265358979323846264338327950288_f32;
    const E: f32 = 2.71828182845904523536028747135266249_f32;
    const SECONDS_IN_DAY: u32 = 60 * 60 * 24;
    println!("PI: {}", PI);
    println!("E: {}", E);
    println!("SECONDS_IN_DAY: {}", SECONDS_IN_DAY);

}