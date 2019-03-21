use std::collections::HashMap;
use std::env::args; // function that returns an iterator over the arguments that the program was supplied

fn main() {
    println!("Hello, world!");

    let mut hm = HashMap::new();

    hm.insert(3, "hello");
    hm.insert(5, "world");

    // let r = hm.get(&3); // r returns an *option* enum with possible values Some(v) or None
    // let r = match hm.get(&3) {
    // 	Some(v)=>v,
    // 	_=>"NOTHING",
    // };
    // let r = hm.get(&4).unwrap(); // will return wrapped item or PANIC!!
    let r = hm.get(&4).unwrap_or(&"NOTHING"); // will return wrapped item or NOTHING - we need a pointer because 'get' always returns a pointer

    println!("{}", r);

    // match "berb".parse::<f32>() {	// can append with .unwrap(), .unwrap_or(), .expect() (which gives us error data we type in)
    // 	Ok(v) => println!("OK - {}", v),	// Ok and Err are of the *result* type
    // 	Err(e) => println!("ERROR - {}", e),
    // }

	match get_arg(3) {
		Ok(v) => println!("OK - {}", v),
		Err(e) => println!("ERROR - {}", e),
	}    
}

fn get_arg(n:usize)->Result<String, String>{
	for (i,a) in args().enumerate() {	// args are the things passed in on the command like after --
		if i == n  {
			return Ok(a);
		}
	}
	Err("Not enough args".to_string())
}

// thodges@CC-Admin’s-MacBook-Pro-(4):~/Workspace/Rust/learn-rust-in-7-days/day01/result$ cargo run -- h i p d
//     Finished dev [unoptimized + debuginfo] target(s) in 0.05s
//      Running `target/debug/result h i p d`
// Hello, world!
// NOTHING
// OK - p