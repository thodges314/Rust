use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("info.txt").expect("File won't open!!!");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Cannot read the file!");

    println!("File contents:\n\n{}", contents);

}
