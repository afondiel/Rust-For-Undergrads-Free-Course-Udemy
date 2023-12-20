// ====== Data Races: concurrency programming ======
// A data race is similar to a race condition and happens when these 3 behaviors occur:
// - Two or more pointers access the same data at same time 
// - At least one of the pointers is being used to write to the data
// - There's no mechanism being used to synchronize access to data 


// Further reading: Rust Doc: https://doc.rust-lang.org/nomicon/races.html


fn main() {
    let _m1 = main1();
    // let _m2 = main2(); // Uncomment the main2 to see the race condition behavior

    println!("{}", _m1);
    // println!("{}", _m2);
}


fn main1() -> String{
    let s = String::from("main1");
    s
}

// Note from The Rustonomicon: 
// A data race has Undefined Behavior, and is therefore impossible to perform in Safe Rust.
// Data races are mostly prevented through rust's ownership system: 
// - it's impossible to alias a mutable reference, 
// - so it's impossible to perform a data race

// fn main2() -> &'static String{ // error[E0106]: missing lifetime specifier
// fn main2() -> &String{ // error[E0106]: missing lifetime specifier
    // let s = String::from("main2");
    // &s /// error[E0515]: cannot return reference to local variable `s`
    // local variable scope limited inside the function, impossible to return the reference
    // returns a reference to data owned by the current function
// }


