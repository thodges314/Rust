fn main() {
	let mut s = String::from("Hello, 中国"); // can mutate String, not &str.  &str points to a static spot in the program's code.
	println!("s.len() = {}", s.len()); // there are 11 chars but 13 bytes
	s.push_str("!!!");
	for c in s.chars() {
		println!("{}", c);
	}
	for (i, c) in s.chars().enumerate() {
		println!("{} - {}", i, c);
	}
	for (i, c) in s.char_indices() { // indices jump around
		println!("{} - {}", i, c);
	}
    for c in s.bytes() {
    	println!("{}", c);
    }
    println!("Number of l's: {}", count_l(&s)); // dereferences String to &str  - &str is good for read only functions because does not consume data
}

fn count_l(s:&str) -> i32 {
	let mut res = 0;
	for c in s.chars() {
		if c == 'l' {
			res += 1;
		}
	}
	res
}