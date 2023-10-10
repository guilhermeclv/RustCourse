//destructuring with .. operator
fn main() {
    let (x, y);
    (x,..) = (3, 4); // you can use .. to ignore some values in a tuple
    [.., y] = [1, 2]; // you can use .. to ignore some values in an array
    // Fill the blank to make the code work
    assert_eq!([x,y], [3,2]);

    println!("Success!");
}