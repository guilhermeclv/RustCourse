//exemple
fn main() {
    let my_str = "hello";

    // three conversions below all depends on the fact: String implements From<&str>:
    let string1 = String::from(my_str);
    let string2 = my_str.to_string();
    // Explicit type annotation is required here
    let string3: String = my_str.into(); // this try to convert to original type
}