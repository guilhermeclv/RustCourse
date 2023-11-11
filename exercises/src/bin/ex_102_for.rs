// Fix the errors without adding or removing lines
fn main() {
    let names = [String::from("liming"),String::from("hanmeimei")];
    for name in &names { // need to add & here because name is a object
        // Do something with name...
        println!("{}", *name)
    }

     println!("{:?}", names);

    let numbers = [1, 2, 3];
    // // The elements in numbers are Copy，so there is no move here
    for n in numbers {
        // Do something with name...
        let random: i32 = n+1;
        println!("{}", random)
    }
    
    println!("{:?}", numbers);
} 