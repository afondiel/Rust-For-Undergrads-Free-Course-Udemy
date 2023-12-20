


fn main() {
    
    let s = String::from("Hello, World!");

    let hello = &s[0..5];
    // let hello = &s[0..6];
    // let hello = &s[0..8];
    // let hello = &s[0..2]; // &:borrow the value

    println!("Sliced String = {}", hello);
    // println!("Length s = {}", s.len());
    // println!("Length Sliced String = {}", hello.len());

}
