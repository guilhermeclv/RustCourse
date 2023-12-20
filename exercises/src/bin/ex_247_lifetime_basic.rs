/* Make it work */

use std::clone;

#[derive(Debug,Clone)]
struct NoCopyType {}

#[derive(Debug)]
struct Example<'a, 'b> {
    a: &'a u32,
    b: &'b NoCopyType
}

fn main()
{ 
  /* 'a tied to fn-main stackframe */
  let var_a = 35;
  let example: Example;
  let var_b: NoCopyType;
  {
    /* Lifetime 'b tied to new stackframe/scope */ 
    var_b = NoCopyType {};
    
    /* fixme */
    example = Example { a: &var_a, b: &var_b };
  }
  
  println!("(Success!) {:?}", example);
}
