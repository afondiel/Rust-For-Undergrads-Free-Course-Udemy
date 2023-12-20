
// == Data Types in Rust ==
// - Scalar: a single value
// - Compound: more than one element/value 

/** == Variables - Primitive types ==  
 * // Scalar:
 * - Boolean: bool, either true or false
 * - (Signed Integer): i8, i16, i32, i64, i128, isize (arch)
 * - (Unsigned)Integer: u8, u16, u32. u64, u128, usize (arch)
 * - Float: f32 (single precision), f64 (double precision)
 * - Character: char (utf-8 encoded)
 * - String?: String, &str
 * - The unit type (), whose only possible value is an empty tuple: ()
 *  // Compound:
 * - Arrays like [1, 2, 3]
 * - Tuples like (1, true)
 * Src: https://doc.rust-lang.org/rust-by-example/primitives.html
*/
// function to print the type of a variable
// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
// Usage: 
// let i = 42;
// print_type_of(&s); // &str


fn main() {
    // Arrays
    // A collectikon of multiple values
    // Every element must have the same type
    // Arrays in Rust have a fixed length (i.e: once created arrays can grow or strink)
    // Allocate data on the stack rather than the heap (dynamic location)

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // get the first value, also the position/index of the first element of an array is '0'
    println!("First value = {}", numbers[0]); 

    // println!("12th element = {}", numbers[12]); //error: this operation will panic at runtime
    // note: `#[deny(unconditional_panic)]` on by default, to ignore previous error
}