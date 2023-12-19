

// impl<'a> PartialEq<i32> for &'a T {
//     // ...
// }

/* Adding HRTB to make it work!*/
fn call_on_ref_zero<'a, F>(f: F) where F: Fn(&'a i32) {
    let zero = 0;
    f(&zero);
}

fn main() {
    println!("Success!");
}
