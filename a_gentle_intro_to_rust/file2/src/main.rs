use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
    let mut file = match File::open(&filename) {
        Ok(expr) => expr,
        Err(err) => return Err(err),
    };
    let mut text = String::new();
    match file.read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(err) => Err(err),
    }
}

fn main() {
    let first = env::args().nth(1).expect("Please supply a file name.");
    let text = read_file_to_string(&first).expect("Bad file, man!");

    println!("The file has {} bytes.", text.len());
    println!("{}", text);
}
