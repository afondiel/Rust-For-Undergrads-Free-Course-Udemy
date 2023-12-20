// #![allow(overflowing_literals)] // test: to avoid overflowing

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
// print_type_of(&i); // &str

fn main() {

    let mut a:i64 = 2147483647; //a is casted to signed integer 64
    //let mut b:f32 = 4294.2; // test

    a = a + 1; // increment a by 1
    println!("The value a = {}", a);
    
    // Test: Having fun :=)
    // let mut a_inc = a + 1; 
    // let mut a_dec = a - 1;
    // println!("The value a_new = {}", a_inc);
    // println!("The value b_new = {}", a_dec);
    //println!("a: {}, a_inc: {}, a_dec: {}", a, a_inc, a_dec);
    
    //Test: casting  
    // let c = a + b as i64;
    //println!(" b = {}", b);
    //println!("casting = {}", (b as i64));
    //println!("1000 as a u8 is : {}", 1000 as u8);
    // print!("The casting value c = {}", c);
    

}

// compile:  `rustc .\integers.rs`
// run: `.\integers.exe`    