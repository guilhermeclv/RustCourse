// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(&s.to_string());

    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}