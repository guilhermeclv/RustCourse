#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);


fn main() {    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
}
