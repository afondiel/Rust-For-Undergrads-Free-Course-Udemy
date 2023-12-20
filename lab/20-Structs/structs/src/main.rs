
// ==== Structs ====
// Custom data type
// Make up a meaningful group
// Building blocks of creating new data types

fn main() {
    
    struct User {
        username: String,
        email: String,
        password: String,
    }

    let user1 = User{
        email: String::from("user1.rust@udemy.com"),
        username: String::from("user1.rustUdemy"),
        password: String::from("******"),
    };

    let user2 = User{
        email: String::from("user2.rust@udemy.com"),
        username: String::from("user2.rustUdemy"),
        password: String::from("*******"),
    };

    println!("E-mail = {}, username = {}, password = {}", user1.email, user1.username, user1.password);
    println!("E-mail = {}, username = {}, password = {}", user2.email, user2.username, user2.password);
}
