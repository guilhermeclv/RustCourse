fn main() {
    let s1 = "hello";
    /* Fill in the blank */
    let s = format!("{s1}, world!");
    println!("{}", s);
    assert_eq!(s, "hello, world!");
}
