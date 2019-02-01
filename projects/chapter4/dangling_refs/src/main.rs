fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {			// dangle returns a reference to a String
	let s = String::from("hello");	// s is a new String

	&s 								// here we return a refereence to String s
}									// s goes out of scope and is dropped.  The ref points to nothing.

// error[E0106]: missing lifetime specifier
//  --> src/main.rs:5:16
//   |
// 5 | fn dangle() -> &String {
//   |                ^ help: consider giving it a 'static lifetime: `&'static`
//   |
//   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from

// error: aborting due to previous error
