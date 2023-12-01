fn main(){
    // Bollean 
    // This type can store true or false values
    // The type is bool
    // The size is 1 byte
    let state_true: bool = true;
    let state_as_u8_true: u32 = state_true as u32; // cast to u8 to print the value in binary
    println!("state_true (bollean): {} {:08b}", state_true, state_as_u8_true); // for store true the value is 0000 0001 in memory
    let state_false: bool = false;
    let state_as_u8_false = state_false as u32; // cast to u8 to print the value in binary
    println!("state_false (bollean): {} {:08b}", state_false, state_as_u8_false); // for store false the value is 0000 0000 in memory
    // why the size is 1 byte?
    // because the smallest addressable unit in memory is 1 byte, is not possible to store a value smaller than 1 byte
  
    // Char
    // This type can store a single character
    // The type is char
    // The size is 4 bytes
    // caracter is different of string in rust 
    // (USE "" for string and '' for char)
    let char_a: char = 'A';
    println!("char_a: {0} = bin({1:032b}) = Hex({1:X})", char_a, char_a as u32); // for store a the value is 0110 0001 in memory
    
    // ALL WRONG need to be fixed
    //A "character" can take more than 4 bytes because it is made of more than one code point. For instance a national flag character takes 8 bytes since it is "constructed from a pair of Unicode scalar values" both from outside the BMP.

    // why the size is 4 bytes?
    // because Rust use utf-8 to store a char, and utf-8 you can use multiple bytes to store a single character
    // by default Rust use 4 bytes to store a char (it is enough to store the most of the characters > 99%)
    // In some cases (very specific cases) a caracter can't be store for a char
    
    //utf-8 use (scape) characters to store some special characters
    
    // it isn't possible to store this caracter in a char, but it is possible to store in a string (because string use a vector of bytes)
    
    // let problem_char = 'वा'; // this caracter use 5 bytes [224, 164, 181, 224, 164, 190] and rust use 4 bytes to store a char
    
    // some times is important take care with the size of the char (because it can be bigger than 4 bytes)
    
    let japonese_char = '中';
    println!("japonese_char: {0} = bin({1:032b}) = Hex({1:X})", japonese_char, japonese_char as u32); // for store a the value is 0110 0001 in memory
    let emoji_char = '😀';
    println!("emoji_char: {0} = bin({1:032b}) = Hex({1:X})", emoji_char, emoji_char as u32); // for store a the value is 0110 0001 in memory
    const BREAK_LINE_CHAR:char = '\n'; //end line
    println!("BREAK_LINE_CHAR: {0} = bin({1:032b}) = Hex({1:X})", BREAK_LINE_CHAR, BREAK_LINE_CHAR as u32); // for store a the value is 0110 0001 in memory
    
    
    
    


    // you can use unicode to define a char
    const UNICODE_CHAR:char = '\u{2764}'; // 
    println!("UNICODE_CHAR: {0}  = bin({1:032b}) = Hex({1:X})", UNICODE_CHAR, UNICODE_CHAR as u32); // for store a the value is 0110 0001 in memory

}