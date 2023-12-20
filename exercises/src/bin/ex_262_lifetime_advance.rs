

// impl<'a> PartialEq<i32> for &'a T {
//     // ...
// }

/* Adding HRTB to make it work!*/
fn call_on_ref_zero<F>(f: F) //where F: Fn(&'a i32) -> it not work beacuse we need to specify the lifetime of the reference
where for<'a> F: Fn(&'a i32) 
{
    let zero = 0;
    f(&zero);
}
fn call_fn_simple<F>(f: F)->()
where F: Fn(&i32)
{
    f(&1);
}

fn main() {
    println!("Success!");
    let my_fn = |x: &i32| println!("my fn: paramns ({})", x);
    call_on_ref_zero(my_fn);
    call_fn_simple(my_fn);
}
