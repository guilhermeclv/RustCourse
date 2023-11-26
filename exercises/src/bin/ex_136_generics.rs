struct Point<T, U> 
//where U:Copy, T:Copy
{
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // Implement mixup to make it work, DON'T modify other code.
    fn mixup<V, W>(self, p: Point<V, W>) -> Point<T, W>{
        Point { 
            x: self.x,
            y: p.y
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};

    let p3 = p1.mixup(p2);
    //let p34 = p1.mixup(&p2); //how to do it work ?
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');

    println!("Success!");
}