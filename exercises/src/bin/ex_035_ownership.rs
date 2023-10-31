fn main() {
    // Use as many approaches as you can to make it work
    let x = String::from("hello, world");
    let y = x.clone();
    println!("{},{}",x,y);

    let x = String::from("hello, world");
    let y = x.to_owned();
    println!("{},{}",x,y);

    let x = &String::from("hello, world");
    let y = x;
    println!("{},{}",x,y);
}