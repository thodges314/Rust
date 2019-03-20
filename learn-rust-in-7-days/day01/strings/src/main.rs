fn main() {
	let mut s = String::from("Hello, 中国"); // can mutate String, not &str.  &str points to a static spot in the program's code.
	println!("s.len() = {}", s.len()); // there are 11 chars
	s.push_str("!!!");
	for c in s.chars() {
		println!("{}", c);
	}
    for c in s.bytes() {
    	println!("{}", c);
    }
}
