//this exercise is about borrowing is very important in Rust
fn main() {
    let x = 5;
    // Fill the blank
    let p = &x;
 
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
    println!("the memory address of x is {:p}", &x);
 }