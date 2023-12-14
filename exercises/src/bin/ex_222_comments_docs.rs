// example to hide lines in documentation

/// ```
/// # fn try_main() -> Result<(), String> {
/// let res = doc_comments::compute::try_div(10, 0)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { 
/// #    try_main().unwrap();
/// #
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    let arg = 5;
    let answer = try_div(arg, 2);

    assert_eq!(Ok(2), answer);
}