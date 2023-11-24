fn main(){
   // exercise 1
   // what is the type that use less number of bit to store the numbers
   // complete the code below (replace __ to correct answer)
   // tip: think about the max number that each type can store
   let my_number:__ = 1;
   let my_number2:__ = 8446744073709551615; 
   let my_number3:__ = -65535;

   // exercise 2
   // fix de code below, but not alter the values
   let generic_name = -1_u32;
   let generic_name2:u64 = -5;
   let generic_name3:i32 = 1.0;
   let generic_name4:f32 = 1;

   // exercise 3
   // print all variables above using println! in binary, hexadecimal and decimal


   // exercise 4
   // complete the code below (replace __ to correct answer)
   // tip: see exemple in 003-vars-basic-numbers.rs at lines 32 to 35
   
   let my_number_1:u32 = __;
   let my_number_2:i32 = __;
   println!("(00000000000010110100000000000000) this number need to be equal this: ({0:032b})",my_number_1);
   println!("(11111111111111111100001000100001) this number need to be equal this: ({0:032b})",my_number_2);

   // exercise 5
   // complete the code below (replace __ to correct answer)
   // tip: see exemple in 003-vars-basic-numbers.rs at lines 125 to 136

   let my_float:f32 = __;
   println!("(11000110101010110100000000000000) this number need to be equal this: ({0:032b})",my_float.to_bits());



}