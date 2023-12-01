// FILL in the blanks and FIX the errors
use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert("Sunface", 98);
    scores.insert("Daniel", 95);
    scores.insert("Ashley", 69.0);
    scores.insert("Katie", "58");

    // Get returns an Option<&V>
    let score = scores.get("Sunface");
    assert_eq!(score, Some(98));

    if scores.contains_key("Daniel") {
        // Indexing returns a value V
        let score = scores["Daniel"];
        assert_eq!(score, __);
        scores.remove("Daniel");
    }

    assert_eq!(scores.len(), __);

    for (name, score) in scores {
        println!("The score of {} is {}", name, score);
    }
}