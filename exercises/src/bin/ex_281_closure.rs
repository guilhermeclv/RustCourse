/* Fill in the blank and fix the error*/
fn factory(x:i32) -> Box<dyn Fn(i32)->i32> { // is necessary to use dyn and the clousure is defined in execution time

    let num = 5;

    if x > 1{
        Box::new(move |x| x + num - 1)
    } else {
        Box::new(move |x| x + num +1)
    }
}
fn main() {
    let f = factory(1);
    println!("f: {}", f(0));
    println!("f: {}", f(2));
}