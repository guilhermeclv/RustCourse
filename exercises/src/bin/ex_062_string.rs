// Use two approaches to fix the error and without adding a new line
fn main() {
    let s = "hello, world".to_string();
    let s1= s.to_string();

    println!("Success!");
}

// fn main() {
//     let s:&str = "hello, world";
//     let s1: &str = s;

//     println!("Success!");
// } //Solução 2