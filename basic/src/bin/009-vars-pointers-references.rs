fn main() {
    let value = 42;
    
    // Obtendo o endereço de memória da variável 'value'
    let pointer = &value as *const i32;

    // Imprimindo o endereço de memória
    println!("Endereço de memória de 'value': {:p}", pointer);

    let mut x = 10;
    
    // Criando um ponteiro raw mutável para 'x'
    let raw_ptr = &mut x as *mut i32;

    // Manipulando o valor através do ponteiro raw
    unsafe {
        *raw_ptr += 5;
    }

    // Imprimindo o valor modificado
    println!("Valor modificado: {}", x);

    let boxed_value = Box::new(7);

    // Obtendo um ponteiro raw constante a partir do Box
    let raw_ptr = Box::into_raw(boxed_value);

    // Convertendo o ponteiro raw novamente para Box (evitando vazamento de memória)
    let boxed_value = unsafe { Box::from_raw(raw_ptr) };

    // Imprimindo o valor
    println!("Valor recuperado: {}", boxed_value);

    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Criando um ponteiro constante para a struct
    let person_ptr = &person as *const Person;

    // Convertendo o ponteiro para uma referência segura
    let reference: &Person = unsafe { &*person_ptr };

    // Imprimindo a referência
    println!("Person: {:?}", reference);
}
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}