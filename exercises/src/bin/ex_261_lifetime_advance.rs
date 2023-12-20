/* Adding trait bounds to make it work */
fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) 
where 'a: 'b
{
    y = x;                      
    let r: &'b &'a i32 = &&0; // two references with different lifetimes are allowed, but b' must outlive 'a because it is a reference to a reference
    let deref_1: &'a i32 = *r; // deref1 is a reference to a value
    let deref_2: i32 = **r; // deref2 is a value
}

fn main() {
    println!("Success!");
    let x = 42;
    
    //understanding references
    let reference_1: &i32 = &x;
    let reference_2: &&i32 = &&x; // it is a reference to a reference
    let deref_once: &i32 = *reference_2; // it is a reference to a value
    let deref_two:i32 = **reference_2; // it is a values

    println!("Value: {}", x);
    println!("Reference 1: {:?}", reference_1);
    println!("Reference 2: {:?}", reference_2);
    println!("Deref once: {:?}", deref_once);
    println!("Deref twice: {:?}", deref_two);
}
