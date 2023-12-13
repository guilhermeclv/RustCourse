fn main(){
    //numbers operations
    let sum = 1+1_i64; // the result is i64, because there is only one i64, rust will infer the result type
    //let sum2 = 1_i64+1_i32; // error: is not possible do operations with different types
    let sub = 1-1;
    let mul = 2*2; // the result is i32 because is the default type for integer numbers in rust
    let div = 5/2; // the result is i32 (result is 2 and not 2.5)
    let div2 = 5.0/2.0; // the result is f64 (result is 2.5), because the numbers are f64 (we use .0) the default type for float numbers in rust
    let rem = 5%2; // remainder of division, example: 5/2 = 2 and remainder 1 (2*2+1=5)
    let combined_math_operations = 1+1*13%2*13-5; //is possible combine operations
    let combined_math_operations2 = (1+1)*2; // we can use brackets to change the order of operations
    let using_math_vars = (sub+mul*2)%div+123; // we can use variables in operations, all need to be the same type
    
    println!("1+1_i64 sum:{}",sum);
    println!("1-1 sub:{sub}");
    println!("2*2 mul:{mul}");
    println!("5/2 div:{}",div);
    println!("5.0/2.0 div2:{div2}");
    println!("5%2 rem:{rem}");
    println!("1+1*13%2*13-5 combined_math_operations:{combined_math_operations}");
    println!("(1+1)*2 combined_math_operations2:{combined_math_operations2}");
    println!("(sub+mul*2)%div+123 using_math_vars:{using_math_vars}");

    //comparison operations
    // comparison operations are used to compare values, the result is (always a bool)

    let greater_than = 1>0; // verify if 1 is greater than 0, the result is true
    let less_than = 1<0; // verify if 1 is less than 0, the result is false
    let greater_than_or_equal = 1>=0; // verify if 1 is greater than or equal 0, the result is true
    let less_than_or_equal = 1<=0; // verify if 1 is less than or equal 0, the result is false
    let equal = 1==1; // verify if 1 is equal 1, the result is true
    let different = 1!=2; // verify if 1 is different 2, the result is true
    let combined_comparison_operations = less_than_or_equal!=(mul>=2); // we can combine operations

    println!("1>0 greater_than:{greater_than}");
    println!("1<0 less_than:{less_than}");
    println!("1>=0 greater_than_or_equal:{greater_than_or_equal}");
    println!("1<=0 less_than_or_equal:{less_than_or_equal}");
    println!("1==1 equal:{equal}");
    println!("1!=2 different:{different}");
    println!("less_than_or_equal!=(mul>=2) combined_comparison_operations:{combined_comparison_operations}");


    //bool operations
    let equal:bool = 1==1; // verify if 1 is equal 1, the result is true
    let equal2 = true==false; // verify if true is equal false, the result is false
    let different = 1!=2; // verify if 1 is different 2, the result is true
    let different2 = true!=false; // verify if true is different false, the result is true
    let not:bool  = !true; // invert the value of (true), the result is false
    let and = true&&false; // verify if true and false is true, the result is false, rebember the idea of the word "and"
    let or = true||false; // verify if true or false is true, the result is true, rebember the idea of the word "or"
    let combined_bool_operation = false&&false||true; // the result is true, because the first operation is false and the second is true
    let combined_bool_operation2 = false&&(false||true); // we can use brackets to change the order of operations, the result is false
    let combined_bool_vars = less_than_or_equal!=(mul>=2)&&not&&(and||or)&&true; // we can combine all type of operations

    println!("1==1 equal:{equal}");
    print!("true==false equal2:{equal2}");
    println!("1!=2 different:{different}");
    println!("true!=false different2:{different2}");
    println!("!true not:{not}");
    println!("true&&false and:{and}");
    println!("true||false or:{or}");
    println!("false&&false||true combined_bool_operation:{combined_bool_operation}");
    println!("false&&(false||true) combined_bool_operation2:{combined_bool_operation2}");
    println!("less_than_or_equal!=(mul>=2)&&not&&(and||or)&&true combined_bool_vars:{combined_bool_vars}");

    //bitwise operations

    //these types operations are used to manipulate bits
    //this can be a little confuse, but is simple, we will use the binary representation of numbers
    // the type of operations can be applied to (all types of numbers)

    let xor_bit = 0b1111^0b0000; // the result is 0b1111, because (1 xor 0 is 1), (1 xor 1 is 0), (0 xor 0 is 0), (0 xor 1 is 1)
    let or_bit = 0b1111|0b0010; // the result is 0b1111, because (1 or 0 is 1), (1 or 1 is 1), (0 or 0 is 0), (0 or 1 is 1)
    let and_bit = 0b1111&0b0010; // the result is 0b0010, because (1 and 0 is 0), (1 and 1 is 1), (0 and 0 is 0), (0 and 1 is 0)
    let not_bit = !0b1111; // the result is 0b0000, ! is used to invert the bits, (1 is 0), (0 is 1)
    let shift_left = 0b0101<<1; // the result is 0b0010, << is used to shift the bits to left, (0001 is 0010)
    let shift_right = 0b1001>>1; // the result is 0b0000, >> is used to shift the bits to right, (0001 is 0000)
    let bit_operations = xor_bit|(or_bit&and_bit)<<(shift_left>>shift_right+2<<2); // we can combine operations

    println!("0b1111^0b0000 xor_bit:{xor_bit:016b}");
    println!("0b1111|0b0010 or_bit:{or_bit:016b}");
    println!("0b1111&0b0010 and_bit:{and_bit:016b}");
    println!("!0b1111 not_bit:{not_bit:016b}");
    println!("0b0101<<1 shift_left:{shift_left:016b}");
    println!("0b1001>>1 shift_right:{shift_right:016b}");
    println!("xor_bit|(or_bit&and_bit)<<(shift_left>>shift_right+2<<2) bit_operations:{bit_operations:016b}");

 
    //assignment operations
    let mut a = 100;
    a+=4; // a = a+1
    a-=1; // a = a-1
    a*=2; // a = a*2
    a/=2; // a = a/2
    a%=3; // a = a%2
    a&=1234; // a = a&2
    a|=2; // a = a|2
    a^=1000; // a = a^2
    a<<=2; // a = a<<2
    a>>=2; // a = a>>2
    println!("a:{a}")


}
    