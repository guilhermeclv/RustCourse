/* Make it work */
use std::collections::HashMap;
fn main() {
    let names = [("sunface",18), ("sunfei",18)];
    let folks: HashMap<_, _> = names.into_iter().collect();

    println!("{:?}",folks);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2:Vec<i32> = v1.into_iter().collect();
    //let v2:Vec<i32> = v1.iter().map(|x| *x).collect();

    assert_eq!(v2, vec![1, 2, 3]);
}
