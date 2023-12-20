// ==== Enums ====
// - Enumerations(enum)
// - Enum => (meaning + Data)
// - Option enum

enum IPVariants {
    IPV4(String),
    IPV6(String),
}


fn main() {
    
    let _ip1 = IPVariants::IPV4(String::from("192.168.0.1"));
    let _ip2 = IPVariants::IPV6(String::from("::1"));
}
