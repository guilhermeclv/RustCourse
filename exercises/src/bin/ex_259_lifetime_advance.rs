/* Annotate struct with lifetime:
1. `r` and `s` must have different lifetimes
2. lifetime of `s` is bigger than that of 'r'
*/
struct DoubleRef<T> {
    r: &T,
    s: &T
}
fn main() {
    println!("Success!")
}
