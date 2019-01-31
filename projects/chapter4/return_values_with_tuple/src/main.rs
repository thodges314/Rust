// compare to references_and_borrowing
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) { // usize is pointer size - 
													// not sure why it's the return from .len()
	let length = s.len();

	(s, length)
}
