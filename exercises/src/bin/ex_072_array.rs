// let v = [10, 40, 30];
// assert_eq!(Some(&40), v.get(1));
// assert_eq!(Some(&[10, 40][..]), v.get(0..2));
// Fix the error
fn main() {
    let mut names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
     let name0 = names.get(0).unwrap();

    // // But indexing is not safe
     let _name1 = &names[1];

    // names[0].push_str(" string");

    println!("The first name is: {} and de second is {} ", name0, _name1);
     println!("The names array has {:?} elements", names);

    println!("Success!");
}