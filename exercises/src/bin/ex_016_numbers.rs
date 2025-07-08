// Fill the blank to make it work
fn main() {
    let x = 1_000.000_1_f64; // f64
    let y = 0.12_f32; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(x), "f64".to_string()); //aprender mais sobre assert_eq
    println!("Success!");
}

fn type_of<T>(_: T) -> String {
    format!("{}", std::any::type_name::<T>())
}