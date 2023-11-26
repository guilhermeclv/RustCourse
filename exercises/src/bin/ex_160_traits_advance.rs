//The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as output types. For example :
use std::fmt;
use core::hash::Hash;

pub trait CacheableItem: Clone + Default + fmt::Debug {
    type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
    fn is_null(&self) -> bool;
}
fn main() {
  println!("Hello, world!");
} 
  //Using of Address is much more clearer and convenient than AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash.