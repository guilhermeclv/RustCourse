/* Refactoring the following code using iterators */
fn main() {
    let arr = [0; 10];
    for i in &arr { // you can use arr.into_iter() or arr.iter()
        println!("{}",i);
    }
}
