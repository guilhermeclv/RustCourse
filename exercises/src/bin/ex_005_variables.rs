// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 5;
    {
        let x = 12; // shadowing (not mut)
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42_u64; // shadowing (not mut)
    println!("{}", x); // Prints "42".
}
// this concept is called shadowing, it is important to reusing variable names without mutating them