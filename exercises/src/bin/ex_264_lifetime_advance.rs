//example
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    // Here comes the reborrow
    let rr: &Point = &*r;
    //r.move_to(2, 10); not allowed anymore, NLL introduced
    println!("rr:{:?}, r:{:?}", rr,r); // Reborrow ends here, NLL introduced

    // Reborrow is over, we can continue using `r` now
    r.move_to(10, 10);
    println!("{:?}", r);
}