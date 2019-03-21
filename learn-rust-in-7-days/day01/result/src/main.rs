use std::collections::HashMap;

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

    match "berb".parse::<f32>() {	// can append with .unwrap(), .unwrap_or(), .expect() (which gives us error data we type in)
    	Ok(v) => println!("OK - {}", v),	// Ok and Err are of the *result* type
    	Err(e) => println!("ERROR - {}", e),
    }
}
