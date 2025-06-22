//A String is stored as a vector of bytes (Vec<u8>), but guaranteed to always be a valid UTF-8 sequence. String is heap allocated, growable and not null terminated.
//&str is a slice (&[u8]) that always points to a valid UTF-8 sequence, and can be used to view into a String, just like &[T] is a view into Vec<T>.

// FILL in the blanks
fn main() {  
    let mut s = String::from("hello, world");
 
    let slice1: &str = &s; // In two ways
    assert_eq!(slice1, "hello, world");
 
    let slice2 = &slice1.split(", world").next().unwrap();
    assert_eq!(slice2.to_string(), "hello");
 
    let mut slice3: String = slice1.clone().to_string(); 
    slice3.push('!');
    assert_eq!(slice3, "hello, world!");
 
    println!("Success!");
 }