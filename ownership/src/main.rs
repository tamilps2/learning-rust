fn main() {

    // {
    //     let s = String::from("Hello");
    //     println!("The string is {}", s);
    // }

    // println!("{} there", s);

    // taking ownership
    // let s2 = String::from("guess it");

    // taking_ownership(&s2);

    // println!("{} is the guess", s2);

    let mut s1 = String::from("hello");

    // Case 1
    // let s2 = &s1; // borrow

    //s2.push_str(", world"); // Error: borrowed, not allowed to modify.

    // Case 2
    let mut s2 = &mut s1; // mutable borrow

    s2.push_str(" test"); // mutable borrow, so can be modified.

    println!("{}, world", s2);
}

// fn taking_ownership(guess: &String) {
//     println!("This is the guess: {}", guess);
// }
