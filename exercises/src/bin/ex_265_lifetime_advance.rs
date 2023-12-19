/* Make it work by reordering some code */
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    *ref1 += 1;
    *ref2 += 2;

    println!("{}", data);
}