use time::PrimitiveDateTime;
use time::macros::{date, format_description, time};
fn main (){
    let b = PrimitiveDateTime::new(date!(2019-01-01), time!(0:00));
    println!("b: {:?}", b);
}