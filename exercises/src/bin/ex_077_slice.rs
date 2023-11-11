
fn main() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3]; // you need remember that the index of the first byte of a char is the index of the char
    // to get a slice of chars using a safe method, you can use the char_indices() method
    let initial_byte = &s.char_indices().nth(0).unwrap().0;
    let final_byte = &s.char_indices().nth(1).unwrap().0;
    let slice2 = &s[*initial_byte..*final_byte];
    let slice_copy:String = s.chars().skip(0).take(2).collect(); //best way to get a slice of chars
    println!("slice_copy {:?}", slice_copy);
    // see the char_indices() method in action
    let test = s.char_indices();
    for (i, c) in test {
        println!("{}: {}", i, c);
    }
    //using string you have the same problem
    let my_string = String::from(s);
    let slice3 = &my_string[0..3];
    println!("slice3 {:?}", slice3);
    
    

    println!("slice2 {:?}", slice2);
    assert!(slice == "你");

    println!("Success!");
}
