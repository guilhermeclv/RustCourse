fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert_eq!(sum, -5);

    for c in 'a'..='z' {
        if c=='c' {
        println!("{}",c);
        break;
        }
        
    }
}