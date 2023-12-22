/* Make it work by changing the trait bound, in two ways*/
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool + Copy, // add the Copy trait because the closure is used twice , so it will be copied by the compiler
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    let closure = |z| z == x.len();
    fn_once(closure);
}