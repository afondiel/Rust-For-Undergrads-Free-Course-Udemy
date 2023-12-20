// functions: block of organized, reusable code
// - Better modularity
// - code reuse

fn main() {
    println!("Hello, world!");
    main1(); // call f1
    main2(42); // call f2 with argument
}

// function with no parameter
fn main1(){
    println!("I'm main1");
}

// function with parameter
fn main2(a:i32){
    println!("What value am I getting = {}", a );
}

