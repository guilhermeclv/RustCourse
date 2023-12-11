//example
fn foo() -> i32 {
    0
}

fn main() {
    let pointer = foo as *const ();
    let function = unsafe {
        std::mem::transmute::<*const (), fn() -> i32>(pointer) // transmute: change the type of the pointer
    };
    assert_eq!(function(), 0);
}