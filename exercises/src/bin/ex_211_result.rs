// ? is almost exactly equivalent to unwrap, but ? returns instead of panic on Err
use std::num::ParseIntError;

// IMPLEMENT multiply with ?
// DON'T use unwrap here
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    //Ok(n1_str.parse::<i32>()? * n2_str.parse::<i32>()?)
    let n1 = n1_str.parse::<i32>()?;
    let n2 = n2_str.parse::<i32>()?;
    Ok(n1 * n2)

    // // intead of using ? we can use match
    // match (n1_str.parse::<i32>(), n2_str.parse::<i32>()) {
    //     (Ok(n1), Ok(n2)) => Ok(n1 * n2),
    //     (Err(e), _) => Err(e),
    //     (_, Err(e)) => Err(e),
    // }    
}
fn main() {
    assert_eq!(multiply("3", "4").unwrap(), 12);
    println!("Success!");
}
