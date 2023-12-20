// ======= Ownership in Rust =======: 
// - Most unique concept to rust
// - It enables Rust to make memory safety guarantees without needing a garbage collector
// 3 rules of Ownership with rust:
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value gets dropped

// Further reading: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html 


fn main() {
    // Rule #1: the value 10 is owner 'a'
    // Rule #2: the 10 value is owned by a, and only a can use it in the entire program
    let a = 10;
    
    // Rule #3: a is btw the scope { }
    // Uncomment the block {} to test rule #3, and comment: // let a = 10; above
    // {
    //    let a = 10;
    // }
    println!("The value: {}", a); // Error: a not accessible out of its scope, value dropped
    // End Rule #3
}
