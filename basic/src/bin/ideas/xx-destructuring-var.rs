use chrono::prelude::*;
//https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html
fn main(){
    let t = Local::now();
    //there is tuple in rust and is possible use destructuring
    let (year, month, day) = (t.year(), t.month(), t.day());
    println!("Hoje é dia {} do {} de {}", day, month, year);
    
    let (x, y);
    (x,..) = (3,4,5,6,7,78);
    [.., y] = [1,2,5,6,67,7,464,46,46,2];
    println!("x = {}, y = {}", x, y);

}