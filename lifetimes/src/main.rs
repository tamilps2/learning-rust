// case1: error
// when looking at this function from the compilers borrow checker perspective,
// it does not know which of the two borrowed arguments it needs to
// keep the reference for, since the function returns only one str slice, 
// and it can be any of the 2 arguments passed.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// case2: with generic lifetimes
fn longest<'live_long>(x: &'live_long str, y: &'live_long str) -> &'live_long str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str
}

fn main() {
    // Case 1:
    // let string1 = String::from("This is a long string");
    // let string2 = "test";

    // let result = longest(string1.as_str(), string2);
    // println!("longest string is '{}'", result);

    // -----------------------------------------------

    // Case 2: Error case
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     // In here, a reference to a string1 or string2 will be returned to result.
    //     // as a human, we know that it will return string1, 
    //     // but the compiler does not know this. Both the string reference should be valid after return 
    //     // from function as specified by the generic lifetime syntax
    //     result = longest(string1.as_str(), string2.as_str());

    //     // string2 will go out of scope here and will be dropped.
    // }
    // // error: string2 is not longer valid.
    // println!("The longest string is {}", result);

    // -----------------------------------
    // Struct lifetimes

    let mut novel = String::from("Call me Ishmael. Some years ago...");

    let mut first_sentence = novel.split('.')
        .next()
        .expect("Could not find .");

    let i = ImportantExcerpt { part: first_sentence };
}

/// ------- Separate code: Following code won't compile because of lifetime.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    /// 'a lifetime
    let string1 = String::from("test");
    let result;                         
    {
        /// 'b lifetime
        let string2 = String::from("sdf");
        result = longest(string1.as_str(), string2.as_str());
    } /// 'b dropped here
    
    println!("longest string is: {}", result);
    /// 'a dropped here
}
