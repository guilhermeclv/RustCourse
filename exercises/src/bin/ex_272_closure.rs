//example
fn main() {
    let mut movable = Box::new(3);

    let mut consume = move || {
        println!("`movable`: {:?}", movable);
        // add 1 to the value of `movable`
        *movable += 1;
    };

    consume();
    consume();

    //clousure every need has their parameter well defined or an execution context
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| { x + 1 };
    let add_one_v4 = |x|   x + 1  ; // infered type
    let use_context = add_one_v4(1_u32);
    println!("use_context: {}", use_context);

    // not work because the compiler can't compile the clousure whit generic type
    //let use_context2 = add_one_v4(1_i32);

}