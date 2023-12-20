//examples of lifetime
fn main() {
    let mut s = String::from("hello");
 
     let r1 = &s;
     let r2 = &s;
     println!("{} and {}", r1, r2);
 
     let r3 = &mut s;
     println!("{}", r3);
     main2();
}


fn main2() {
    let mut u = 0i32;
    let mut v = 1i32;
    let mut w = 2i32;
    
    // lifetime of `a` = α ∪ β ∪ γ
    let mut a = &mut u;     // --+ α. lifetime of `&mut u`  --+ lexical "lifetime" of `&mut u`,`&mut u`, `&mut w` and `a`
    //use(a); 
    println!("a0:{a}");                //   |                            |
    *a = 3; // <-----------------+                            |
    println!("a1:{a}");                    //                                |
    a = &mut v;             // --+ β. lifetime of `&mut v`    |
    println!("a2:{a}");                  //   |                            |
    *a = 4; // <-----------------+                            |
    println!("a3:{a}");                       //                                |
    a = &mut w;             // --+ γ. lifetime of `&mut w`    |
    println!("a4:{a}");                //   |                            |
    *a = 5; // <-----------------+ <--------------------------+
    println!("a5:{a}");   
    }