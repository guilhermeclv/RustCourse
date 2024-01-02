/* Make it work in two ways */
use std::fmt::Display;
//fn foobar(thing: Display) {}
fn foobar_1(thing: &dyn Display) {
    println!("{}", thing);
}

fn foobar_2(thing: Box<dyn Display>) {
    println!("{}", thing);
}    

fn main() {
    foobar_1(&"Hello there!");
    foobar_2(Box::new("Hello there!"));
}

