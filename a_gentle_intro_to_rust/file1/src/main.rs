use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let first = env::args().nth(1).expect("Please supply a file name.");
    let mut file = File::open(&first).expect("I cannot open that file.");
    let mut text = String::new();
    file.read_to_string(&mut text)
        .expect("I can't read the file.");
    println!("The file has {} bytes.", text.len());
    println!("{}", text);
}

// The file here is closed when the function ends and the file variable is dropped.
