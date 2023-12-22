
fn main() {
    let mut s = String::new();

    let mut update_string = |str| s.push_str(str);

    exec2(&mut update_string);
    exec(update_string);
    
    println!("{:?}",s);
    // println!("{:?}",exec2(update_string));
}

/* Fill in the blank */
fn exec<'a, F: FnMut(&'a str)->()>(mut f: F)  {
    f("hello");
    f(" world");
}

fn exec2<'a, F: FnOnce(&'a str)->()>(f: F)  {
    f("first word ");
}

// fn exec2<F: FnOnce(&str)->()>(f: F)  {
//     f("hello2")
// }
