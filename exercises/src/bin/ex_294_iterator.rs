/* Fill in the blanks */
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().__.__;

    assert_eq!(v2, vec![2, 3, 4]);
}
