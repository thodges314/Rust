fn main() {
    let s1 = gives_ownership();			// gives_ownership moves its return value to s1.

    let s2 = String::from("hello");		// s2 comes into scope

    let s3 = takes_and_gives_back(s2);	// s2 is moved into takes_and_gives_back, which also moves its
    									// return value into s3

    println!("s1:{}", s1);				// should succeed
    // println!("s2:{}", s2);				// should fail
    println!("s3:{}", s3);				// should succeed

}										// here s1 and s3 goes of scope and are dropped.
										// s2 goes out of scope but was moved so nothing happens

fn gives_ownership() -> String {		// gives_ownership will move it's return value into the function
										// that called it

	let some_string = String::from("hello");	// some_string comes into scope

	println!("some_string: {}", some_string);

	some_string							// some_string is returned and moves out of the calling function
}

// takes_and_gives_back will take a string and return one

fn takes_and_gives_back(a_string: String) -> String {	// a_string comes into scope

	println!("a_string: {}", a_string);
	
	a_string							// a_string is returned and moved out to the calling function.
}

// error[E0382]: borrow of moved value: `s2`
//   --> src/main.rs:10:23
//    |
// 6  |     let s3 = takes_and_gives_back(s2);    // s2 is moved into takes_and_gives_back, which also moves its
//    |                                   -- value moved here
// ...
// 10 |     println!("s2:{}", s2);                // should fail
//    |                       ^^ value borrowed here after move
//    |
//    = note: move occurs because `s2` has type `std::string::String`, which does not implement the `Copy` trait

// error: aborting due to previous error
