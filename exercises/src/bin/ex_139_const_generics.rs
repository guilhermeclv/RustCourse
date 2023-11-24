//Here's an example of a type and implementation making use of const generics: a type wrapping a pair of arrays of the same size.
struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T, N> {
    // ...
}