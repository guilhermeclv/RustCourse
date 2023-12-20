
/* Make it work by adding proper lifetime annotation */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = "abcd";
    let s2 = "xyz";
    let result = longest(s1, s2);
    println!("The longest string is {}", result);
    // s3 and s1 have different lifetime? 
    // why it work without lifetime annotation?
    // because s3 is a string literal, which has a static lifetime
    {
        let s3 = String::from("abcd");
        let result2 = longest(&s1, &s3);
        println!("The longest string is {}", result2);
    }
    let s4 = s1;
    println!("s4 is {}", s4);

}