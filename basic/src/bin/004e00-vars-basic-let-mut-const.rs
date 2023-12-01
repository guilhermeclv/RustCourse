fn main(){
    // exercise about let, mut and const
    // exercise 1
    // when you need to use a value that you define while you are writing the code, what type you use?
    // a) let
    // b) let mut
    // c) const

    // exercise 2
    // if you need to store a value that you read from a file once, what type you use?
    // a) let
    // b) let mut
    // c) const

    // exercise 3
    // if you need store a value that will alter during the execution of the program, what type you use?
    // a) let
    // b) let mut
    // c) const

    // exercise 4
    // fix the code below to make it compile
    let x = 10;
    let mut y = 20;
    const Z: i32 = 30;
    println!("x: {}, y: {}, z: {}", x, y, Z);
    x=Z;
    y=x;

}