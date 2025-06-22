// Remove a line in the code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
<<<<<<< HEAD
 
=======
    let mut x = x; 
>>>>>>> f11c22dbc9b53d814171f827ba981efe6b89f975
    x += 3;


    let y = 4;
    // Shadowing
    let y = "I can also be bound to text!"; 

    println!("Success!");
}
//Feito