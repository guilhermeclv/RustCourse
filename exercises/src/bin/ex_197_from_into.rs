// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// FILL in the blanks
fn main() {
    let num = Number::from(30);
    assert_eq!(num.value, 30);

    let num: Number = 30.into();
    assert_eq!(num.value, 30);

    println!("Success!");
}

// // Nice version
// #[derive(Debug)]
// struct Number<T> {
//     value: T,
// }

// impl<T> From<T> for Number<T> 
// where T: std::ops::Add<Output = T>
// {
//     fn from(item: T) -> Self {
//         Number { value: item }
//     }
// }

// fn main() {
//     let num = Number::from(30);
//     assert_eq!(num.value, 30);

//     let num: Number<i32> = 30.into();
//     assert_eq!(num.value, 30);

//     println!("Success!");
// }