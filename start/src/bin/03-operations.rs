fn main(){
    //numbers operations
    let sum = 1_i64+1;
    let sub = 1-1;
    let mul = 2*2;
    let div = 4/2;
    let rem = 5%2;
    println!("sum:{sum}");
    println!("sub:{sub}");
    println!("mul:{mul}");
    println!("div:{div}");
    println!("rem:{rem}");
    //boll operations
    let equal = 1==1;
    let different = 1!=2;
    let not = !true;
    let and = true&&false;
    let or = true||false;
    println!("equal:{equal}");
    println!("different:{different}");
    println!("not:{not}");
    println!("and:{and}");
    println!("or:{or}");
    //bitwise operations
    let xor_bit = 0b1111^0b0000;
    let or_bit = 0b1111|0b0000;
    let and_bit = 0b1111&0b0000;
    let not_bit = !0b1111;
    let shift_left = 0b0001<<1;
    let shift_right = 0b0001>>1;
    println!("xor_bit:{xor_bit:0b}");
    println!("or_bit:{or_bit:0b}");
    println!("and_bit:{and_bit:0b}");
    println!("not_bit:{not_bit:0b}");
    println!("shift_left:{shift_left:0b}");
    println!("shift_right:{shift_right:0b}");

    //comparison operations
    let greater_than = 1>0;
    let less_than = 1<0;
    let greater_than_or_equal = 1>=0;
    let less_than_or_equal = 1<=0;
    println!("greater_than:{greater_than}");
    println!("less_than:{less_than}");
    println!("greater_than_or_equal:{greater_than_or_equal}");
    println!("less_than_or_equal:{less_than_or_equal}");

    //assignment operations
    let mut a = 100;
    a+=1; // a = a+1
    a-=1; // a = a-1
    a*=2; // a = a*2
    a/=2; // a = a/2
    a%=2; // a = a%2
    a&=2; // a = a&2
    a|=2; // a = a|2
    a^=2; // a = a^2
    a<<=2; // a = a<<2
    a>>=2; // a = a>>2
    println!("a:{a}")


}