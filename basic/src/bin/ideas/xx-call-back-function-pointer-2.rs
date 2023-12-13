fn print_today() -> i32{
    use chrono::prelude::*;
    let t = Local::now();
    let (year, month, day) = (t.year(), t.month(), t.day());
    println!("Hoje é dia {} do {} de {}", day, month, year);
    return 0;
}
fn call_function(f: fn() -> i32) -> i32{
    return f();
}
fn main(){
    let function_pointer = print_today;
    let _:i32 = call_function(function_pointer);
}  