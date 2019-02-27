use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Result;

fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file = File::open(&filename)?; // returns error or result
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text) // wraps string to return result
}

fn main() {
    let first = env::args().nth(1).expect("Please supply a file name.");
    let text = read_file_to_string(&first).expect("Bad file, man!");

    println!("The file has {} bytes.", text.len());
    println!("{}", text);
}
