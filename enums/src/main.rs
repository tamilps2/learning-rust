// enum without value using structs
// fn main() {
//     enum IpAddrKind {
//         V4,
//         V6
//     };    

//     struct IpAddr {
//         kind: IpAddrKind,
//         value: String
//     };

//     let ip = IpAddr {
//         kind: IpAddrKind::V4,
//         value: String::from("127.0.0.1")
//     };

//     println!("ip is {}", ip.value);
// }


fn main () {
    enum IpAddrStr {
        V4(String),
        V6(String)
    };

    enum IpAddrDiffTypes {
        V4(u8, u8, u8, u8),
        v6(String)
    };
}