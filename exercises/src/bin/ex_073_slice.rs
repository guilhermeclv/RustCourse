
// Fix the errors, DON'T add new lines!
fn main() {
    let arr = [1, 2, 3];
    let s1:&[i32] = &arr[0..2];
    
    let s2:&str = "hello, world";
    
    let dinamic_index:usize = 2;
    let s3:&[i32] = &arr[dinamic_index..3];
    //safe way to made slice return a option
    let s4:&[i32] = arr.get(dinamic_index..3).unwrap_or(&[]);

    // print s1 values
    println!("s1: {:?}", s1.get(1).unwrap_or(&0));
    println!("s3: {:?}", s3);
    println!("dinamic_index:{dinamic_index:#034b}");

    println!("Success!");
}