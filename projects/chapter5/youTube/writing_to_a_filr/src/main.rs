use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("new_File.txt").expect("Cannot create file!!!");
    file.write_all(b"test data!").expect("Cannot write to file!"); // creates a byte slice - learn about this

}
  