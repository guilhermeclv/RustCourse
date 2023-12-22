/* Fill in the blank using two aproaches,
 and fix the errror */
 fn create_fn() -> impl Fn(i32) -> i32 {
    let num = 5;

    move |x| x + num
}

fn create_fn2() -> impl FnMut(i32) -> i32 {
    let mut num = 5;

    move |x| {num=num+1; x + num}
}

fn create_fn3() -> Box<dyn Fn(i32) -> i32> {
    let num = 5;
    Box::new(move |x| x + num)
}

fn create_fn4() -> Box<dyn FnMut(i32) -> i32> {
    let mut num = 5;
    Box::new(move |x| {num=num+1; x + num})
}

fn main() {
    let fn_plain = create_fn();
    println!("fn_plain: {}", fn_plain(5));
    println!("fn_plain: {}", fn_plain(6));

    let mut fn_mut = create_fn2();
    println!("fn_mut: {}", fn_mut(5));
    println!("fn_mut: {}", fn_mut(6));

    let fn_box = create_fn3();
    println!("fn_box: {}", fn_box(5));
    println!("fn_box: {}", fn_box(6));

    let mut fn_box_mut = create_fn4();
    println!("fn_box_mut: {}", fn_box_mut(5));
    println!("fn_box_mut: {}", fn_box_mut(6));
}
