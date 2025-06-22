// Fill the blanks and fix the errors
fn main() {
    // Integer addition
    assert!(1_u32 + 2_u32 == 3);

    // Integer subtraction
    assert!(1_i32 - 2_i32 == -1);
    assert!(1_i32 - 2_i32== -1); 
    
    assert!(3 * 50 == 150);

    assert!((9.60 / 3.20) as f32 == 3.0); // error ! make it work

    assert!(24 % 5 == 4);
    // // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}