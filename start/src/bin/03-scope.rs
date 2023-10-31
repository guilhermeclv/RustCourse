fn main (){

    let my_var:u64 = 10;
    let var_calculate = { // it is important use brackets to free memory when the block is finished
        let my_var2:u64 = 20;
        my_var + my_var2 // the last line is return
    };
    //println!("my_var2: {}", my_var2); // error: not found `my_var2` in this scope
    println!("var_calculate: {}", var_calculate);
}