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
    
    // UTF8 was made to use 1 byte to 4 bytes to store a character, but in some cases it is necessary to use more than 4 bytes to store a character
    // in 2007 the unicode consortium created a extension to use more than 4 bytes to store a character 
    // https://www.unicode.org/charts/PDF/U1F100.pdf
    // but Rust use 4 bytes to store a character "UTF-8" (it is enough to store the most of the characters > 99%)
    // You need to take care with the size of the char (because it can be bigger than 4 bytes)
    
    // There is a way to store a character with more than 4 bytes in Rust?
    //      Yes, you can use a string to store a character with more than 4 bytes ( we will see in the future)

    // UTF-8 use 1 byte to 4 bytes to store a character
    // it is compatible with ASCII (1 byte)

    // scematic of utf-8: (x) = free bit
    // 0xxxxxxx = 1 byte (ASCII)
    // 110xxxxx 10xxxxxx = 2 bytes
    // 1110xxxx 10xxxxxx 10xxxxxx = 3 bytes
    // 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx = 4 bytes

    let japonese_char = '中';
    println!("japonese_char: {0} = bin({1:032b}) = Hex({1:X})", japonese_char, japonese_char as u32); // for store a the value is 0110 0001 in memory
    let emoji_char = '😀';
    println!("emoji_char: {0} = bin({1:032b}) = Hex({1:X})", emoji_char, emoji_char as u32); // for store a the value is 0110 0001 in memory
    const BREAK_LINE_CHAR:char = '\n'; //end line
    println!("BREAK_LINE_CHAR: {0} = bin({1:032b}) = Hex({1:X})", BREAK_LINE_CHAR, BREAK_LINE_CHAR as u32); // for store a the value is 0110 0001 in memory
    
    // you can use unicode to define a char
    // Unicode is a standard that define a code point to each character
    // https://www.unicode.org/charts/

    const UNICODE_CHAR:char = '\u{2764}'; //But for use char in rust you need to use the unicode with 4 bytes or less
    println!("UNICODE_CHAR: {0}  = bin({1:032b}) = Hex({1:X})", UNICODE_CHAR, UNICODE_CHAR as u32); // for store a the value is 0110 0001 in memory

    // The terme "code point" is used to represent a character in unicode (1 code point use 4 bytes)
    // but a symbol can be represented by more than one code point (in this case the character use more than 4 bytes)
    
    // example: the character brazil flag "🇧🇷" use 2 code points (🇧 and 🇷) and 8 bytes to store [240, 159, 135, 167, 240, 159, 135, 167]
    // example: the character "वा" use 2 code points (व and ा) and 5 bytes to store
    // example: the character "中" use 1 code points (中) and 3 bytes to store
    // if try to store this characters in a char, it will not work (because rust use 4 bytes to store a char)
    // see the examples below:
    
    //let problem_char = '🇧🇷'; // Generate this erro: (charater literal may only contain one codepoint)
    // dont worry, you will learn how to store this characters in a string in the future

    let my_string = "🇧🇷"; // this is a string (it is not a char)
    let bytes_vector:[u8;8] = my_string.as_bytes().to_vec().try_into().unwrap(); // convert the string to u8 vector
    let big_unicode_char = u64::from_le_bytes(bytes_vector); // convert the string to u64
    println!("my_string: {0}  = Hex({1:X})", my_string, big_unicode_char); 

    let my_string2 = "वा"; // this is a string (it is not a char)
    let bytes_string2 = my_string2.as_bytes();
    let mut bytes_vector2:[u8;8] = [0;8];
    for i in 0..bytes_string2.len(){
        bytes_vector2[i] = bytes_string2[i];
    }
    let big_unicode_char2 = u64::from_le_bytes(bytes_vector2); // convert the string to u32
    println!("my_string2: {0}  = Hex({1:X})", my_string2, big_unicode_char2);
    
}