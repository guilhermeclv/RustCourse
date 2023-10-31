// Don't use clone ,use copy instead
fn main() {
    let mut x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
    x.0 = 99;
    println!("{:?}, {:?}", x, y);
    let mut xx:i64 =123;
    let yy = &xx;
    //xx = 456; it is not allowed
    print!( "{},{}",xx,yy)
}