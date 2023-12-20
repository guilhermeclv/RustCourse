// `'a` must live longer than the function.
// Here, `&String::from("foo")` would create a `String`, followed by a
// reference. Then the data is dropped upon exiting the scope, leaving
// a reference to invalid data to be returned.

/* Fix the error in three ways  */
fn invalid_output() -> &'static str{ 
   "foo"
}

fn main() {
}

fn invalid_output_2() -> &'static str { 
    "foo"
}


fn invalid_output_3<'a>(s: &'a String) -> &'a String { 
    s
}

