fn main() {
    
    // In these exercises we will learn about print in rust and use only number with exemple, because we don't learn about variables yet

    println!("Hello, world!"); // print a text in the screen YOU NEED USE (") to start and end a text , SEE THE EXEMPLE
    println!("{}",1); // print a number in the screen FOR IS USE {} Insite of ""
    println!("YOU CAN WRITE A TEXT HERE , and put values inside the text using ({})",9); 
    println!("{} {}",1,2); // is possible use more than one {} in the same text to print more than one value
    println!("random text {} 12345 {} blablalba {} {} {}",1,2,3,4,5); // is possible use more than one {} in the same text to print more than one value
    print!("**print don't add a new line in the end of text**"); // print a text in the screen without add a new line in the end of text
    print!("**text in the same line**");
    println!(""); // print a new line
    println!("text rand1 {2}text randX {1} text randY{0} text randZ",1,2,3); // is possible use the index of value to print it in the text
    println!("number in binary {:b}",10); // print a number in binary
    println!("number in octal {:o}",10); // print a number in octal
    println!("number in hexadecimal {:x}",10); // print a number in hexadecimal
    // how to print " inside a text
    println!(" dfd \" dfddf"); // use \ to print " inside a text (\) is a scape character in rust

    // this is the basic about print in rust, we are going to learn more about it later, per hour just remember that you need use {} to print a value in the screen
    // and you can use {} to print more than one value in the same text

}