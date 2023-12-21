/* Fill in the blank and fix the errors */
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
    let total = v1_iter.sum();

    assert_eq!(total, __);

    println!("{:?}, {:?}",v1, v1_iter);
}