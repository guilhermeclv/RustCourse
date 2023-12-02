
// FIX the error and IMPLEMENT the code
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 { 
        println!("{:?}",  v.get(i)) // use get() method to get to avoid panic
    }

    for i in 0..5 {
        v.push(i + 2);
    }
    let drained_value:Vec<i32> = v.drain(0..3).collect();
    println!("drained_value: {:?}",drained_value);
    println!("new values: {:?}", v);
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}
