use std::env;

fn main() {
    let first = env::args().nth(1).expect("Please supply an arg!");
    let n: i32 = first.parse().expect("Please supply an integer!!");
    println!("The arg is: {}", n);
}

// DON'T USE PANICS FOR RELEASE CODE!!!
