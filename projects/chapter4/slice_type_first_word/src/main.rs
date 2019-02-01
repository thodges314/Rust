fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word: {}", word);

    s.clear();

    println!("the first word (again): {}", word);	// will give an error because s is clear -
    												// word hasnothing to reference
}

fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			// return i;
			return &s[0..i];
		}
	}
	&s[..]
}


// error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
//   --> src/main.rs:8:5
//    |
// 4  |     let word = first_word(&s);
//    |                           -- immutable borrow occurs here
// ...
// 8  |     s.clear();
//    |     ^^^^^^^^^ mutable borrow occurs here
// 9  | 
// 10 |     println!("the first word (again): {}", word);
//    |                                            ---- immutable borrow later used here

