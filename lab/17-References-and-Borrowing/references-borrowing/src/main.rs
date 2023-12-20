// ======= Recall: Ownership Rules in Rust =======: 
// - Most unique concept to rust
// - It enables Rust to make memory safety guarantees without needing a garbage collector
// 3 rules of Ownership with rust:
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value gets dropped

// Further reading: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html 

fn main() {

    let s = String::from("Jayesh");
    
    // Passing argument by value
    // main1(s);
    // println!("{}", s); 
    // error[E0382]: `s` was used after its contents have been moved into println! function.
    // println! borrows `s` value after used in main1 function
    
    // Solution: pass argument by reference to borrow the value effeciently
    // comment the block below and uncomment the similar block above to recreate the error[E0382]
    main1(&s);
    println!("{}", s);
}

// fn main1(x:String){ // Uncomment this to recreate error[E0382]
fn main1(x:&String){ // comment this to recreate error[E0382]
    println!("{}", x);
}


// entire error log: ../../error-E0382.log)



