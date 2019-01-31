fn main() {
    let s = String::from("hello");		// s comes into scope

    takes_ownership(s);					// s' value moves into the function and is no longer needed here

    let x = 5;							// x comes into scope

    makes_copy(x);						// x would move into the function, but i32 is Copy, so it's ok to
    									// still use x afterwards

    println!("Our i32: {}", x);			// works fine
    // println!("Our String: {}", s);		// fails

}										// here x goes out of scope, then s, but since s' value was moved,
										// nothing special happens

fn takes_ownership(some_string: String)	{	// some_string comes into scope
	println!("{}", some_string);
}										// here, some_string goes out of scope and 'drop' is called.
										// The backing memory is freed.

fn makes_copy(some_integer: i32) {		// some_integer comes into scope
	println!("{}", some_integer);
}										// here some_integer goes out of scope.  Nothing special happens.


// error[E0382]: borrow of moved value: `s`
//   --> src/main.rs:12:32
//    |
// 4  |     takes_ownership(s);                    // s' value moves into the function and is no longer needed here
//    |                     - value moved here
// ...
// 12 |     println!("Our String: {}", s);        // fails
//    |                                ^ value borrowed here after move
//    |
//    = note: move occurs because `s` has type `std::string::String`, which does not implement the `Copy` trait

// error: aborting due to previous error
