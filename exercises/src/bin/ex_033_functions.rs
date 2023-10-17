fn main() {
    println!("Success!");
    let x = get_option(1);
    println!("x = {:?}", x.unwrap_or(0));
}

fn get_option(tp: u8) -> Option<i32> {
   match tp {
       1 => {
           return Some(1);
       }
       _ => {
           return None;
       }
   };
   
   // Rather than returning a None, we use a diverging function instead
   never_return_fn()
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    unimplemented!()
    //todo!();
}