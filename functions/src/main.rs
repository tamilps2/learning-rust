fn main() {
    println!("Hello, world!");

    for number in (1..6).rev() {
        println!("{}!", number);
    }

    another_function(3);
    println!("The number is {}", five());
}

fn another_function(x: i32) {
    println!("The value of x is {}", x);
}

fn five() -> i32 {
    5
}