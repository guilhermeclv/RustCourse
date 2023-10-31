use std::io;
fn main () {
    let number = 3_i32;
    if number < 5 {
        println!("condition if");
    } else if number == 5 {
        println!("condition else if");
    } else {
        println!("condition else");
    }

    if true {
        println!("condition if");
    } else {
        println!("condition else");
    }

    if let (1,x)=(1,number) { // create intern variable when condition is true
        println!("using (if let, X is:{x})");
    } 
    
    //println!("using (if let, X is:{})", x); // error: not found `x` in this scope

    let condition = true;
    // in rust if return a value and is possible use this in a variable (last line in block is return)
    let number2 = if condition { 5 } else { 6 }; // not exist ternary operator in rust but is possible use this
    println!("The value of number2 is: {}", number2);
    
    println!("white a small phrase:");
    let mut text_input = String::new();
    let input = io::stdin();
    input.read_line(&mut text_input).unwrap();

    if text_input.len() < 5 {
        println!("text_input is less than 5 characters long");
    } else if text_input.len() >= 5 && text_input.len() < 10 {
        println!("text_input is longer than 5 characters long but less than 10");
    } else if text_input.len() >= 10 && text_input.len() < 20 {
        println!("text_input is longer than 9 characters long but less than 20");
    } else {
        println!("text_input is longer than 20 characters");
    }
}