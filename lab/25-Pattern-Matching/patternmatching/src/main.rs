// ==== Pattern Matching =====
// - Powerful operator in Rust called `match`
// - Compare one value against a series of patterns
// - Execute code based on which pattern gets matched
// - Patterns can have literals, variables names , wild cards etc

fn main() {
    let variable_name = String::from("Jayesh");

    println!("Char at position 3 is {}",
    match variable_name.chars().nth(3) {
        Some(v) => v.to_string(),
        None => "No character at this positon".to_string(),
    });
}


