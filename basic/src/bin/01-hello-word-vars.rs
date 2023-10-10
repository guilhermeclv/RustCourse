use std::io;

fn main() {
    let _no_useful_number: i32 = 10; //immutable and save in stack
    let texto: &str = "Hello, world!";
    println!("{}", texto);
    let text1: &str = "Hello,";
    let text2: &str = "world!"; //it is a pointer to a string literal (steak)
    println!("{} {}", text1, text2);
    let mut name = String::new();
    println!("Digite seu nome: ");
    let input = io::stdin();
    input.read_line(&mut name).unwrap(); // read a line from the standard input to the string we created
    println!("Olá, {}", name); 
}
