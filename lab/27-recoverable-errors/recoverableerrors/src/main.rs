use std::fs::File;

// ===== Recoverable Errors =====
// - situation where we can report the error to the user
// Example: File not found
// - Result enum to rescue
// enum Result<T, E>{
//     Ok(T),
//     Err(E),
// }


fn main() {
    
    let f = File::open("hello.txt");
}
