struct Xy{
    x:i32
}
fn main() {
    let x = Box::new(5);

    let mut y3 = Box::new(Xy{x:123});     // Implement this line, dont change other lines!
    
    (*y3).x = 4;
    y3.x = 5;
    
    let mut y2 = Box::new([3;3]);     // Implement this line, dont change other lines!
    
    (*y2)[0] = 4;
    y2[0] = 5;

    let mut y = Box::new(3);     // Implement this line, dont change other lines!
    
    *y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}