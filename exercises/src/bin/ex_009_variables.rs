fn main() {
    let (x, y, z);
    (x,..) = (3, 4);
    [z, .., y] = [2, 6, 6];
    // Fill the blank to make the code work
    assert_eq!([x,y,z], [3,6,2]);

    println!("Success!");
}
//Feito