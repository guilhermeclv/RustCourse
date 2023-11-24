fn main(){
    // In rust we have 3 types of variables
    // let - immutable variable
    // mut - mutable variable
    // const - immutable const, but with a fixed value in the time

    // CONSTS

    const MY_CONST_NAME:i64 = 1; // in general we use uppercase and snake case for const names
    const MY_CONST:u32 = 1; // const need to be difined with a type and start with a value
    println!("const value: (MY_CONST_NAME)={},(MY_CONST)={}",MY_CONST_NAME, MY_CONST); // this line will compile, because const is immutable
    
    //const MY_CONST2:u32; // const need to be difined with a type and start with a value
    //MY_CONST2 = 2; // this line will not compile, because const need to be initialized with a value


    // LET
    // let can assume only one value during the execution of the program
    // but you not need to initialize the let with a value
    //ex:
    let my_let_2:u32; // let need to be difined with a type or a value, but in this case you need initialize with a type
    // println!("let value: (my_let_2)={}",my_let_2); // THIS NOT WORK BECAUSE YOU DON'T ASSIGN A VALUE TO THIS VARIABLE
    my_let_2 = 2; // you are assigning a value to this variable, (BUT YOU EVER NEED TO ASSIGN A VALUE TO A VAR BEFORE USE IT)
    // my_let_2 = 3; // NOT WORK, BECAUSE LET IS IMMUTABLE
    println!("let value: (my_let_2)={}",my_let_2); 
    

    let my_let_name = 1.0; // in general we use snake case for let names
    let my_let:i64 = 1_i64; // let need to be difined with a type or a value
    
    // TAKE CARE with this line below, we will talk about shadowing later
    let my_let = 33_i64; // this work but you are not mutating the variable, you are redefining creating a new variable with the same name TAKE CARE

    println!("let value: (my_let_name)={},(my_let)={}",my_let_name, my_let); // this line will compile, because let is immutable

    // MUT
    // mut can assume many values during the execution of the program
    // but you not need to initialize the mut with a value
    //ex:
    let mut my_mut_2:u32; // mut need to be difined with a type or a value, but in this case you need initialize with a type
    // println!("mut value: (my_mut_2)={}",my_mut_2); // THIS NOT WORK BECAUSE YOU DON'T ASSIGN A VALUE TO THIS VARIABLE
    my_mut_2 = 2_u32; // you are assigning a value to this variable, (BUT YOU EVER NEED TO ASSIGN A VALUE TO A VAR BEFORE USE IT)
    my_mut_2 = 3; // WORK, BECAUSE YOU USE MUT SO YOU CAN CHANGE THE VALUE OF THIS VARIABLE
    my_mut_2 = 10; // WORK :)
    println!("mut value: (my_mut_2)={}",my_mut_2);

    // YOU CAN'T CHANGE THE TYPE VALUE OF VARIABLE IN RUST (IN GENERAL)
    // EX: IF YOU DEFINE A VARIABLE WITH I32 YOU CAN'T PUT A U32 VALUE IN THIS VARIABLE 
    let mut my_mut_name = 1100_i32; // in general we use snake case for mut names
    // my_mut_name = 1_000_000_u32; // not work, because you are changing the type of the variable
    my_mut_name=100000; // work, because you are not changing the type of the variable
    println!("mut value: (my_mut_name)={}",my_mut_name); // this line will compile, because mut is mutable



    

}