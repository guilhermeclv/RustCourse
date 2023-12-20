//examples
struct Ref<'a, T: 'a> {
    field: &'a T
}

// Rust 2018
struct Ref2<'a, T> {
    field: &'a T
}
trait Reader {
    fn read(&self) -> String;
}

struct BufReader<'a>(&'a str);

// impl<'a> Reader for BufReader<'a> {
//     fn read(&self) -> String {
//         self.0.to_string()
//     }
// }

//newest version to make it work
impl Reader for BufReader<'_> {
    fn read(&self) -> String {
        self.0.to_string()
    }
}


fn main() {

}