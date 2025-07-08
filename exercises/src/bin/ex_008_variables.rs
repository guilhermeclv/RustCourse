fn main() {
<<<<<<< HEAD
    let (x, y) = (1, 2);
    //x += 2;
=======
    let (mut x, y) = (1, 2);
    x += 2;
>>>>>>> f11c22dbc9b53d814171f827ba981efe6b89f975

    assert_eq!(x, 1);
    assert_eq!(y, 2);

    println!("Success!");
}
//Feito