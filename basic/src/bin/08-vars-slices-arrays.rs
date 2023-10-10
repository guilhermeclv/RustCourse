use std::array;

fn main(){
    //Arrays
    let mut my_first_array = ["my","name","is","joão pedro"]; // is not possible mix typing in a array it is possible in (only in tuple)
    let mut other_sintaxe_array = ["";5]; // equivalent to [1,1,1,1,1]
    other_sintaxe_array[0] = "my name";
    //using get method
    println!("my_first_array = [{}] [{}] [{}] [{}]",my_first_array.get(999).unwrap_or(&"erro_index"),my_first_array.get(1).unwrap(),my_first_array.get(2).unwrap(),my_first_array.get(3).unwrap());
    println!("common array = [{}] [{}] [{}] [{}]",my_first_array[0],my_first_array[1],my_first_array[2],my_first_array[3]);
    println!("other_sintaxe_array lenght = {}",other_sintaxe_array.len());

    let my_first_slice = &my_first_array[0..2];
    let my_copy_array = &my_first_array[..]; // copy all array
    let _my_copy_array2 = &my_first_array[1..]; 
    let _my_copy_array3 = &my_first_array[..3];
    
    println!("my_first_slice = [{}] [{}]",my_first_slice[0],my_first_slice[1]);
    println!("lenght of my_first_slice = {}",my_first_slice.len());
    assert!(my_copy_array==my_first_array);
    println!("");

    let slice_array = &mut my_first_array[0..2];
    slice_array[0] = "changed1";
    println!("slice_array = [{}] [{}]",slice_array[0],slice_array[1]);
    println!("my_first_array = [{}] [{}]",my_first_array[0],my_first_array[1]);
    println!("");

    let editable_slice_array = &mut my_first_array[0..2].to_vec();
    editable_slice_array[0] = "changed2";
    println!("editable_slice_array = [{}] [{}]",editable_slice_array[0],editable_slice_array[1]);
    println!("my_first_array = [{}] [{}]",my_first_array[0],my_first_array[1]);
    println!("");

    //Tuple
    let my_tuple = ("my","name","is","joão pedro");
    println!("debug mode print={:?}",my_tuple); // debug mode (print all tuple)
    println!("my_tuple = [{}] [{}] [{}] [{}]",my_tuple.0,my_tuple.1,my_tuple.2,my_tuple.3);
    let (a,b,c,d) = my_tuple; // destructuring
    println!("destructuring my_tuple = [{}] [{}] [{}] [{}]",a,b,c,d);
    let array_tuple = [my_tuple;4];
    println!("array_tuple = [{}] [{}] [{}] [{}]",array_tuple[0].0,array_tuple[1].1,array_tuple[2].2,array_tuple[3].3);
    let mut my_tuple_2 = ("my","name","is","joão pedro"); // tuple is not mutable, but you can change the value of a variable
    my_tuple_2.0 = "my name";
    my_tuple_2 = ("my name","is","joão pedro","and i'm 18 years old"); // you can change the value of a variable
    println!("mytuple_2 = {:?}",my_tuple_2);
    println!("");
    //enum
    enum Color{
        Red,
        Green,
        Blue
    }

    let my_color = Color::Red;
    
    match my_color{
        Color::Red => println!("my_color is Red"),
        Color::Green => println!("my_color is Green"),
        Color::Blue => println!("my_color is Blue"),
    }
    


}