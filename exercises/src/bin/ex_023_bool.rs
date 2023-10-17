// Make println! work
fn main() {
    let _f: bool = false;

    let t = true;
    if !!t { // !!t is the same as t
        println!("Success!");
    }
} 