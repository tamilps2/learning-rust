fn largest<T>(list: &[T]) -> T {
    let mut large = list[0];

    for &item in list.iter() {
        if item > largest {
            large = item;
        }
    }

    large
}

fn main() {
    let nums = vec![23, 343, 4545, 45];

    println!("largest is {}", largest(&nums));
}
