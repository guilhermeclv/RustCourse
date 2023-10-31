
// Remove something to make it work
// Don't remove a whole line !
fn main() {
    let mut s = String::from("hello");
    //&mut is possible has only one reference
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success!");
}