use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    // add values
    marks.insert("Rust Programming", 96);
    marks.insert("Web Development", 94);
    marks.insert("UX Design", 75);
    marks.insert("Professional Computing Studies", 45);

    // find length of HashPam
    println!("How many subjects have you studied? {}", marks.len());

    // find values in keys
        match marks.get("Web Development") {
    	Some(expr) => println!("You got {} for Web Development!!", expr),
    	None => println!("You didn't study Web Development!!!"),
    };

    // remove a value
    marks.remove("Web Development");

    // loop through hashmap
    for (key, value) in &marks {
    	println!("For {} you got {}%.", key, value);
    };

    // find values in keys
    match marks.get("Web Development") {
    	Some(expr) => println!("You got {} for Web Development!!", expr),
    	None => println!("You didn't study Web Development!!!"),
    };

    // check for key
    println!("Did you study C++? {}", marks.contains_key("C++"));
}
