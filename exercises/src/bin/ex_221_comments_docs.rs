// in src/compute.rs

/// # Another Topic
///
/// blablba
/// ## Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doc_comments::compute::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

fn main(){
    let arg = 5;
    let answer = div(arg, 2);

    assert_eq!(2, answer);
}