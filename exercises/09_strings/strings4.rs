// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

// not recommended: take &str and convert to String inside
fn string_slice(arg: &str) {
    println!("{arg}");
}

// recommended: take String directly
fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned()); // force to allocate a String

    string("nice weather".into()); // force to allocate a String

    string(format!("Interpolation {}", "Station")); // force to allocate a String

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim()); // trim returns &str, which does not change the value, but only borrows it

    string("Happy Monday!".replace("Mon", "Tues")); // force to allocate a String

    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // force to allocate a String
}
