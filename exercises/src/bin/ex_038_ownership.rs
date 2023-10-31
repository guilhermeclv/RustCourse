// Fix the error without removing code line
fn main() {
    let s = String::from("hello, world");

    print_str(&s);
    print_str2(s.to_owned());
    println!("{}", s);
}

fn print_str(s: &String)  {
    println!("{}",s)
}
fn print_str2(s: String)  {
    println!("{}",s)
}