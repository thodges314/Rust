fn main() {
	let s = String::from("Hello, 中国");
	for c in s.chars() {
		println!("{}", c);
	}
    for c in s.bytes() {
    	println!("{}", c);
    }
}
