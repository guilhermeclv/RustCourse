
// Fill the blank
struct Person {
    name: String,
    age: u8,
}
fn main() {
    println!("Success!");
    let xyz = build_person("joao".to_string(), 2);

} 

fn build_person(name: String, age: u8) -> Person {
    let xyz = Person {
        age,
        name
    };
    println!("{} is {} years old.", xyz.name, xyz.age);
    xyz
}