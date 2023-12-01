//If the performance of SipHash 1-3 doesn't meet your requirements, you can find replacements in crates.io or github.com.
//The usage of third-party hash looks like this:

use std::hash::BuildHasherDefault;
use std::collections::HashMap;
// Introduce a third party hash function
use twox_hash::XxHash64;

fn main(){
    let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();
    hash.insert(42, "the answer");
    assert_eq!(hash.get(&42), Some(&"the answer"));
}
