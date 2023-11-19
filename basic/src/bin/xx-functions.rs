fn sum(num1:i32, num2:i32) -> i32 {
    num1 + num2 // return is implicit, is not necessary to use return and ;
}
fn sum2(num1:i32, num2:i32) -> i32 {
    return num1 + num2; // common approach
}
fn void_fn ()->(){ //this is a function that return nothing like a (void em C) -> ()
    println!("other fn");
}
fn last_line_fn ()->i32{ //the last line of a function is a return
    1 //this is a return of a function (not use ;)
}
fn return_fn (my_number:i32)->i32{ //is possible use return in a function but is not necessary, is useful for early returns in a function
    if my_number > 0   {
        return 0;
    }
    1 
}
fn _higher_order_fn (my_number:i32, my_closure:fn(i32,i32)->i32)->impl FnOnce(i32)->i32{ //is possible use a function as a parameter
    let nm = move|x:i32| my_closure(my_number,x);
    return nm;
}
// is NOT possible use named parameters in rust for call funcions like python (my_function1(my_param=1))
// is NOT possible use default parameters in rust for call funcions like python (my_function2(var1,my_param=1):)
fn main(){
    let my_closure = |x:i32| x + 1; //Closure is a anonymous function , like a labda in python (lambda x: x + 1) or a arrow function in javascript (x => x + 1)
    println!("closure(1) sum+1 = {}",my_closure(1)); //you can call a closure in the same line
    println!("closure(1) sum-1 = {}",(|x| x -1)(1)); //you can call a closure in the same line
    let n1 = 10;
    let n2 = 2;
    let sum3 = sum;
    let result = sum3(n1,n2);
    println!("{} + {} is {}", n1, n2 , result);
    assert!(result == sum2(n1,n2));
    void_fn();
    println!("last_line_fn = {}",last_line_fn());
    println!("return_fn = {}",return_fn(1));
}