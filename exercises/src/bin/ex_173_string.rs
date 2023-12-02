// Modify the code below to print out: 
// Here, there’s no need to allocate more memory inside the loop.
fn main() {
   let mut s = String::with_capacity(20);

   println!("{}", s.capacity());

   for _ in 0..2 {
       s.push_str("hello");
       println!("{}", s.capacity());
   }
   println!("{s}");
   println!("Success!");
}