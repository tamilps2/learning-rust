// fn main() {
//     let mut fruits: Vec<String> = Vec::new();

//     fruits.push(String::from("apple"));
//     fruits.push(String::from("orange"));
//     fruits.push(String::from("banana"));

//     for fruit in fruits {
//         println!("Fruits in basket: {}", fruit);
//     }

//     let v = vec![23, 343, 54];

//     println!("First item is: {}", v[0]);
// }

use std::collections::HashMap;

fn main() {
    let sentence = "hello world country world";

    let mut words = HashMap::new();

    for word in sentence.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", words);
}