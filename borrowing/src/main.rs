/**
 * 
 */
fn main() {
    let mut s1 = String::from("hello");

    // Basic:
    // let len = calculate_length(&s1); // borrow, not move

    // println!("The length of '{}' is {}.", s1, len); // s1 is still valid since it's not moved.

    // change(&mut s1); // sending a mutable reference

    // println!("{}", s1);

    // Case 1
    // let s2 = &s1; // borrow

    //s2.push_str(", world"); // Error: borrowed, not allowed to modify.

    // Case 2
    // let s2 = &mut s1; // mutable borrow

    // s2.push_str(" modified"); // mutable borrow, so can be modified.

    // println!("{}, world", s2);

    // println!("{}, world", s1);

    // Case 3: No multiple mutable borrow
    // let s2 = &mut s1; // mutable borrow
    // let s3 = &mut s1; // error, already has a mutable borrow

    // s2.push_str(" world");
    // s3.push_str(" another");

    // Case 3.1: Above issue can be resolved by scoping the mutation using {}
    // {
    //     let s2 = &mut s1;

    //     s2.push_str(" from");
    // }

    // let s3 = &mut s1;
    // s3.push_str(" rust world");

    // println!("{}", s1);

    // Case 4: No mutable reference when there is an immutable reference and vice versa.
    let s2 = &s1; // immutable borrow
    let s3 = &s1; // immutable borrow
    let s4 = &mut s1; // error, people are reading this data, you can't ask for mutable reference.

    // calculate_length(s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// fn change(s: &mut String) {
//     s.push_str(" world"); // modifying the value with reference, not need for return.
// }