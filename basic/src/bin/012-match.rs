fn plus2(number: i32) -> i32{
    number+2
}
fn main(){
    // match is a control flow operator like if but match is more powerful
    let number = 3_i32;
    match number {
        1 => println!("number is 1"),
        2 => println!("number is 2"),
        3 => println!("number is 3"),
        4 => println!("number is 4"),
        5 => println!("number is 5"),
        _ => println!("number is not 1,2,3,4 or 5"),
    }
    //is possible use match for destructuring like if let
    let number2 = 5_i32;
    let my_tuple = (1,2,number2);
    match my_tuple {
        (1,2,3) => println!("my_tuple is (1,2,3)"),
        (1,2,free_var_3) => println!("my_tuple is (1,2,{free_var_3})"),
        (1,..,free_var_3) => println!("my_tuple is (1,,{free_var_3})"),
        (free_var_1,free_var_2,free_var_3) => println!("my_tuple is ({free_var_1},{free_var_2},{free_var_3})"),
    }

    //is possible use match with a array
    let my_array = [1,2,3,4,5];
    let result_array = match my_array {
        [first,..,last] if first > last => first,
        [..,last] => last,
    };
    println!("result_array is {}",result_array);


    // using @ for store a value in a variable
    // and use this variable in the match
    // this is useful for use the same value in multiple matchs
    let number3 = 5_i32;
    let _result = (1..=100).contains(&number3);
    
    match number3 {
        free_var_1 @ 1..=5 => println!("number3 is {} and is between 1 and 5",free_var_1),
        free_var_2 @ 6..=10 => println!("number3 is {} and is between 6 and 10",free_var_2),
        free_var_1 @ 11..=100 => println!("number3 is {} and is between 6 and 10",free_var_1),
        _ => println!("number3 is not between 1 and 10"),
    }

    match plus2(23_i32){
        0 => println!("plus2(23) is 0"),
        // free_var_1 @ free_var_1 < 0 => println!("plus2(23) is {free_var_1} less than 0"), not work
        free_var_1 if free_var_1 < 0 => println!("plus2(23) is {free_var_1} less than 0"), // right way
        free_var_2 if free_var_2 > 0 => println!("plus2(23) is {free_var_2} greater than 0"),
        _ => println!(""),
    }

    // match ca return a value
    let number4 = 5_i32;
    let result = match number4 {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        free_var_1 @ 5..=100 => free_var_1,
        _ => 0,
    };
    println!("result is {}",result);

    //match is an 
    let number5 = 5_i32;
 


}