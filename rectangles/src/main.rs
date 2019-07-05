#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width: 32, height: 44};
    let square = Rectangle::square(50);

    println!("the area of the rectangle is {}", rect1.area());
    
    if rect1.can_hold(&square) {
        println!("Can hold squaer");
    } else {
        println!("Can not hold squaer");
    }
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
