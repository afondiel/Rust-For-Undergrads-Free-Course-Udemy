// More references: https://doc.rust-lang.org/std/fs/struct.File.html

use std::fs::File;

fn main() {
    // ******** Uncomment each scenario to see the results *****
    // Scenario #1: error: no file found 
    // let f = File::open("hello.txt").expect("We don't have a file yet!");
    // Scenario #2: works fine, NO error, but the file doesn't exist
    // let f = File::open("hello.txt");

    // Scenario #0: if file not found, error message displayed 
    // let f = File::open("hello.txt");

    // let foo = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("File not found!");
    //     },
    // };

}
