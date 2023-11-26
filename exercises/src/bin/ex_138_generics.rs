// Add generic for Val to make the code work, DON'T modify the code in `main`.
struct Val<T> {
    val: T,
}

impl<T> Val<T> { // why need use impl<T> Val<T> (see the example below)
    fn value(&self) -> &T {
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}

//exemple
struct Val2<T, U> {
    x: T,
    y: U,
}

impl<T> Val2<i32, T> {
    fn get_x(&self) -> i32 {
        self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}