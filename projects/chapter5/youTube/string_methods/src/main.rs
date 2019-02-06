fn main() {
    // replace method
    {
    	let my_string = String::from("Rust is fantastic!!");
    	println!("After replace: {}", my_string.replace("fantastic", "great"));
    }

    // lines method
    {
    	let my_string = String::from("The weather is\nnice\noutside mate.");

    	for line in my_string.lines() {
    		println!("[ {} ]", line);
    	};
    }

    // split
    {
    	let my_string = String::from("Leave+a+like+if+you+enjoyed!");
    	let tokens: Vec<&str> = my_string.split("+").collect(); // .collect() changes iterator into vector

    	println!("{}", my_string);
    	println!("At index 2: {}", tokens[2]);
    }

    // trim
    {
    	let my_string = String::from("  My name is Thomas  \n\r");

    	println!("Before trim: {}", my_string);
    	println!("After trim: {}", my_string.trim());
    }

    // chars
    {
    	let my_string = String::from("CamdenBloke on YouTube");

    	println!("{}", my_string);
    	
    	// get char at index
    	match my_string.chars().nth(4) {
    		Some(expr) => println!("Character at index 4: {}", expr),
    		None => println!("No character at index 4"),
    	};
    }
}
