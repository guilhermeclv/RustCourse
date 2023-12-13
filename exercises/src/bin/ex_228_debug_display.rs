/* Make it work*/
use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    /* Implement.. */
}

impl fmt::Debug for Point2D {
    /* Implement.. */
}

fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };
    assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    
    println!("Success!");
}
