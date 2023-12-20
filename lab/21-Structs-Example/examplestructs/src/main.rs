struct Square {
    side: u32,
    // label: String,
}

fn main() {
    
    let square1 = Square{
        side: 10,
        // label: String::from("Square"),
    };

    let area = area(&square1);
    let area_no_struc = area_no_struc(&square1.side); // test

    println!("Area of Square is {}", area);
    println!("Area of Square no-struc is {}", area_no_struc); // test

}

fn area(square: &Square) -> u32 {
    square.side*square.side
}

// Test: same function without struct
fn area_no_struc(square: &u32) -> u32 {
    square*square
}