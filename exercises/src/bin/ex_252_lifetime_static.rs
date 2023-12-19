/* Fill in the blank in two ways */
fn main() {
    __;
    need_static(v);

    println!("Success!")
}

fn need_static(r : &'static str) {
    assert_eq!(r, "hello");
}