enum IPVariants {
    IPV4(String),
    IPV6(String),
}

enum Message {
    Write(String),
    Color(i32,i32,i32),
    Move{x:i32,y:i32},
    Quit,
}

impl Message {
    fn call(&self){
        println!("I'm inside the implementation");
    }
}

fn main() {
    
    let _ip1 = IPVariants::IPV4(String::from("192.168.0.1")); // No display fmt
    let _ip2 = IPVariants::IPV6(String::from("::1")); // No display fmt

    let _variable = Message::Write(String::from("Hi!"));
    let _color    = Message::Color(255, 255, 222);

    // _color.call();
    _variable.call();
}
