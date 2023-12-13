//exemples of formating
fn main() {
    // Exponent
    println!("{:2e}", 1000000000); // => 1e9
    println!("{:2E}", 1000000000); // => 1E9

    // Pointer address
    let v= vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); // => 0x600002324050

    // Escape
    println!("Hello {{}}"); // => Hello {}
}
