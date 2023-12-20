fn main() {
    // function call
    let b = sum(1, 2); // safety notes: check the return value

    println!("The sum = {}", b);
}

// add two numbers together and return the sum multiply by ten
fn sum(a:i32, b:i32) -> i32{
    let z = a + b; // safety notes: z size has to be the double of sizeof(a + b), (ex: i64), to avoid overflow
    return z*10;
}
