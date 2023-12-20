// example
// {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {}", r); //   |       |
//                           // --+       |
// }   

/* Annotate `r` and `x` as above, and explain why this code fails to compile, in the lifetime aspect. */
// this occored because the lifetime of x is shorter than the lifetime of r
fn main() {  
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
}