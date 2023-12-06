// To use `from_str` method, you need to introduce this trait into the current scope.
use std::str::FromStr;
fn main() {
    let parsed: i32 = "5".__.unwrap();
    let turbo_parsed = "10".__.unwrap();
    let from_str = __.unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);

    println!("Success!");
}