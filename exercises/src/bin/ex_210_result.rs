
// Result<T> is an enum to describe possible errors. It has two variants:

// Ok(T): A value T was found
// Err(e): An error was found with a value e
// In short words, the expected outcome is Ok, while the unexpected outcome is Err.

// FILL in the blanks and FIX the errors
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> __ {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn main() {
    let result = multiply("10", "2");
    assert_eq!(result, __);

    let result = multiply("t", "2");
    assert_eq!(result.__, 8);

    println!("Success!");
}