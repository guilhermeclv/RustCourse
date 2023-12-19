//examples
impl<'a> Reader for BufReader<'a> {
    // 'a is not used in the following methods
}

// can be written as :
impl Reader for BufReader<'_> {
    
}

// Rust 2015
struct Ref<'a, T: 'a> {
    field: &'a T
}

// Rust 2018
struct Ref<'a, T> {
    field: &'a T
}