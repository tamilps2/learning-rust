// fn largest<T>(list: &[T]) -> T {
//     let mut large = list[0];

//     for &item in list.iter() {
//         if item > large {
//             large = item;
//         }
//     }

//     large
// }

// fn main() {
//     let nums = vec![23, 343, 4545, 45];

//     println!("largest is {}", largest(&nums));

//     let floats = vec![1.3, 2.4, 5.3, 8.3];

//     println!("largest float is {}", largest(&floats));
// }


struct Point<T, U> {
    x: T,
    y: U,
}

// by specifying types after impl we specify this
// methods apply only to Point of that type
impl<T, U> Point<T, U> {
    fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point {x: 1, y: 5.2};
    let p2 = Point {x: "hello", y: "word"};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}