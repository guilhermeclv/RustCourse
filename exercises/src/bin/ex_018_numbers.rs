fn main() {
    let mut sum = 0;
    for i in -3..3 { // -3 -2 -1 0 1 2 (using ..= to include the last value) or -3..3 (using .. to exclude the last value)
        sum += i
    }

    assert!(sum == -3);

    for c in 'a'..='z' {
        println!("{}",c);
    }
}