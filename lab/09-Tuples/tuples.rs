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
    let tup:(i32,i32,i32) = (1,2,3); // explicit type annotation

    let (x,y,z) = tup;

    let a = tup.0; // get/retrieve the value of the first element

    println!("A = {}",a);

    println!("X = {} , Y = {} and Z = {}",x,y,z);
}