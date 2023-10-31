fn main() {
    let mut t = (String::from("hello"), String::from("world"));
 
     // Fill the blanks
     let (ref s1, ref s2) = t;
     // t.0 = String::from("hello2"); // Error! cannot assign to `t.0` which is behind a `&` reference
     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
 }