fn main() {
<<<<<<< HEAD
    let x=define_x();
=======
    let x:&str="hello";
>>>>>>> f11c22dbc9b53d814171f827ba981efe6b89f975
    println!("{}, world", x); 
}

fn define_x() -> &'static str {
    let x:&str = "hello";
    x
}
//Feito