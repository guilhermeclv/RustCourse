fn main() {
    let x: i32 = 10;
    let y:i32 = 2;
    {
        let y: i32 = 5;
        let x:i32 = 2;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}