use std::env::args;

// we want to say hellow to everyone who'se name is in the args list and whose name begins with 'W'
fn main() {
    for a in args() {
    	if a.chars().next().unwrap_or(' ') == 'W' {
    		println!("Hello, {}", a);
    	}
    }
}

// cargo run -- Alfred Walter Hanry Waldo