// (Option/Some) in rust is a way to use None in a var but with security
use rand::Rng;
fn randon_number() -> Option<i32>{
    //generate a random number
    let mut rng = rand::thread_rng(); 
    let number=rng.gen_range(1..=100);
    if number < 50 {
        Some(number)
    } else {
        None
    }
}
fn sum_optional(a: i32, b: Option<i32>) -> i32 {
    a + b.unwrap_or(0)
}
fn main(){
let my_var:Option<i32> = Some(5);
let my_var2:Option<i32> = None;
let _sum = sum_optional(1, None); // you can use ir like a optional value like a argument
println!("my_var is: {}", my_var.unwrap_or(0));
println!("my_var2 is: {}", my_var2.unwrap_or(0));
let my_var3 = randon_number();
println!("my_var3 is: {:?}", my_var3)

}