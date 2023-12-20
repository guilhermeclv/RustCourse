/* Make it work by adding proper lifetime annotations */
#[derive(Debug)]
struct ImportantExcerpt <'t> {
    part: &'t str,
}

impl <'a> ImportantExcerpt <'a>{
    fn level (&'a self) -> i32 {
        3
    }
}

fn main() {
    let test_1 = "test_1";
    let my_struc = ImportantExcerpt { part: test_1 };
    println!("my_struc: {:?}", my_struc);
    println!("my_struc.level(): {:?}", my_struc.level());
}