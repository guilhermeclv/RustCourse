// Fix the error
struct Person {
    name: String,
    age: u8,
    hobby: String
}
fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("programming")
    };
    assert_eq!(p.name, "sunface");
    println!("Success!");
} 
