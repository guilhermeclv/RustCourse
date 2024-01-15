fn main (){
    // how to store a list of values in rust? You can use arrays
    // arrays are a list of values of THE SAME TYPE and with a FIXED SIZE
    // the size of an array is defined in compile time
    let my_first_array:[i32;5]; // this is an array that will store 5 values of type i32
    // the type of an array is defined by [type;size]
    my_first_array = [1,2,3,4,5]; // this is an array that will store 5 values of type i32
    // like a variable, you can define the value together with the declaration
    let my_second_array = [1,2,3,4,5]; // the type of my_second_array is infered by the compiler ([i32;5])
    // you can access the values of an array by the index
    // the index of an array starts with 0, in this case the index goes from 0 to 4 (5 values)
    println!("my_first_array[0]: {}", my_first_array[0]); // my_first_array[0]: 1
    println!("my_first_array[0]: {}", my_first_array[2]); // my_first_array[0]: 3

    // the compiler can verify if the index is valid in compile time
    // println!("my_first_array[0]: {}", my_first_array[5]); // error: index out of bounds: the len is 5 but the index is 5

    // you can change the value of an array
    let mut my_mut_array = [1,2,3,4,5];
    my_mut_array[0] = 10;
    println!("my_mut_array[0]: {}", my_mut_array[0]); // my_mut_array[0]: 10
    // we can alter all values of an array
    my_mut_array = [11,21,31,41,51];
    println!("my_mut_array[0]: {}", my_mut_array[0]); // my_mut_array[0]: 11

    // is possible print all values of an array using a debug print
    println!("my_mut_array: {:?}", my_mut_array); // my_mut_array: [11, 21, 31, 41, 51]

    // you can define an array with the same value multiple times
    let my_array_with_same_value = [1;1000]; // this is an array with 1000 values of 1 [value;size]

    // you can manipulate arrays with slices
    // slices are a reference of a part of an array
    // you can define a slice using the index of the array
    // Every time that we use & we are creating a reference of a value in memory

    let my_slice:&[i32]; // this is a slice of i32 (only a reference of a part of an array)
    my_slice = &my_first_array[1..3]; // this is a slice of my_mut_array from index 1 to 3 (not include 3)
    // NOTE: when we use [x..y] we are creating a range from x to y (not include y)
    // we can read the values of a slice
    println!("my_slice[0]: {}", my_slice[0]); // my_slice[0]: 2
    println!("my_slice[1]: {}", my_slice[1]); // my_slice[1]: 2
    println!("all values in my_slice: {:?}", my_slice); // all my_slice: [2, 3]

    // like all variables, we can define the value of a slice together with the declaration
    let my_slice2 = &my_mut_array[0..3]; // this is a slice of my_mut_array from index 0 to 3 (not include 3)
    println!("all values in my_slice2: {:?}", my_slice2); // all my_slice2: [11, 21, 31]

    // REMBER: when you use a slice you are creating a reference of a value in memory, you are not copying the value

    // so is possible use slice to change the values of an array, but to do this we need to use a mutable slice
    // the varible from origin need to be mutable too
    let mut my_mut_slice:&mut[i32] = &mut my_mut_array[1..3]; // this is a mutable slice of i32 (only a reference of a part of an array)
    my_mut_slice[0] = 100;
    println!("my_mut_slice[0]: {}", my_mut_slice[0]); // my_mut_slice[0]: 100
    println!("all values my_mut_array: {:?}", my_mut_array); // all my_mut_array: [11, 100, 31, 41, 51]

    // is possible use slice from a slice?
    // yes, you can use slice from a slice
    let my_slice3 = &my_slice[0..2]; // this is a slice of my_slice2 from index 0 to 2 (not include 2)
    println!("all values in my_slice3: {:?}", my_slice3); // all my_slice3: [1,2]

    // how a array is stored in memory?
    // the array is stored in a contiguous memory space, bacause the size is fixed
    // we can print the memory address of an array, and the first element e the last element
    println!("memory address of my_mut_array[0]: {:p}", &my_mut_array[0]); 
    println!("memory address of my_mut_array[4]: {:p}", &my_mut_array[4]); 
    // total memory size of my_mut_array = 4 * 5 bytes (i32) = 20 bytes
    let total_size = &my_mut_array[4] as *const i32 as usize - &my_mut_array[0] as *const i32 as usize + 4;
    // for cal total size we need to add 4 bytes because need include the last element size
    println!("total size of my_mut_array: {} bytes", total_size); // total size of my_mut_array: 20 bytes


}