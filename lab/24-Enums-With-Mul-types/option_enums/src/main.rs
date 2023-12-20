// ==== Option enums ====
// - Scenarios where a value could be something or nothing
// - Option can handle failures at times, instead of panic!

// Rust by Example Doc: https://doc.rust-lang.org/rust-by-example/std/option.html

enum Option<T>{
    Some(T), // tuple of value with type T
    None,    // failue or lack of value
}

fn main() {
    // See Pattern Matching for more example 
}
