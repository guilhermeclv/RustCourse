// Fix the error with at least two solutions
fn main() {
    let s:&str = "hello, world".into();
    greetings(s)
}

fn greetings(s: &str) {
    println!("{}",s)
}
//Explicar o uso do BOX e do .INTO()