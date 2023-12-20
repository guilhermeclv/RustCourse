#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
// repair static lifetime and not use let mut
static mut config: Option<&mut Config> = None;

/* Make it work without changing the function signatures of `init`*/
// fn init() -> Option<&'static mut Config> {
//     Some(&mut Config {
//         a: "A".to_string(),
//         b: "B".to_string(),
//     })
// }

fn init2() -> Option<Config> {
    Some(Config {
        a: "A".to_string(),
        b: "B".to_string()
    })
}

fn init() -> Option<&'static mut Config> {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    Some(Box::leak(c))
}

fn main() {

    let x = Box::new(41);
    // try to change the value of x
    let static_ref = Box::leak(x);
    *static_ref += 1;
    assert_eq!(*static_ref, 43);
    //println!("static {:?} original {:?}",static_ref,x); //not work because x is moved

    unsafe {
        config = init();

        println!("{:?}",config)
    }
}