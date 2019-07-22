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

// enum with value
// fn main () {
//     enum IpAddrStr {
//         V4(String),
//         V6(String)
//     };

//     enum IpAddrDiffTypes {
//         V4(u8, u8, u8, u8),
//         v6(String)
//     };
// }

// enum with match expression
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
    BitCoin
}

fn main() {
    println!("value in cents is {}", value_in_cents(Coin::Penny));
    println!("value in cents is {}", value_in_cents(Coin::Nickel));
    println!("value in cents is {}", value_in_cents(Coin::Dime));
    println!("value in cents is {}", value_in_cents(Coin::Quarter));
    println!("value in cents is {}", value_in_cents(Coin::BitCoin));


    // if let 
    if let Coin::Dime = Coin::Quarter {
        println!("It's a match");
    } else {
        println!("It's not a match");
    }
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        _ => {
            println!("Not a valid coin");
            0
        }
    }
}