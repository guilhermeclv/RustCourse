fn main() {
    let x=define_x();
    println!("{}, world", x); 
}

fn define_x() -> &'static str {
    let x:&str = "hello";
    x
}
//Feito