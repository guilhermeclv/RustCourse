struct Unit{
    age: u8
}
trait SomeTrait { // Similar a interface in C#, Java,Ts
    // ...Some behaviors defined here.
    fn write_age(&self) -> u8;
}

// We don't care about what fields  are  in the Unit, but we care about its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait for Unit { 
    fn write_age(&self) -> u8 {
        println!("Age: {}", self.age);
        self.age.clone()
    }
 }
fn main() {
    let u = Unit { age: 30 };
    let a = u.write_age();
    do_something_with_unit(u);
    println!("Success!");
} 

// Fill the blank to make the code work
fn do_something_with_unit(u:Unit) {   }