#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_it_adds_two() {
        assert_eq!(add_two(2), 4);
        assert_ne!(add_two(3), 4);
    }

    #[test]
    fn test_rectangle_impl() {
        let rect1 = Rectangle {length: 10, width: 20};
        let rect2 = Rectangle {length: 5, width: 5};

        assert!(rect1.can_hold(&rect2), true);
    }
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(x: i32) -> i32 {
    x + 2
}

fn main() {
    println!("Hello, world!");
}
