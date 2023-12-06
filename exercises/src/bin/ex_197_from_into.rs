// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    // IMPLEMENT `from` method
}

// FILL in the blanks
fn main() {
    let num = __(30);
    assert_eq!(num.value, 30);

    let num: Number = __;
    assert_eq!(num.value, 30);

    println!("Success!");
}