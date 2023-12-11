
fn main() {
    let arr :[u64; 13] = [999; 13];
    //value in memory hex:0x0000_0000_0000_03E7|0x0000_0000_0000_03E7|...|0x0000_0000_0000_03E7 
    assert_eq!(std::mem::size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr; // a is a pointer to the first element of arr
    let b = a as *const [u8]; // b is a pointer to the first byte of arr BUT can generate a wrong pointer
    unsafe {
        assert_eq!(std::mem::size_of_val(&*b), 1 * 13)
    }
    println!("arr: {:?}", arr);
    println!("a: Point:{:?} - value first: {:?}", a, unsafe{(*a)[0]});
    println!("b: Point:{:?} - value first: {:?}", b, unsafe{(*b)[0]});
    // why 231? because 999 = 0b0000_0000_0000_0000_0000_0011_|1110_0111| and 231 = 0b1110_0111
    println!("b: Point:{:?} - value second: {:?}", b, unsafe{(*b)[1]});
    // why 3? because 999 = 0b0000_0000_0000_0000_|0000_0011|_1110_0111 and 3 = 0b0000_0011

    println!("Success!");
}