// Only modify `assert_eq!` to make the `println!` work(print `42` in terminal)
fn main() {
    let x: i32 = 12;
    {
<<<<<<< HEAD
        let x = 12;
        assert_eq!(x, 12);
=======
        let x = 5;
        assert_eq!(x, 5);
>>>>>>> f11c22dbc9b53d814171f827ba981efe6b89f975
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // Prints "42".
}

//Feito