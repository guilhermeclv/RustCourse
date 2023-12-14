//exemples of formating
fn main() {
    // Exponent
    println!("{:2e}", 1000000000); // => 1e9 cientific notation
    println!("{:2E}", 1000000000); // => 1E9

    // Pointer address
    let v= vec![1, 2, 3];
    println!("{:p}", v.as_ptr()); 


    let reference_to_vec = &v;
    let pointer_to_vec: *const i32 = v.as_ptr();
    println!("p1:{:p} p2:{:p}", reference_to_vec, pointer_to_vec);
    //reference_to_vec is a struct with name fat pointer (pointer + len + capacity) and this struct has a pointer diferente from pointer_to_vec
    //why pointer_to_vec is different from v.as_ptr()?
    //because pointer_to_vec is a reference to v, not the pointer to v
    //so pointer_to_vec is a pointer to a pointer
    //to se it we can use de code bellow
    let fat_pointer_len=reference_to_vec.len();
    let fat_pointer_ptr=reference_to_vec.as_ptr();
    let fat_pointer_cap=reference_to_vec.capacity();
    println!("fat_pointer_len:{} fat_pointer_ptr:{:p} fat_pointer_cap:{}", fat_pointer_len, fat_pointer_ptr, fat_pointer_cap);
    

    // Escape
    println!("Hello {{}}"); // => Hello {}
}
