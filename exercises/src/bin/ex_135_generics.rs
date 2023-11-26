// Modify this struct to make the code work
struct Point<T,K> {
    x: T,
    y: K,
}

fn main() {
    // DON'T modify this code.
    let p = Point{x: 5, y : "hello".to_string()};

    println!("Success!");
}