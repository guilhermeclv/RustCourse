fn main (){
    // TO THINK in scope we need to think in the life time of a variable, all variables have a life time
    // In rust the scope is defined by the brackets {} 
    // all inside the brackets is in the same scope and dies at the end of the scope
    // all outside the brackets is in the same scope and dies at the end of the main function

    let my_var:u64 = 10;
    let var_calculate = { // it is important use brackets to free memory when the block is finished
        let my_var2:u64 = 20;
        my_var + my_var2 // the last line is return
    };

    println!("var_calculate: {}", var_calculate);

    // you can define a scope without return, only for free memory
    {
        let my_var3:u64 = 30;
        println!("my_var3: {}", my_var3);
    }

    //println!("my_var3: {}", my_var3); // error: not found `my_var3` in this scope

    // you can not use return word in a block
    // because it is not a function (word return is only for functions)
    
    // let _var_calculate2 = {
    //    let _my_var4:u64 = 40;
    //    return _my_var4; // error: cannot return value referencing local variable `my_var4`
    // };

    // in this moment is important we start thinking in lifetimes
    // what is a lifetime? is the time that a variable is alive in memory
    // in the exemple below my_var6 is alive until the end of the block and my_var5 is alive until the end of the program

    let my_var5:u64 = 50;
    let var_calculate3 = {
        let my_var6:u64 = 60;
        my_var5 + my_var6
    };
    println!("var_calculate3: {} , my_var5: {}", var_calculate3, my_var5);
    // but if we try to use my_var6 outside the block we will have an error
    //println!("my_var6: {}", my_var6); // error: not found `my_var6` in this scope

    // my_ref_var is a reference of i32 (&i32), don't confuse with i32, We will see in the future
    // but you can think my_ref_var like a reference of a value in memory, not the value itself
    // in this case we can't do: my_ref_var = &my_var8; because my_var8 is not alive until the end of the main function
    // i.e, the value in memory of my_var8 will dies at the end of the block

    let mut my_ref_var = &70; // my_ref_var is alive until the end of the main function 
    let mut my_copy_var = 70; // my_copy_var is alive until the end of the main function
    {
        let mut my_var8 = 80; // my_var8 is alive until the end of the block
        //my_ref_var = &my_var8; // generate error: `my_var8` does not live long enough
        my_copy_var = my_var8; // this is possible because my_var8 is copied to my_copy_var
    }
    println!("my_ref_var: {}, my_copy_var: {}", *my_ref_var, my_copy_var); // this is possible because my_var8 is copied to my_ref_var

}