use std::process::Command;

fn main() {
    let mut cmd = Command::new("python");
    cmd.arg("python_script.py");

    // execute command
    match cmd.output() {
    	Ok(o) => {
    		// o.stdout is a vector of bytes (not a string)
    		unsafe {
    			// also is a not unchecked method - but you have to check results.
    			// https://doc.rust-lang.org/beta/std/str/fn.from_utf8.html
    			println!("Output: {}", String::from_utf8_unchecked(o.stdout));
    		}
    	},
    	Err(e) => {
    		println!("There was an error: {}", e);
    	}
    }
}
