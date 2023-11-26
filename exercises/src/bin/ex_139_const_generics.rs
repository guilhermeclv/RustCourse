//Here's an example of a type and implementation making use of const generics: a type wrapping a pair of arrays of the same size.
use std::fmt;
struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for ArrayPair<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArrayPair {{ left: {:?}, right: {:?} }}", self.left, self.right)
    }
}

fn main(){

    let arr = ArrayPair {
        left: [1, 2, 3],
        right: [4, 5, 6],
    };

    println!("{:?}", arr);
}