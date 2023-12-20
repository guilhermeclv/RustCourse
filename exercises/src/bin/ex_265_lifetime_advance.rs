/* Make it work by reordering some code */
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    *ref1 += 1;
    let ref2 = &mut *ref1;
    *ref2 += 2;
    
    println!("{}", data);
}
    