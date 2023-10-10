fn main() {
    let v1 = 247_u8 + 8; // this will overflow return error
    let v2 = i8::checked_add(119, 8).unwrap(); // it is very important to use unwrap() here
    println!("{},{}",v1,v2);
 }