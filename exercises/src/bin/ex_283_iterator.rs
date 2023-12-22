//example

fn main() {
    let v = vec![1, 2, 3];
    for x in &v {
        println!("v0{}",x)
    }

    for x in v.clone().into_iter() {
        println!("v1{}",x)
    }

    for (value,index) in v.into_iter().enumerate() {
        println!("i{}: v2{}", index, value);
    }


}