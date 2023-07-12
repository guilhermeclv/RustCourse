fn copy_vars(){
    let original_var = 1_i32; // (primitive types are copied by default in rust like int, float, bool, char, etc)
    let mut copy_var = original_var; //copy the value of original_var it is not a reference
    copy_var+=1;
    println!("Copy - original var:{orginal_var}, copy var:{copy_var}",orginal_var=original_var,copy_var=copy_var);
}
fn copy_reference(){
    let original_var = 1_i32;
    let copy_var = &original_var; //copy the reference of original_var it is a reference ( pointer in C)
    //*copy_var=2_i32; //this is not possible because copy_var is a reference
    //original_var+=1_i32; //this is not possible because original_var is borrowed
    println!("Reference - original var:{}, copy var:{}",original_var,*copy_var);
}
fn copy_reference_mut(){
    let mut original_var = 1_i32;
    let copy_var = &mut original_var; //copy the reference of original_var it is a reference ( pointer in C)
    *copy_var=2_i32; //this is possible because copy_var is a mutable reference
    //original_var+=1_i32; //this is not possible because original_var is borrowed
    //println!("Reference - original var:{}, copy var:{}",original_var,*copy_var); not work because original_var is borrowed
    println!("Reference - original var:{}, copy var:{}",1,*copy_var);
}
fn understanding_ownership(){
    // by default all variables in heap area are moved (change the ownership)
    let original_var = String::from("Hello");
    let copy_var1 = original_var; // change the ownership , original_var is moved to copy_var1 (original_var not exist anymore)
    let copy_var2 = copy_var1; // change the ownership, (copy_var1 not exist anymore)
    println!("Reference - original var:-, copy var1:-, copy var2:{}",copy_var2); // not work because original_var is moved to copy_var change the ownership
}
fn copy_reference_heap_ownership(){
    let original_var = String::from("Hello");
    //let copy_var = original_var; // change the ownership
    //println!("Reference - original var:{}, copy var:{}",original_var,copy_var); // not work because original_var is moved to copy_var change the ownership
    let copy_var = &original_var; //copy the reference of original_var it is a reference ( pointer in C)
    println!("Reference - original var:{}, copy var:{}",original_var,*copy_var);
}
fn say_hello(name: &String){
    println!("Hello {}",name);
}
fn say_hello2(name: String){
    println!("Hello {}",name);
}
fn change_name(name: &mut String){
    *name = String::from("Samarino");
}
fn main (){
    copy_vars();
    copy_reference();
    copy_reference_mut();
    understanding_ownership();
    copy_reference_heap_ownership();

    // In rust is possible have only 1 mutable reference per time (in a scope)
    let mut name = String::from("John");
    //say_hello2(name); // name is borrowed not ideal
    say_hello(&name);
    say_hello2(name.clone()); // name is copied
    change_name(&mut name); // name is borrowed and mutable in other scope
    name.push_str(" !"); // is possible change because change_name finish and name is not borrowed anymore
    println!("Name:{}",name); // name is not borrowed

}