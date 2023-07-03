use std::io;

fn main() {
    let texto: &str = "Hello, world!";
    println!("{}", texto);
    let text1: &str = "Hello,";
    let text2: &str = "world!"; //é um ponteiro para a posição de memória onde está a string
    println!("{} {}", text1, text2);
    let mut name = String::new();
    println!("Digite seu nome: ");
    io::stdin().read_line(&mut name); //iremos resolver isso mais a frente
    println!("Olá, {}", name);
    
}
