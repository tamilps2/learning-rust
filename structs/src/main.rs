// fn main() {
//     struct User {
//         username: String,
//         email: String,
//         is_active: bool,
//         sing_in_count: u64,
//     };

//     let username = String::from("Tamil Selvan");
//     let email = String::from("tamilps2@gmail.com");

//     let user1 = User {
//         username,
//         email,
//         is_active: true,
//         sing_in_count: 34
//     };

//     let user2 = User {
//         username: String::from("tesing"),
//         email: String::from("sdflkj@dfd.com"),
//         ..user1
//     };

//     println!("The user is {}, with email {}, and his active status is {} ({})", user1.username, user1.email, user1.is_active, user1.sing_in_count);
//     println!("The user is {}, with email {}, and his active status is {} ({})", user2.username, user2.email, user2.is_active, user2.sing_in_count);
// }

fn main() {
    struct Color (i32, i32, i32);
    struct Point (i32, i32, i32);

    let _white = Color(1, 1, 1);
    let origin = Point(0, 0, 0);

    println!("Point is x: {},y: {},z: {}", origin.0, origin.1, origin.2);
}