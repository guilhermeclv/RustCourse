//example
fn main() {
    let movable = Box::new(3);

    let consume = move || {
        println!("`movable`: {:?}", movable);
    };

    consume();
    consume();

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

}