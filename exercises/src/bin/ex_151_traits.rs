struct Sheep {}
struct Cow {}

trait Animal {
    fn noise(&self) -> String;

}

impl Animal for Sheep {
    fn noise(&self) -> String {
        "baaaaah!".to_string()
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        "moooooo!".to_string()
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
// FIX the errors here, you can make a fake random, or you can use trait object.

// fn random_animal(random_number: f64) -> impl Animal {
//     if random_number < 0.5 {
//         Cow {}
//     } else {
//         Cow {}
//     }
// }
enum RandomAnimal {
    Sheep(Sheep),
    Cow(Cow),
}

impl Animal for RandomAnimal {
    fn noise(&self) -> String {
        match self {
            RandomAnimal::Sheep(sheep) => sheep.noise(),
            RandomAnimal::Cow(cow) => cow.noise(),
        }
    }
}

fn random_animal(random_number: f64) -> RandomAnimal {
    if random_number < 0.5 {
        RandomAnimal::Sheep(Sheep{})
    } else {
        RandomAnimal::Cow(Cow{})
    }
}
// this is the best way to do it, but it's not the only way
// fn random_animal(random_number: f64) -> Box<dyn Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow {})
//     }
// }

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}