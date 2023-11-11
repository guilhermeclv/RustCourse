
fn main() {
    let arr: [char; 5] = ['中', '国', '人', '国', '人'];

    let slice = &arr[..=2]; // 8 bytes to store the pointer and 8 bytes to store the length = 16 bytes
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);
    assert!(std::mem::size_of_val(&arr) == 20);
    println!("Success!");
}
