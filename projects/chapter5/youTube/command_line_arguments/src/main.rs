use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // for arg in args.iter() {
    // 	println!("{}", arg);
    // }

    println!("{}", args[1]);
}

// first arg is path to executable
