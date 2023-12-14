#[derive(Debug)]
struct Structure(i32);


struct Deep(Structure);
//implement display format for deep
impl std::fmt::Display for Deep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0.0)
    }    
}
//implement debug format for deep
impl std::fmt::Debug for Deep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.0.0)
    }
}
fn main() {    
    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?

    /* Make it print: Now 7 will print! */
    println!("Now {:?} will print!", Deep(Structure(7)));
    println!("Now {} will print!", Deep(Structure(7)));

    println!("Now {} will print!", Deep(Structure(7)).0.0);

}
