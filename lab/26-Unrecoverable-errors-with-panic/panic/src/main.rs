// ===== Error Handing ====
// Two major categories:
// - Recoverable errors - result<T,E>
// - Unrecoverable errors - panic!  // Don't Panic ;) 

// The panic! Macro
// - Bad things happen in code -> panic!
// - Program prints a failure message, unwinds, clean stack, and quits
// - Common occurrence: Bugs

// For more reference: 
// - https://doc.rust-lang.org/book/ch09-00-error-handling.html
// - https://doc.rust-lang.org/rust-by-example/error.html

fn main() {
    // panic!("Crash and Quit!");
    let v = vec![1,2,3];
    v[99];     // No compilation or syntax error but runtime error => segmentation error in c? 

    /***
     * thread 'main' panicked at src\main.rs:20:6:
     * index out of bounds: the len is 3 but the index is 99
     * note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
     * error: process didn't exit successfully: `target\debug\panic.exe` (exit code: 101)
     * 
     */

}
