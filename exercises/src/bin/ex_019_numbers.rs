// Fill the blanks
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range{ start: 1, end: 5 }); // it return a Range
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); //it return a RangeInclusive
    assert_eq!(RangeInclusive::new(1, 5).contains(&5), true); // it return a bool
    assert_eq!(RangeInclusive::new(1, 5).collect::<Vec<i32>>(),[1,2,3,4,5]); // it return a bool

    println!("Success!");
}