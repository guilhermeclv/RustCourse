
// FIX the error and IMPLEMENT the code
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..3 {
        println!("{:?}", v[i]) //porque o V está sublinhado? // resolução for está alem do 3
    }

    for i in 0..5 {
       v.push(i+2);
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}
