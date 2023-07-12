use chrono::prelude::*;

fn main(){
    let t = Local::now();
    //there is tuple in rust and is possible use destructuring
    let (year, month, day) = (t.year(), t.month(), t.day());
    println!("Hoje é dia {} do {} de {}", day, month, year);
}