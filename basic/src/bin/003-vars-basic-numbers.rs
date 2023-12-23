fn main(){
    // vars is very important in programming, vars is a way to store data in memory
    // Here we will learn the basic about numeric vars
    // there are many types of numeric vars
    
    // INTEGER VARS

    // integer vars, is a number without decimal part , like 1, 2, 3, 21321331, -121, etc.
    // integer vars can be signed or unsigned

    // It's very important to know the size of the vars, because the size of the vars is the size of memory that the vars will use
    
    // all vars are store in memory in binary format, so the size of the vars is the size of the binary number
    // for example, the number 1 in binary is 1, the number 2 in binary is 10, the number 3 in binary is 11, the number 4 in binary is 100, etc.
    // so the size of the vars is the size of the binary number

    // We are start thinking in a number with 8bits (1byte) and withouth sign (unsigned)
    let generic_name:u8 = 3; // To define a vars we use the keyword (let) and the name of the vars (generic_name) and the type of the vars (u8)
    // u8 is a unsigned integer with 8bits (1byte) of size (u8 = unsigned 8bits) minimum value is (00000000)=0 and maximum value is (11111111)=255
    //we can print to understand better
    println!("generic_name in decimal:{0} in binary {0:08b}",generic_name); // print the value of the vars (generic_name)
    // we used {0:08b} with 8 digit and binary format (b) to print the value of the vars (generic_name) in binary format

    //let generic_name_2:u8 = -4 // REBEMBER this line will generate a error because the vars is unsigned and can't store negative numbers
    // in rust we can store using signed vars with 8,16,32,64,128 bits (1,2,4,8,16 bytes)
    let _my_unsigned_8bits:u8 = 3; // unsigned 8bits (1byte) minimum value is (00000000)=0 and maximum value is (11111111)=255
    let my_unsigned_16bits:u16 = 3; // unsigned 16bits (2bytes) minimum value is (0000000000000000)=0 and maximum value is (1111111111111111)=65535
    let my_unsigned_32bits:u32 = 3; // unsigned 32bits (4bytes) minimum value is 0 and maximum value is 4294967295
    let my_unsigned_64bits:u64 = 3; // unsigned 64bits (8bytes) minimum value is 0 and maximum value is 18446744073709551615
    let my_unsigned_128bits:u128 = 3; // unsigned 128bits (16bytes) minimum value is 0 and maximum value is 340282366920938463463374607431768211455

    // to STORE NEGATIVE NUMBERS we use signed vars
    // rust use a pattern (two's complement) to store negative numbers, so the first bit is used to store the sign of the number
    
    // exemple: 3 in binary is 00000011, -3 in binary is 11111101 (first invert the bits (11111100) and after sum 1 (11111101))
    // |---------------number--------------| (32bits) 
    // |00000000|00000000|00000000|00000011| = 3 (unsigned 32bits)
    //                   ~                     (invert the bits)
    // |11111111|11111111|11111111|11111100| = (~3) 
    //                   +                     (sum 1)
    // |00000000|00000000|00000000|00000001| = 1 
    //                   =
    // |11111111|11111111|11111111|11111101| = -3 (signed 32bits) (two's complement)

    // rebember signed vars use the (double of bits) to store the same number of unsigned vars because the first bit is used to store the sign of the number
    
    let my_signed_8bits:i8 = 3; // signed 8bits (1byte) minimum value is -128 and maximum value is 127
    println!("my_signed_8bits in decimal:{0} in binary {0:08b}",my_signed_8bits); // print the value of the vars (my_signed_8bits)
    let my_signed_8bits_negative:i8 = -3; // signed 8bits (1byte) minimum value is -128 and maximum value is 127
    println!("my_signed_8bits_negative in decimal:{0} in binary {0:08b}",my_signed_8bits_negative); // verify the two's complement
    
    // transform to binary AND transform in a signed integer (you can use ! to invert the bits and after sum 1 to get the two's complement)
    let my_transformed_signed_8bits_negative:i8 = !my_signed_8bits+1; 
    println!("my_transformed_signed_8bits_negative in decimal:{0} in binary {0:08b}",my_transformed_signed_8bits_negative); // verify the two's complement

    let my_signed_16bits:i16 = 3; // signed 16bits (2bytes) minimum value is -32768 and maximum value is 32767
    let my_signed_32bits:i32 = 3; // signed 32bits (4bytes) minimum value is -2147483648 and maximum value is 2147483647
    let my_signed_64bits:i64 = 3; // signed 64bits (8bytes) minimum value is -9223372036854775808 and maximum value is 9223372036854775807
    let my_signed_128bits:i128 = 3; // signed 128bits (16bytes) minimum value is -170141183460469231731687303715884105728 and maximum value is 170141183460469231731687303715884105727

    // we can use the keyword (isize) and (usize) to store the size "default" of the memory of the computer
    // This is a reference to the size of the memory of the computer, but you can use any size of the vars 
    // if the computer is 32bits the isize and usize is a signed and unsigned vars with 32bits
    // if the computer is 64bits the isize and usize is a signed and unsigned vars with 64bits
    let my_isize:isize = 3; 
    let my_usize:usize = 3; 

    // dont worry about (std::mem::size_of_val(&my_isize)*8) is a way to get the size of the vars, we will learn about this in the future
    println!("size of my_isize is {} bits",std::mem::size_of_val(&my_isize)*8); // print the size of the vars (my_isize)
    println!("size of my_usize is {} bits",std::mem::size_of_val(&my_usize)*8); // print the size of the vars (my_usize)

    //when you put a value to add in a vars, this value has a type
    // if you put a value without a type, rust will try to infer the type of the vars
    // DEFAULT TYPE OF A NUMBER IS i32 (signed 32bits)
    let my_generic_number = 3; // rust infer the type of the vars (my_generic_number) is i32 (signed 32bits) because the default
    let my_generic_number2 = 3_u64; // You can specify the type of a value using the suffix (u8,u16,u32,u64,u128,i8,i16,i32,i64,i128,isize,usize)
    let my_generic_number3 = 3_i128; // You can specify the type of a vars using the syntax (let my_generic_number3:u64 = 3)
    
    // print the size of the vars (my_generic_number)
    println!("size of my_generic_number is {} bits",std::mem::size_of_val(&my_generic_number)*8); // print the size of the vars (my_generic_number)
    println!("size of my_generic_number2 is {} bits",std::mem::size_of_val(&my_generic_number2)*8); // print the size of the vars (my_generic_number2)
    println!("size of my_generic_number3 is {} bits",std::mem::size_of_val(&my_generic_number3)*8); // print the size of the vars (my_generic_number3)

    // is possible enter values hexadecimal, octal and binary for integer vars in rust

    let my_int_var_1:i8 = 0b111_1011; // number binarry using 0b are considered unsigned
    let my_int_var_2:u8 = 0b0000_0101;
    let my_int_var_3:i8 = 0o123; // number octal using 0o are considered unsigned
    let my_int_var_4:i8 = -0x12; // for use hexadecimal we use 0x
    let my_int_var_5:i8 = 0x12;

    println!("my_int_var in decimal:{0} in binary {0:08b}",my_int_var_1); 
    println!("my_int_var in decimal:{0} in binary {0:08b}",my_int_var_2); 
    println!("my_int_var in decimal:{0} in octal {0:o}",my_int_var_3); 
    println!("my_int_var in decimal:{0} in hexadecimal -0x12",my_int_var_4); 
    println!("my_int_var in decimal:{0} in hexadecimal {0:X}",my_int_var_5); // {0:X} work only print positive values
    
    
    // OBS:
    // hexadecimal is a way to represent a number using 16 symbols (0,1,2,3,4,5,6,7,8,9,A,B,C,D,E,F)
    // hexadecimal is common and very useful to represent binary numbers, because each symbol in hexadecimal represent 4 bits in binary
    // exemple:
    // 0xABC123 = 0b1010_1011_1100_0001_0010_0011 = 11256099
    // octal is a way to represent a number using 8 symbols (0,1,2,3,4,5,6,7)

    // DECIMAL VARS

    // for store a number with decimal part we use float vars 
    // float vars can be negative or positive (because the way to representation the number IEEE-754)
    // FLOAR VARS are a aproximation of the number, so IS NOT possible store a number with decimal part with infinite precision
    // for example, the number 1/3 is 0.3333333333... (infinite 3), so is not possible store this number with all decimal part
    // this concept is very important, because float vars is not a exact number, is a aproximation of the number (WE NEED REBEMBER THIS)
    // Rust use (IEEE-754) to store float vars, so the size of the vars is the size of the IEEE-754
    // in rust we can store decimal numbers using float vars with 32,64 bits (4,8 bytes)
    
    let my_float_32bits:f32 = 3.0; // float 32bits (4bytes) minimum value is 1.17549435e-38 and maximum value is 3.40282347e+38
    let my_float_64bits:f64 = 3.0; // float 64bits (8bytes) minimum value is 2.2250738585072014e-308 and maximum value is 1.7976931348623157e+308
    let my_float_default = 3.0; // rust infer the type of the vars (my_float_default) is f64 (float 64bits) because the default
    let my_float_infered_32 = 3.0_f32; // You can specify the type of a value using the suffix (f32,f64)
    let my_float_infered_64 = 3.0_f64; 

    println!("size of my_float_32bits is {} bits",std::mem::size_of_val(&my_float_32bits)*8); // print the size of the vars (my_float_32bits)
    println!("size of my_float_64bits is {} bits",std::mem::size_of_val(&my_float_64bits)*8); // print the size of the vars (my_float_64bits)
    println!("size of my_float_default is {} bits",std::mem::size_of_val(&my_float_default)*8); // print the size of the vars (my_float_default)
    println!("size of my_float_infered_32 is {} bits",std::mem::size_of_val(&my_float_infered_32)*8); // print the size of the vars (my_float_infered_32)
    println!("size of my_float_infered_64 is {} bits",std::mem::size_of_val(&my_float_infered_64)*8); // print the size of the vars (my_float_infered_64)
    
    // Considering a float var with 64 bits - IEEE-754 (8 bytes) we have 1 bit for the sign, 11 bits for the exponent and 52 bits for the mantissa
    // The Sign Bit is used to store the sign of the number (0 for positive and 1 for negative)
    // The Exponent is used to store the number of decimal places that the number will be moved to the left or right (we will understand better in the exemple)
    // The Mantissa is used to store the number with decimal part (we will understand better in the exemple)
    
    // |Sign Bit|--Exponent---|-----------------------Mantissa-----------------------|
    //     x    | xxxxxxxxxxx | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx |
    //    1 bit |   11 bits   |                         52 bits                      | = (64bits) for float 64bits - IEEE-754
    //    1 bit |    8 bits   |                         23 bits                      | = (32bits) for float 32bits - IEEE-754

    // to convert a number binary to decimal we use the formula (number = mantissa * 2^exponent)

    // example: 0011111111000101010010000000000000000000000000000000000000000000 (64 bits = f64)
    // |Sign Bit|--Exponent---|-----------------------Mantissa-----------------------|
    //     0    | 01111111100 | 0101010010000000000000000000000000000000000000000000 |
    
    // the sign is 0, so the number is positive
    // the exponent is 01111111100, so the number is 1020 (2^2+2^3+2^4+2^5+2^6+2^7+2^8+2^9), but is nessesary subtract a bias (1023 for 64bits) or (127 for 32bits) to get the exponent
    // the bias exist to balance the negative and positive exponent and become possible store number less than 1
    // so the real exponent is 1020 - 1023 = -3
    // The mantissa is a decimal binary number, in this case is 1.010101001 but the 1 is implicit (not stored), some times it can look strange, but is a way to save memory
    // than mantissa in binary is 1.010101001 = in decimal (2^0 + 2^-2 + 2^-4 + 2^-6 + 2^-9) = 1.330078125
    // so the number is 1.330078125 * 2^-3 = 0.166259765625

    let my_float_exemple:f64 = 0.166259765625;
    println!("(0.1650390625) in 64 bits - binary ({0:064b})",my_float_exemple.to_bits()); // is necessary use to_bits() to get the binary in float vars


    // to convert a number decimal to binary we use the formula (mantissa = number / 2^exponent)
    // exemple: 85.125
    // we will separate the number in integer part and decimal part
    // integer part = 85 
    // decimal part = 0.125

    // to transform a integer part in binary we use the division by 2 and get the rest of the division
    // NUMBER | DIVISION BY 2 | REST OF DIVISION
    // 85     | 42            | 1
    // 42     | 21            | 0
    // 21     | 10            | 1
    // 10     | 5             | 0
    // 5      | 2             | 1
    // 2      | 1             | 0
    // 1      | 0             | 1

    // so 85 is (1010101) in binary = (2^6 + 2^4 + 2^2 + 2^0) = 85

    // to transform a decimal part in binary we use the multiplication by 2 and get the integer part of the multiplication
    // NUMBER | MULTIPLICATION BY 2 | INTEGER PART
    // 0.125  | 0.25                | 0
    // 0.25   | 0.5                 | 0
    // 0.5    | 1                   | 1

    // so 0.125 is (0.001) in binary = (2^-3) = 0.125

    // so 85.125 is (1010101.001) in binary = (2^6 + 2^4 + 2^2 + 2^0 + 2^-3) = 85.125
    // now to discovery the exponent we need move the number to the left or right until the number is between 1 and 2

    // in this case 85.125 is (1010101.001) in binary = (1.010101001 * 2^6) = 85.125 (the exponent is 6)
    // for mantisa de 1 is implicit, so the mantissa is (010101001) in binary
    // so for a var with 64bits the mantiisa have 52bits, this case: (0101010010000000000000000000000000000000000000000000) we puts 0 to complete the 52bits
    // and for a var with 32bits the mantiisa have 23bits, this case: (01010100100000000000000) we puts 0 in the end to complete the 23bits

    // REBEMBER there is a BIAS to balance the negative and positive exponent and become possible store number less than 1
    // for 64bits the bias is 1023, so the real exponent is 6 + 1023 = 1029
    // for 32bits the bias is 127, so the real exponent is 6 + 127 = 133
    // so the exponent to store the number 85.125 in 64bits is (1029) in binary (100 0000 0101) 
    // and in 32bits is (133) in binary (1000 0101)

    // The number is positive, so the sign is 0
    // so 85.125 in binary using IEEE-754 is:
    //|Sign Bit|--Exponent---|-----------------------Mantissa-----------------------|
    //    0    | 10000000101 | 0101010010000000000000000000000000000000000000000000 | = (64bits) for float 64bits - IEEE-754
    //    0    | 10000101    | 01010100100000000000000                              | = (32bits) for float 32bits - IEEE-754


    let myfloat_exemple_64:f64 = 85.125;
    let myfloat_exemple_32:f32 = 85.125;
    println!("(85.125) in 32 bits - binary ({0:032b})",myfloat_exemple_32.to_bits()); // is necessary use to_bits() to get the binary in float vars
    println!("(85.125) in 64 bits - binary ({0:064b})",myfloat_exemple_64.to_bits()); 

}