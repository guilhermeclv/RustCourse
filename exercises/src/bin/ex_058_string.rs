// Fix all errors without adding newline
fn main() {
    let mut s = String::from("hello");
    s.push(',');// it not alter the string literal
    s.push_str(" world"); // it not alter the string literal
    s += "!"; // in this case you need alter the var to

    println!("{}", s);
}