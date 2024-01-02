//examples
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {
    // --snip--
}

// fn returns_long_type() -> Thunk {
//     // --snip--
// }

 type Result<T> = std::result::Result<T, std::io::Error>;

fn main() {
    let f: Thunk = Box::new(|| println!("hi"));

    type Meters = u32;
    
    let x: u32 = 5;
    let y: Meters = 5;
    
    println!("x + y = {}", x + y);
    }