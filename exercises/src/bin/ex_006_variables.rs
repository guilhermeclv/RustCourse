// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    println!("x1 = {}", x);
    x = 7;
    // Shadowing and re-binding
    let x = x+3; 
    println!("x2 = {}", x);
    let y = 4;
    println!("y1 = {}", y);
    // Shadowing
    let y = "I can also be bound to text!"; 
    println!("y2 = {}", y);
    println!("Success!");
}