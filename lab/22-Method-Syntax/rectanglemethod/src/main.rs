// ======== Methods ========
// - Similar to functions
// - Declared with `fn`
// - Defined with the context of the struct, with keyword `impl`
// - First parameter is always `&self`

// further reading-Characteristics of Object-Oriented Languages: https://doc.rust-lang.org/book/ch17-01-what-is-oo.html


struct Rectangle {
    lenght: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.lenght*self.width
    }
}

fn main() {

    let rectangle1 = Rectangle{
        lenght: 20,
        width: 10 
    };

    println!("Area of Rectangle = {}", rectangle1.area());
}
