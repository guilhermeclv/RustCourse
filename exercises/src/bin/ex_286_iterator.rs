// all interators implement the Iterator trait which is defined as follows:

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // Methods with default implementations elided
// }

/* Fill the blanks and fix the errors.
Using two ways if possible */
fn main() {
    let mut v1 = vec![1, 2].into_iter() ;

    assert_eq!(v1.next(), Some(1));
    assert_eq!(v1.next(), Some(2));
    assert_eq!(v1.next(), None);
}