// Rust is a variable immutable by nature
fn main() {
    
    // creates an immutable type i.e, the value cannot be modified/ reassigned twice
    // let a = 1;
    let mut a = 1; // mutability qualifier
    print!("The value of a is {}",a);
    a = 2; // error: if mutability not handled
    print!("The value of a is {}",a);
}


