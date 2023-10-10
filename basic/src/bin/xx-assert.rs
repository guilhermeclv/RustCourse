fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
#[derive(Debug, PartialEq)] // PartialEq is a trait that allow to compare two structs 
struct Person {
    name: String,
    age: u32,
}

// impl PartialEq for Person { // another way to implement PartialEq via impl
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name && self.age == other.age
//     }
// }

fn main(){

    let x = 10;
    assert!(x > 0, "O valor de x deve ser maior que 0"); //verify if expression is true

    let result = add(2, 3);
    assert_eq!(result, 5, "A soma está incorreta"); // verify if expression is equal

    let result = subtract(7, 2);
    assert_ne!(result, 0, "A subtração não pode ser igual a 0"); // verify if expression is not equal

    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let person2 = Person {
        name: "Bob".to_string(),
        age: 25,
    };

    // Verificando igualdade usando assert_eq!
    assert_eq!(person1,person1); // Passa, pois é a mesma struct
    assert_ne!(person1,person2); // Falha, pois são structs diferentes


}