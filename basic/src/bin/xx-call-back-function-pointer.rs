fn print_today(){
    use chrono::prelude::*;
    let t = Local::now();
    let (year, month, day) = (t.year(), t.month(), t.day());
    println!("Hoje é dia {} do {} de {}", day, month, year);
}
fn call_function(f: fn()){
    return f();
}
fn main(){                                                                    
    let function_pointer: fn() = print_today;
    call_function(function_pointer);
}