//The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as output types. For example :
pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
    type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
    fn is_null(&self) -> bool;
  }
  //Using of Address is much more clearer and convenient than AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash.