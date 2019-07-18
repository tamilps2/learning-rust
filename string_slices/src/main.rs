fn main() {
    let s = String::from("Hello, world!");

    let word = first_word(&s);

    println!("First word is: {}", word);

    let a = [2, 34, 545, 56];
    let b = &a[1..3];
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' || item == b',' {
            return &s[..i];
        }
    }

    &s[..]
}