fn main() {
    let v1 = 251_u32 + 8_u32;
    let v2 = u32::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }

 //Feito