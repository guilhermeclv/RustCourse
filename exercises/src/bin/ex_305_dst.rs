/* Make it work with const generics */
// Other way to do the same thing
fn my_function<T: std::marker::Copy, const N: usize>(n:T) -> [T; N] {
    [n; N]
}

fn main() {
    let arr = my_function::<u32, 5>(10);
    let arr2:[u32; 10] = my_function(2);
    println!("{:?}",arr);
    println!("{:?}",arr2);
}