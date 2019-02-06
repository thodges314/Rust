use std::io;

fn main() {
    let mut input = String::new();

    println!("Hey mate!  Say something!: ");
    match io::stdin().read_line(&mut input) {
    	// Some(expr) => expr,
    	Ok(_) => {
    		println!("Success!  You said: {}", input.to_uppercase());
    	},
    	// None => expr,
    	Err(e) => println!("Oops, something went wrong: {}", e),
    }
}
