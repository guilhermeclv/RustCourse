
// in lib.rs
// use cargo doc --open to generate documentation

/// Add one to the given value and return the value
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let arg = 5;
    let answer = add_one(arg);

    assert_eq!(6, answer);
}