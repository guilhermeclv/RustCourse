// use utf8_slice;
// fn main() {
//     let s = "The 🚀 goes to the 🌑!";

//     let rocket = utf8_slice::slice(s, 4, 5);
//     // Will equal "🚀"
// }
fn main() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}