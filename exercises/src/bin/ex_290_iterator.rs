//example
struct Counter {
    count: i32,
}

impl Counter {
    fn new(initial_value:i32) -> Counter {
        Counter { count: initial_value }
    }
}

impl Iterator for Counter { // trait Iterator is defined in the standard library
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new(0);

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    for i in Counter::new(-5) {
        println!("{}", i);
    }
}
