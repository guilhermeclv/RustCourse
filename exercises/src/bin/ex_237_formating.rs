
fn main() {
    assert_eq!(format!("__", 27), "0b11011");
    assert_eq!(format!("__", 27), "0o33");
    assert_eq!(format!("__", 27), "0x1b");
    assert_eq!(format!("__", 27), "0x1B");

    println!("{:x}!", 27); // Hex with no prefix => 1b

    println!("{:#010b}", 27); // Pad binary with 0, width = 10,  => 0b00011011

    println!("Success!");
}
