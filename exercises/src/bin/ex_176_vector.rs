// FILL in the blank
fn main() {
    let mut v1 = Vec::from([1, 2, 4]);
    let j =v1.get(0).unwrap_or(&5);
    let j2 = match v1.get(0) {
        Some(var)=>var,
        None=>&5, 
    };
    v1.insert(0, 5);
    v1.remove(0);
    v1.push(3);
    
    let mut v2 = Vec::new();
    v2.extend([1, 2, 4, 3]);

    assert_eq!(v1, v2);

    println!("Success!");
}
