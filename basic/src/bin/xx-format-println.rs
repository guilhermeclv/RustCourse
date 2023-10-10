fn main(){
    let format_string_0 = format!("Hello, world! {}",5_u32);
    let format_string_1 = format!("Hello, world! {0} {1} {0}",5_u32,6_u32); 
    let format_string_2 = format!("Hello, world! {number1} {number2} {number1}",number1=5_u32,number2=6_u32); 
    let format_string_3 = format!("Hello, world! {number1} {number2} {number1}",number2=5_u32,number1=6_u32); 
    let format_string_4 = format!("Hello, world! {0} {number2} {0}",5_u32,number2=6_u32); 
    let format_string_5 = format!("Hello, world! {:04}",5_u32); //complete with zeros {:x} x digits
    let generic_string = "Hello, world!";
    let format_string_6 = format!("{generic_string}");
    
    println!("{}",format_string_0);
    println!("{}",format_string_1);
    println!("{}",format_string_2);
    println!("{}",format_string_3);
    println!("{}",format_string_4);
    println!("{}",format_string_5);
    println!("{}",format_string_6);

    println!("Hello {:5}!", "x");
    println!("Hello {:1$}!", "x", 5);
    println!("Hello {1:0$}!", 5, "x");
    println!("Hello {:width$}!", "x", width = 5);
    let width = 5;
    println!("Hello {:width$}!", "x");

    assert_eq!(format!("Hello {:<5}!", "x"),  "Hello x    !");
    assert_eq!(format!("Hello {:-<5}!", "x"), "Hello x----!");
    assert_eq!(format!("Hello {:^5}!", "x"),  "Hello   x  !");
    assert_eq!(format!("Hello {:>5}!", "x"),  "Hello     x!");


}