// FILL in the blank and FIX errors
fn main() {
    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");    // Explicar novamente relação entre quantidade de bit

    let slice2 = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format //O caractere '世' começa no byte 7 e ocupa 3 bytes.
    
    // Iterate through all chars in s
    for (i, c) in s.__ {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("Success!");
}