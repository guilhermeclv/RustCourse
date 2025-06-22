// Fix error with at least two solutions
fn main() {
    let s = "hello, world";
    greetings(s.to_string())
}

fn greetings(s: String) {
    println!("{}", s)
}

//adicionado o to string() para a chamada da função greetings