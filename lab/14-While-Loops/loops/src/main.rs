// ============= Loops ===============
// - execute a block of code more than once
// - Repeating code with loop
// Common types of loops:
// - while: conditional loops
// - for: loops through a condition

// ================ Rust Reference ============
// Rust supports 5 loop expressions:
// - `loop` expression denotes an infinite loop.
// - `while` expression loops until a predicate is false.
// - `while let` expression tests a pattern.
// - `for` expression extracts values from an iterator, looping until the iterator is empty.
// - `labelled block` expression runs a loop exactly once, but allows exiting the loop early with break
// Src: https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops

fn main() {
    // infinite loop
    // Use Ctrl+C: to stop it
    loop {
        println!("Hello! World.");
        break; // stop the inf loop after one execution
    }

    let mut a = 3;
    // while loop
    while a !=0 {   
        println!("{}", a );
        a = a - 1;
    }
    
}
