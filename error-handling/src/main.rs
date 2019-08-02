use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("world.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("world.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!("Cannot create file")
                }
            }
        },
        Err(error) => {
            panic!("There was a problem opening the file {:?}", error)
        }
    };
}

// fn main() {
//     panic!("Crash and burn");
// }

// fn main() {
//     let v = vec![1, 2, 4];

//     v[34];
// }