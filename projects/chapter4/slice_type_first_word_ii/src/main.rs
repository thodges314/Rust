fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);	// needed ref to make &str
    println!("the first word: {}", word);

    let my_string_literal = "hello world";
    let word = first_word(&my_string_literal[..]);	// quite silly, already an &str
    println!("the first word: {}", word);

    let word = first_word(my_string_literal);	// best way for string literal
    println!("the first word: {}", word);

}

fn first_word(s: &str) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			// return i;
			return &s[0..i];
		}
	}
	s 				// can also just return s directly
}
