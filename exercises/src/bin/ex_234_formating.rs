//You can pad numbers with extra zeros

fn main() {
    println!("Hello {:5}!", 5); // => Hello     5!
    println!("Hello {:+}!", 5); // =>  Hello +5!
    println!("Hello {:05}!", 5); // => Hello 00005!
    println!("Hello {:05}!", -5); // => Hello -0005!
    let xyz = format!("{number:0>width$}", number=1, width=6);
    println!("{}", xyz);

    /* Fill in the blank */
    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");
    
    println!("Success!")
;}
