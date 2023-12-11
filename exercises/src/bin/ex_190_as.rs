// FIX the errors and FILL in the blank
// DON'T remove any code
fn main() {
    let decimal = 97.123_f32;

    let integer: u8 = decimal as u8;

    let c1: char = decimal as u8 as char;
    let c2 = (integer+1) as char;
    println!("c1 = {}, c2 = {}", c1, c2);
    assert_eq!((integer+1), 'b' as u8);

    println!("Success!");
}