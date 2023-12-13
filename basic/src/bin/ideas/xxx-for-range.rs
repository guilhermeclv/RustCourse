fn main(){
    for i in 0..5 {
        print!("{},", i);
    }
    println!("");
    for i in 0..=5 {
        print!("{},", i);
    }
    println!("");
    let myarray = [1,2,3,4,5];
    
    for i in myarray {
        print!("{},", i);
    }
    println!("");
    let v = vec![1,2,3,4,5];
    for i in &v {
        print!("{},", i);
    }
    println!("");
    let mut v = vec![1,2,3,4,5];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }
}