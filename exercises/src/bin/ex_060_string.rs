// Fix errors without removing any line
// fn main() {
//     let s1 = String::from("hello,");
//     let s2 = String::from("world!");
//     let s3 = s1.clone() + &s2; 
//     assert_eq!(s3, "hello,world!");
//     println!("{}", s1);
// }

// //Porque usar o clone, sugestão do RUST

fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    //s1.push_str(&s2);
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}