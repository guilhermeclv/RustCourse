// Typically the main function will look like this:
// fn main() {
//     println!("Hello World!");
// }
// However main is also able to have a return type of Result. If an error occurs within the main function it will return an error code and print a debug representation of the error( Debug trait ).
// The following example shows such a scenario:
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> { //main can return a result type
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}