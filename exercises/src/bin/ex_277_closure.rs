//examples
fn main() {
    let s = String::new();

    let update_string = move || println!("{}",s);

    exec(update_string);
}

fn exec<F: FnOnce()>(f: F)  {
    f()
}


// fn main() {
//     let s = String::new();

//     let update_string = move || println!("{}",s);

//     exec(update_string);
// }

// fn exec<F: Fn()>(f: F)  {
//     f()
// }