fn main() {
    assert!(0.1+0.2-0.3<1e-10); // never use float point to compare exact value (this is not a rust problem, but a float point problem)

    println!("Success!");
}