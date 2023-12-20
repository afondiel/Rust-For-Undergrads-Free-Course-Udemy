// Control Flow
// - if expressions
// - else if expressions
// - Loops for repetition

fn main() {
    
    let a = 10;
    
    // if one of the conditions/statements is satisfied, then the program ends
    // if we have more than one conditions that are true, the first one is executed first, and it ends
    if a % 3 == 0 {
        println!("a can be divided by 3 to get zero");
    } 
    else if a % 2 == 0 {
        println!("a can be divided by 2 to get zero");
    }
    else if a % 5 == 0 {
        println!("a can be divided by 5 to get zero");
    }
    else{
        println!("a cannot be divided by 2,3,5 to get zero");
    }

}
