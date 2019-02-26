fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() > 0 {
        for arg in args {
            println!("{}", arg);
        }
    }
}
