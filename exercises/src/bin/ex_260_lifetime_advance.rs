/* Adding trait bounds to make it work */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a: 'c, 'c>  ImportantExcerpt<'a> { // 'a: 'c means that the lifetime 'a must outlive 'c
    fn announce_and_return_part(&'a self, announcement: &'c str) -> &'c str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    println!("Success!")
}
