enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
    other_name
}

/* Fill in the blank */
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}
