use std::env::args;

// we want to say hellow to everyone who'se name is in the args list and whose name begins with 'W'
fn main() {
    for a in args() {
    	if a.chars().next().unwrap_or(' ') == 'W' {
    		println!("Hello, {}!", a);
    	}
    }
}

// cargo run -- Alfred Walter Hanry Waldo

// THEIR SOLUTION
// fn main() {
//     for a in args() {
//     	if let Some(c) = a.chars().next() {
//     		match c {
//     			'w' | 'W' => println!("Hello, {}!", a),
//     			_=>{}
//     		}
//     	}
//     }
// }
// 
// Their solution uses more concepts and accounts for either a big or little 'W'.